use std::fmt::Display;

use iridescent::Styled;

use crate::{player::Player, theme};

/// Represents the chat prompt which appears as the final line of every client
/// message. The prompt is configurable, so we represent it with a struct that
/// implements `StyledString`.
pub struct Prompt {
    health: Option<i32>,
    max_health: Option<i32>,
    mana: Option<i32>,
    max_mana: Option<i32>,
}

impl From<&Player> for Prompt {
    fn from(player: &Player) -> Self {
        Self {
            health: Some(player.health),
            max_health: Some(player.max_health),
            mana: Some(player.mana),
            max_mana: Some(player.max_mana),
        }
    }
}

impl Display for Prompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prompt = String::new();

        if let (Some(health), Some(max_health)) = (self.health, self.max_health) {
            let hp = format!("{}/{}hp", health, max_health).foreground(theme::RED);
            prompt.push_str(&format!("{}", hp));
        }

        if let (Some(mana), Some(max_mana)) = (self.mana, self.max_mana) {
            let mp = format!(" {}/{}mp", mana, max_mana).foreground(theme::BLUE);
            prompt.push_str(&format!("{}", mp));
        }

        writeln!(f, "\n[{}]", &prompt.dim())
    }
}
