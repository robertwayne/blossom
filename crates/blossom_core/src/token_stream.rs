/// A TokenStream is a sequence of strings that will be parsed, then handled, by the broker and the
/// game loop. TokenStreams are constructed by the conncetion loop after receiving a valid packet
/// from a peer, then sent to the broker via a ClientEvent::Command.
///
/// TokenStreams are also accessible when implementing the `GameCommand` trait, so each command has
/// full access to the all the characters sent in a message.
#[derive(Debug, Default)]
pub struct TokenStream {
    // The first word in a message; represents a command name. This is used for matching against
    // the command name and its aliases.
    pub command: String,
    // The rest of the message, split into a vector of strings.
    pub remaining: Vec<String>,
}

impl From<String> for TokenStream {
    fn from(message: String) -> Self {
        let tokens = message.split_whitespace().collect::<Vec<_>>();
        TokenStream {
            command: tokens[0].to_string(),
            remaining: tokens[1..].iter().map(|t| t.to_string()).collect(),
        }
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenStream {} {}",
            self.command,
            self.remaining.join(" ")
        )
    }
}
