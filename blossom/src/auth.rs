use std::net::IpAddr;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use iridescent::{constants::RED, Styled};
use nectar::{event::TelnetEvent, option::TelnetOption};
use sqlx::postgres::PgPool;

use crate::{
    account::Account,
    blossom_log,
    connection::Connection,
    entity::EntityId,
    error::{Error, ErrorType, Result},
    logging::{Action, Kind, Loggable},
    player::{PartialPlayer, Player},
    role::Role,
    theme,
    utils::{capitalize, is_http},
    vec3::Vec3,
};

/// Creates a new account and character.
async fn create(name: &str, password: &str, pg: &PgPool) -> Result<PartialPlayer> {
    let name = name.trim();
    let password = password.trim();

    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();
    let hash = argon.hash_password(password.as_bytes(), &salt)?.to_string();

    let is_first = is_first_account(pg).await?;
    let roles = if is_first { vec![Role::Admin] } else { vec![] };

    // If the player supplied an email, we will send them a confirmation email,
    // but this exists on a separate table until activated; thus we always apply
    // default values to a new account.
    let account_record = sqlx::query!(
        "insert into accounts (password_hash, roles)
        values ($1, $2)
        returning id",
        hash,
        &Role::as_str_list(&roles),
    )
    .fetch_one(pg)
    .await?;

    let player_record = sqlx::query!(
        "insert into players (account_id, name)
        values ($1, $2)
        returning id",
        account_record.id,
        name
    )
    .fetch_one(pg)
    .await?;

    Ok(PartialPlayer::new(
        player_record.id,
        Account::new(account_record.id, roles),
    ))
}

/// Attempts to log a player in by validating their password.
async fn login(name: &str, password: &str, addr: IpAddr, pg: &PgPool) -> Result<Player> {
    let name = name.trim();
    let password = password.trim();

    // We check if a name is associated with an account when the player is
    // prompted to enter their username at the start, thus we can guarantee that
    // a single record will exist if this function is called.
    let record = sqlx::query!(
        r#"select p.id, p.name, p.position, p.health, p.max_health, p.mana, p.max_mana, p.xp, p.level, p.afk, p.brief, a.id as "account_id", a.password_hash, a.email as "email?", a.roles
        from players p 
        join accounts a on p.account_id = a.id 
        where p.name = $1"#,
        name,
    )
    .fetch_one(pg)
    .await?;

    let argon = Argon2::default();
    let hash = PasswordHash::new(&record.password_hash)?;

    if argon.verify_password(password.as_bytes(), &hash).is_ok() {
        Ok(Player {
            _entityid: EntityId::empty(),
            _addr: addr,
            id: record.id,
            account: Account {
                id: record.account_id,
                email: record.email,
                roles: Role::list(&record.roles),
            },
            name: record.name.to_string(),
            position: Vec3::from(record.position.as_slice()),
            health: record.health,
            max_health: record.max_health,
            mana: record.mana,
            max_mana: record.max_mana,
            xp: record.xp,
            xp_to_level: record.level * 1000,
            level: record.level,
            brief: record.brief,
            afk: record.afk,
            dirty: false,
            seen: true,
        })
    } else {
        Err(Error::new(ErrorType::Authentication, "Invalid credentials"))
    }
}

/// Returns whether a player exists or not (via name).
async fn name_exists(name: &str, pg: &PgPool) -> Result<bool> {
    let name = name.trim();

    let record = sqlx::query!(
        r#"select exists (select 1 from players where name = $1)"#,
        name
    )
    .fetch_one(pg)
    .await;

    match record {
        Ok(record) => Ok(matches!(record.exists, Some(true))),
        Err(_) => Ok(false),
    }
}

