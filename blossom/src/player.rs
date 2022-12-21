use std::net::IpAddr;

use sqlx::PgPool;

use crate::{
    account::Account,
    entity::{Entity, EntityId},
    logging::Loggable,
    quickmap::QuickMapKey,
    vec3::Vec3,
};

pub type PlayerId = i32;

/// Represents a partially initialized state for a player. This exists solely
/// for the authentication loop to prepare a struct from the database between
/// stages (username, password) before reading all of the players data and
/// filling out a `Player` struct. There is never a reason to use this outside
/// of that specific area -- use `Player` instead.
#[derive(Debug)]
pub struct PartialPlayer {
    pub id: PlayerId,
    pub account: Account,
}

impl PartialPlayer {
    pub fn new(id: i32, account: Account) -> Self {
        Self { id, account }
    }
}

/// The full representation of a player after authenticating with the server.
/// Note that you should never need to create a `Player` struct yourself, as all
/// of that is handled inside the auth flow. In general, you should just be
/// accessing and/or modifying a player from the world state.
#[derive(Clone, Debug)]
pub struct Player {
    pub _entityid: EntityId,
    pub _addr: IpAddr,
    pub id: PlayerId,
    pub account: Account,
    pub name: String,
    pub position: Vec3,
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub xp: i32,
    pub xp_to_level: i32,
    pub level: i32,
    pub brief: bool,
    pub afk: bool,
    pub dirty: bool,
    pub seen: bool,
}

impl Player {
    pub fn new(id: PlayerId, addr: IpAddr) -> Player {
        Player {
            _entityid: EntityId::default(),
            _addr: addr,
            id,
            account: Account::default(),
            name: String::new(),
            position: Vec3::new(0, 0, 0),
            health: 0,
            max_health: 0,
            mana: 0,
            max_mana: 0,
            xp: 0,
            xp_to_level: 0,
            level: 0,
            brief: false,
            afk: false,
            dirty: false,
            seen: false,
        }
    }

    pub async fn save(&mut self, pg: PgPool) {
        if sqlx::query!(
            "update players
            set position = $1,
                health = $2,
                max_health = $3,
                mana = $4,
                max_mana = $5,
                xp = $6,
                level = $7,
                brief = $8,
                afk = $9
            where id = $10",
            &self.position.as_vec(),
            self.health,
            self.max_health,
            self.mana,
            self.max_mana,
            self.xp,
            self.level,
            self.brief,
            self.afk,
            self.id
        )
        .execute(&pg)
        .await
        .is_ok()
        {
            self.dirty = false;
        } else {
            tracing::error!("Failed to save player {} to the database.", self.id);
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl QuickMapKey<PlayerId> for Player {
    fn key(&self) -> PlayerId {
        self.id
    }
}

impl Entity for Player {
    fn id(&self) -> EntityId {
        self._entityid
    }
}
