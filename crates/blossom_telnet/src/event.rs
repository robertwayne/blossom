use crate::{constants::*, option::TelnetOption, subnegotiation::SubnegotiationType};

/// Represents all Telnet events supported by Blossom.
#[derive(Debug)]
pub enum TelnetEvent {
    Character(u8),
    Message(String),
    Do(TelnetOption),
    Will(TelnetOption),
    Dont(TelnetOption),
    Wont(TelnetOption),
    Subnegotiation(SubnegotiationType),
    GoAhead,
    Nop,
}

impl Into<u8> for TelnetEvent {
    fn into(self) -> u8 {
        match self {
            TelnetEvent::Message(_) => 0x00,
            TelnetEvent::Do(_) => DO,
            TelnetEvent::Will(_) => WILL,
            TelnetEvent::Dont(_) => DONT,
            TelnetEvent::Wont(_) => WONT,
            TelnetEvent::Subnegotiation(_) => SB,
            TelnetEvent::Character(byte) => byte,
            TelnetEvent::GoAhead => GA,
            TelnetEvent::Nop => NOP,
        }
    }
}
