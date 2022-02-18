/// An Input is a sequence of strings that will be parsed, then handled, by the broker and the
/// game loop. Inputs are constructed by the conncetion loop after receiving a valid packet
/// from a peer, then sent to the broker via a ClientEvent::Command.
///
/// Inputs are also accessible when implementing the `GameCommand` trait, so each command has
/// full access to the all the characters sent in a message.
#[derive(Debug, Default)]
pub struct Input {
    // The first word in a message; represents a command name. This is used for matching against
    // the command name and its aliases.
    pub command: String,
    // The rest of the message, split into a vector of strings.
    pub args: Vec<String>,
}

impl From<String> for Input {
    fn from(message: String) -> Self {
        let tokens = message.split_whitespace().collect::<Vec<_>>();
        Input {
            command: tokens[0].to_string(),
            args: tokens[1..].iter().map(|t| t.to_string()).collect(),
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input {} {}", self.command, self.args.join(" "))
    }
}
