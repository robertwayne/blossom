use crate::player::PlayerId;

/// Represents the result of a command entered by the player. A `ClientOnly`
/// response will be displayed to that connection only. A Broadcast takes a
/// channel and a message to broadcast to that channel. A Private takes a
/// player name and a message to send to that player.
#[derive(Clone, Debug)]
pub enum Response {
    // An empty response that DOES NOT signal the connection loop to close the
    // connection.
    Empty,
    // An empty response that DOES signal the connection loop to close the
    // connection.
    Close,
    // A response that is sent to a single client.
    Client(String),
    // A response that is sent to a group of clients; represented as an array of
    // player IDs.
    Channel(Vec<PlayerId>, String),
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Empty => write!(f, "Empty"),
            Response::Close => write!(f, "Close"),
            Response::Client(s) => write!(f, "Client {s}"),
            Response::Channel(players, s) => write!(
                f,
                "Channel {} {}",
                players.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(", "),
                s
            ),
        }
    }
}

impl Response {
    /// Utility function for creating a `Response::Client` variant with less
    /// boilerplate.
    pub fn client_message(s: impl Into<String>) -> Self {
        Self::Client(s.into())
    }

    /// Utility function for creating a `Response::Channel` variant with less
    /// boilerplate.
    pub fn channel_message(players: Vec<PlayerId>, s: impl Into<String>) -> Self {
        Self::Channel(players, s.into())
    }
}
