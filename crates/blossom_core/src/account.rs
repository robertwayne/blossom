use crate::role::Role;

/// Represents the account that a player belongs to. Accounts hold generic
/// player data, and allow for the ability to have multiple characters in the
/// future. In general, this struct should only be interacted with by the
/// internal auth functions.
#[derive(Clone, Debug, Default)]
pub struct Account {
    pub id: i32,
    pub email: Option<String>,
    pub roles: Vec<Role>,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Account {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            email: None,
            roles: Vec::new(),
        }
    }
}
