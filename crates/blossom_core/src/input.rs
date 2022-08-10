use simsearch::{SearchOptions, SimSearch};

use crate::searchable::Searchable;

/// An Input is a sequence of strings that will be parsed, then handled, by the
/// broker and the game loop. Inputs are constructed by the conncetion loop
/// after receiving a valid packet from a peer, then sent to the broker via a
/// `ClientEvent::Command`.
///
/// Inputs are also accessible when implementing the `GameCommand` trait, so
/// each command has full access to the all the characters sent in a message.
#[derive(Debug, Default)]
pub struct Input {
    // The first word in a message; represents a command name. This is used for
    // matching against the command name and its aliases.
    pub command: String,
    // The rest of the message, split into a vector of strings.
    pub args: Vec<String>,
}

impl Input {
    /// Performs a fuzzy search and comparison of the command input (first
    /// argument) against the supplied values. The values are any entity that
    /// implements the `Searchable` trait.
    ///
    /// This will return the index of the most relevant match within the
    /// original collection.
    ///
    /// This uses the `SimSearch` levenshtein implementation, which has a
    /// limitation of ASCII-only strings. UTF-8 string compatability can be
    /// enabled by removing this option. The benefit is that the search is
    /// SIMD-powered and MUCH faster.
    ///
    /// @TODO: Make this configurable via the game config file. @TODO: Return
    /// the reference to T instead of the index in the collection.
    pub fn fuzzy_match<T>(&self, values: &[&T]) -> Option<usize>
    where
        for<'a> &'a T: Searchable,
    {
        let options = SearchOptions::new().levenshtein(true).threshold(0.5);
        let mut engine = SimSearch::new_with(options);

        values.iter().enumerate().for_each(|(i, s)| {
            engine.insert(i, s.search_key());
        });

        match self.args.get(0) {
            Some(arg) => {
                let results = engine.search(arg);

                if results.is_empty() {
                    return None;
                }

                // return index of the most relevant match
                Some(results[0])
            }
            None => None,
        }
    }
}

impl From<String> for Input {
    fn from(message: String) -> Self {
        let tokens = message.split_whitespace().collect::<Vec<_>>();
        Input {
            command: tokens[0].to_string(),
            args: tokens[1..]
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input {} {}", self.command, self.args.join(" "))
    }
}
