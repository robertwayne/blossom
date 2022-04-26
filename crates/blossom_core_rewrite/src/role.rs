/// Represents a permissions-based role that an account can hold.
#[derive(Clone, Debug, PartialEq)]
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

impl Role {
    /// Creates a new Vec<Role> from a Vec<String> of roles. Useful when deserializing roles from
    /// the database, which are stored as an array of varchar.
    pub fn list(roles: Vec<String>) -> Vec<Role> {
        roles.iter().map(|r| Role::from(r.clone())).collect()
    }
}
