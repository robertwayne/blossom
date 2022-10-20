/// Represents a permissions-based role that an account can hold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    Moderator,
    Builder,
    Player,
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "admin" => Role::Admin,
            "moderator" => Role::Moderator,
            "builder" => Role::Builder,
            "player" => Role::Player,
            _ => panic!("Invalid role: {}", role),
        }
    }
}

impl From<Role> for String {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => "admin".to_string(),
            Role::Moderator => "moderator".to_string(),
            Role::Builder => "builder".to_string(),
            Role::Player => "player".to_string(),
        }
    }
}

impl Role {
    /// Creates a new Vec<Role> from a Vec<String> of roles. Useful when
    /// deserializing roles from the database, which are stored as an array of
    /// varchar.
    pub fn list(roles: &[String]) -> Vec<Role> {
        roles.iter().map(|r| Role::from(r.clone())).collect()
    }

    /// Creates a new Vec<String> from a Vec<Role>.
    pub fn as_str_list(roles: &[Role]) -> Vec<String> {
        roles.iter().map(|r| String::from(r.clone())).collect()
    }
}
