/// Trait for game objects that can be searched via fuzzy-string matching by the
/// input parser. Entities implementing this trait should prefer to return their name,
/// as that is the most likely  way a player would interact with them.
pub trait Searchable {
    fn search_key(&self) -> &str;
}
