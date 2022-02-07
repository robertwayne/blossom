use std::env;

use rhai::{serde::from_dynamic, Dynamic, Engine};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlossomScriptError {
    #[error("invalid glob pattern: {0}")]
    BadPattern(#[from] globwalk::GlobError),
    #[error("invalid path: {0}")]
    InvalidPath(#[from] std::io::Error),
    #[error("invalid script: {0}")]
    InvalidSyntax(#[from] Box<rhai::EvalAltResult>),
}

/// This function will return a collection of an entire type of game object
/// (eg. rooms, items, spells). If you need to get a specific object, or a
/// specific set of of a type, use get_game_object or get_game_object_set.
///
/// Valid patterns look as such:
///     ./scripts/rooms/grassy_hill.rhai
///     ./scripts/items/broadsword.rhai
pub fn get_game_objects<T>(engine: &Engine, module_type: &str) -> Result<Vec<T>, BlossomScriptError>
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