/// Returns whether the account being created is the first account on the
/// server. If so, we will grant them the `admin` role.
async fn is_first_account(pg: &PgPool) -> Result<bool> {
    let record = sqlx::query!(r#"select exists (select 1 from players)"#)
        .fetch_one(pg)
        .await
        .map_err(|_| Error {
            kind: ErrorType::Database,
            message: "Failed to check if first account.".to_string(),
        })?;

    match record.exists {
        Some(true) => Ok(false),
        _ => Ok(true),
    }
}

/// This starts an authentication process for a player. It will handle input,
/// finding if a player name exists, password authentication, new account
/// creation flow, and login. It will always return a player if it succeeds.
///
/// This function will return additionally returns a special flag, `restart`,
/// which will restart the authentication process if the player. We need to do
/// this because some responses should drop the connection instead.
pub async fn authenticate(conn: &mut Connection, pg: PgPool) -> Result<Option<Player>> {
    let name = match get_name(conn).await {
        Ok(name) => name,
        Err(e) => return Err(e),
    };

    let exists = name_exists(&name, &pg).await?;

    if exists {
        let password = get_password(conn).await?;
        let partial_player = login(&name, &password, conn.ip(), &pg).await;

        if let Ok(player) = partial_player {
            blossom_log!(Kind::Join, conn);

            Ok(Some(player))
        } else {
            conn.send_message("Invalid credentials.").await?;
            blossom_log!(Kind::FailedJoin, conn);

            Ok(None)
        }
    } else {
        let password = set_password(conn).await?;
        let partial_player = create(&name, &password, &pg).await?;
        let colored_name = &name.foreground(theme::BLUE).bold();

        // Send a welcome message to the player.
        conn.send_message(&format!(
            "{} {}{}",
            "\nWelcome,".foreground(theme::YELLOW).bold(),
            colored_name,
            "!".foreground(theme::YELLOW).bold()
        ))
        .await?;

        // Send some helpful information to the player.
        conn.send_message(&format!(
            "{} {} {} {} {}",
            "You can view all the commands by typing"
                .foreground(theme::YELLOW)
                .bold(),
            "help".foreground(theme::BLUE).bold(),
            "or".foreground(theme::YELLOW).bold(),
            "?".foreground(theme::BLUE).bold(),
            "at any time.".foreground(theme::YELLOW).bold()
        ))
        .await?;

        // If this is the first account on the server, we will let the player
        // know that they have been granted the `admin` role.
        if partial_player.account.roles.contains(&Role::Admin) {
            conn.send_message(&format!(
                "\n{} {} {}",
                "You are the first character on the server. You have been granted the ADMIN role. Type"
                    .foreground(theme::RED)
                    .bold(),
                "@help".foreground(theme::BLUE).bold(),
                "for a list of admin commands.".foreground(theme::RED).bold()
            ))
            .await?;
        }

        blossom_log!(Kind::Join, conn);

        Ok(Some(Player {
            _entityid: EntityId::empty(),
            _addr: conn.ip(),
            id: partial_player.id,
            account: partial_player.account,
            name,
            position: Vec3::new(0, 0, 0),
            health: 100,
            max_health: 100,
            mana: 100,
            max_mana: 100,
            xp: 0,
            xp_to_level: 100,
            level: 1,
            brief: false,
            afk: false,
            dirty: false,
            seen: false,
        }))
    }
}

/// Prompt the player for their name and returns the input.
async fn get_name(conn: &mut Connection) -> Result<String> {
    let name = loop {
        conn.send_message("What is your name? If you are new, enter the name you wish to use.")
            .await?;

        if let Some(msg) = conn.try_next().await {
            // Because this is the first frame we receive from the client, we
            // have to check if it contains HTTP traffic, and if so, drop it
            // silently.
            if is_http(&msg) {
                return Err(Error {
                    kind: ErrorType::Internal,
                    message: "HTTP traffic is not allowed.".to_string(),
                });
            }

            let msg = msg.trim();
            if !validate_username(msg) {
                conn.send_message(&format!(
                    "{}",
                    "Name should be between 3 and 16 alphabetical characters.".foreground(RED)
                ))
                .await?;

                continue;
            }

            break capitalize(msg);
        }
    };

    Ok(name)
}

/// Prompts an existing player for their password and returns the input.
async fn get_password(conn: &mut Connection) -> Result<String> {
    let mut failure_count = 0;

    // ECHO off
    conn.send_iac(TelnetEvent::Will(TelnetOption::Echo)).await?;

    let password = loop {
        // Drop the connection if they enter an incorrect password 3 times.
        if failure_count > 3 {
            return Err(Error {
                kind: ErrorType::Authentication,
                message: "Too many incorrect password attempts.".to_string(),
            });
        }

        conn.send_message("What is your password?").await?;

        if let Some(msg) = conn.try_next().await {
            let msg = msg.trim();

            if msg.is_empty() {
                conn.send_message(&format!(
                    "{}",
                    "Invalid credentials.".foreground(theme::RED)
                ))
                .await?;

                failure_count += 1;
                continue;
            }

            break msg.to_string();
        }
    };

    // ECHO on
    conn.send_iac(TelnetEvent::Wont(TelnetOption::Echo)).await?;

    Ok(password)
}

/// Prompts a new player for their password and returns the input. This will
/// also ask if the player wishes to create a new character with the name they
/// provided.
async fn set_password(conn: &mut Connection) -> Result<String> {
    loop {
        // Set the players password -- we will turn off echo for this.
        conn.send_message("Character not found. Create a new character with this name? [Y/n]")
            .await?;

        if let Some(msg) = conn.try_next().await {
            match msg.to_lowercase().as_str() {
                "y" | "yes" | "" => break,
                "n" | "no" => {
                    return Err(Error {
                        kind: ErrorType::Authentication,
                        message: "Character not found.".to_string(),
                    })
                }
                _ => continue,
            }
        }
    }

    // ECHO off
    conn.send_iac(TelnetEvent::Will(TelnetOption::Echo)).await?;

    let password = loop {
        conn.send_message("What will your password be? [`q` to quit]")
            .await?;

        if let Some(msg) = conn.try_next().await {
            let msg = msg.trim();

            if msg == "exit" {
                return Err(Error {
                    kind: ErrorType::Authentication,
                    message: "Character creation cancelled.".to_string(),
                });
            }

            if msg.is_empty() || msg.len() < 8 {
                conn.send_message(&format!(
                    "{}",
                    "Password should be at least 8 characters.".foreground(theme::RED)
                ))
                .await?;

                continue;
            }

            break msg.to_string();
        }
    };

    // ECHO on
    conn.send_iac(TelnetEvent::Wont(TelnetOption::Echo)).await?;

    Ok(password)
}

/// Returns a bool indicating whether or not the username is valid.
fn validate_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 16 && username.chars().all(|c| c.is_ascii_alphabetic())
}
