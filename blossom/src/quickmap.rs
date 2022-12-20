use std::collections::HashMap;

use crate::entity::{Entity, EntityId};

/// Used as a `HashMap` key within `QuickMap`'s. This should be a non-changing
/// value.
pub trait QuickMapKey<T> {
    fn key(&self) -> T;
}

/// This is a 'data structure' that represents a U as two separate structures,
/// one for fast iteration, and the other for fast lookups via a key of type T,
/// which is any type that implements the `QuickMapKey` trait.
///
/// The first data structure is a simple Vec<U>, which is used for iterating
/// over U. Because filter maps are a common operation, and systems prefer the
/// ability to iterate over groups / all U at once, this offers ideal
/// performance. While using a `HashMap` would truly be MORE than performant
/// enough, it is still a fair bit slower when iterating over groups of data.
///
/// The second data structure is a HashMap<T, usize>. This is for performing
/// fast lookups It is as fast +/- a few ns as a straight `HashMap` when
/// performing lookups. Because it is simply a map of (T, usize), where T is
/// generally an i32 or Vec3, the additional size is negligible. The real cost
/// is the extra complexity, but as mentioned above, this is all abstracted out
/// and handled in the background, so no one will ever need to know how they are
/// retrieving or iterating over data, just that it will always be fast.
///
/// In addition, it is possible to change the structures in the background
/// without having to worry about breaking existing code.
#[derive(Debug, Default)]
pub struct QuickMap<T, U> {
    array: Vec<U>,
    map: HashMap<T, usize>,
}

impl<T, U> QuickMap<T, U>
where
    T: PartialEq + Eq + std::hash::Hash + Copy,
    U: QuickMapKey<T> + Entity,
{
    pub fn new() -> Self {
        Self {
            array: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: U) -> T {
        let key = value.key();
        let index = self.array.len();
        self.array.push(value);
        self.map.insert(key, index);

        key
    }

    pub fn remove_by_id(&mut self, id: EntityId) {
        let entity = self.array.iter().find(|e| e.id() == id);
        if let Some(entity) = entity {
            let key = entity.key();

            if let Some(index) = self.map.get(&key) {
                self.array.remove(*index);
                self.map.remove(&key);
            }
        }
    }

    pub fn remove(&mut self, key: &T) {
        if let Some(index) = self.map.remove(key) {
            self.array.remove(index);
        }
    }

    pub fn get(&self, key: &T) -> Option<&U> {
        self.map.get(key).map(|index| &self.array[*index])
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut U> {
        self.map.get(key).map(|index| &mut self.array[*index])
    }

    pub fn get_by_id(&self, id: EntityId) -> Option<&U> {
        self.array.iter().find(|e| e.id() == id)
    }

    pub fn get_by_id_mut(&mut self, id: EntityId) -> Option<&mut U> {
        self.array.iter_mut().find(|e| e.id() == id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &U> {
        self.array.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut U> {
        self.array.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn clear(&mut self) {
        self.array.clear();
        self.map.clear();
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.map.keys()
    }
}
