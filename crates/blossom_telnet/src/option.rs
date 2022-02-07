use crate::constants::{ECHO, GA, SGA};

/// Represents all Telnet options supported by Blossom.
#[derive(Debug)]
pub enum TelnetOption {
    Echo,
    GoAhead,
    SupressGoAhead,
    Unknown(u8),
}

impl From<u8> for TelnetOption {
    fn from(byte: u8) -> Self {
        match byte {
            ECHO => TelnetOption::Echo,
            GA => TelnetOption::GoAhead,
            SGA => TelnetOption::SupressGoAhead,
            _ => TelnetOption::Unknown(byte),
        }
    }
}

impl Into<u8> for TelnetOption {
    fn into(self) -> u8 {
        match self {
            TelnetOption::Echo => ECHO,
            TelnetOption::GoAhead => GA,
            TelnetOption::SupressGoAhead => SGA,
            TelnetOption::Unknown(byte) => byte,
        }
    }
}
