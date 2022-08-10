use std::env;

use rhai::{serde::from_dynamic, Dynamic, Engine};
use serde::de::DeserializeOwned;

/// This function will return a collection of an entire type of game object
/// (eg. rooms, items, spells). If you need to get a specific object, or a
/// specific set of of a type, use `get_game_object` or `get_game_object_set`.
///
/// Valid patterns look as such:
///     ./scripts/rooms/grassy_hill.rhai
///     ./scripts/items/broadsword.rhai
pub fn get_game_objects<T>(engine: &Engine, module_type: &str) -> Result<Vec<T>, ScriptError>
where
    T: 'static + Sync + Send + DeserializeOwned + std::fmt::Debug,
{
    let mut objects: Vec<T> = Vec::new();

    let root = env::current_dir()?;
    let pattern = format!("**/{}/**/*.rhai", module_type);

    let walker = globwalk::GlobWalkerBuilder::from_patterns(root, &[pattern])
        .max_depth(5)
        .build()?
        .into_iter()
        .filter_map(Result::ok);

    for item in walker {
        tracing::debug!("Trying to load script: {:?}", item.path());
        match engine.eval_file::<Dynamic>(item.path().into()) {
            Ok(result) => {
                tracing::debug!("Successfully loaded {:?}", item.path().file_name());
                objects.push(from_dynamic::<T>(&result)?);
            }
            Err(err) => {
                tracing::error!("Failed to load {}: {}", item.path().display(), err);
            }
        }
    }

    Ok(objects)
}

#[derive(Debug)]
pub enum ScriptErrorType {
    BadPattern,
    InvalidPath,
    InvalidSyntax,
}

#[derive(Debug)]
pub struct ScriptError {
    pub kind: ScriptErrorType,
    pub message: String,
}

impl std::error::Error for ScriptError {}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<globwalk::GlobError> for ScriptError {
    fn from(err: globwalk::GlobError) -> Self {
        Self {
            kind: ScriptErrorType::BadPattern,
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for ScriptError {
    fn from(err: std::io::Error) -> Self {
        Self {
            kind: ScriptErrorType::InvalidPath,
            message: err.to_string(),
        }
    }
}

impl From<Box<rhai::EvalAltResult>> for ScriptError {
    fn from(err: Box<rhai::EvalAltResult>) -> Self {
        Self {
            kind: ScriptErrorType::InvalidSyntax,
            message: err.to_string(),
        }
    }
}

/// This creates a scripting engine which can consume .rhai files. It's
/// important to note that it's okay to create many of these, which is
/// why it is not part of the shared state.
pub fn create_engine() -> Engine {
    let mut engine = rhai::Engine::new();
    engine.disable_symbol("eval");
    engine.set_max_string_size(4096);
    engine.set_max_array_size(1024);
    engine.set_max_operations(1024);

    engine
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::{entity::EntityId, room::Room, vec3::Vec3};

//     #[test]
//     fn get_game_objects_rooms() -> Result<(), BlossomScriptError> {
//         let engine = create_engine();
//         let rooms = get_game_objects::<Room>(&engine, "data")?;

//         assert!(vec![Room {
//             entity_id: EntityId::empty(),
//             name: "Test Room".to_string(),
//             description: "This is a test room.".to_string(),
//             position: Vec3::new(0, 0, 0),
//             exits: Vec::new(),
//         }]
//         .iter()
//         .any(|item| rooms.contains(item)));

//         Ok(())
//     }
// }
