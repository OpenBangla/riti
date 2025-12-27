use std::collections::HashMap;

use ahash::RandomState;
use emojicon::{BengaliEmoji, Emojicon};

/// Data which is shared between the methods.
pub(crate) struct Data {
    suffix: HashMap<String, String, RandomState>,
    autocorrect: HashMap<String, String, RandomState>,
    emojicon: Emojicon,
    bengali_emoji: BengaliEmoji,
}

impl Data {
    pub(crate) fn new() -> Data {
        Data {
            suffix: serde_json::from_slice(include_bytes!("../data/suffix.json")).unwrap(),
            autocorrect: serde_json::from_slice(include_bytes!("../data/autocorrect.json")).unwrap(),
            emojicon: Emojicon::new(),
            bengali_emoji: BengaliEmoji::new(),
        }
    }

    pub(crate) fn find_suffix(&self, string: &str) -> Option<&str> {
        self.suffix.get(string).map(String::as_str)
    }

    /// Search for a `term` in the AutoCorrect dictionary.
    pub(crate) fn search_corrected(&self, term: &str) -> Option<&str> {
        self.autocorrect.get(term).map(String::as_str)
    }

    pub(crate) fn get_emoji_by_emoticon(&self, emoticon: &str) -> Option<&str> {
        self.emojicon.get_by_emoticon(emoticon)
    }

    pub(crate) fn get_emoji_by_name(&self, name: &str) -> Option<impl Iterator<Item = &str>> {
        self.emojicon.get_by_name(name)
    }

    pub(crate) fn get_emoji_by_bengali(&self, name: &str) -> Option<impl Iterator<Item = &str>> {
        self.bengali_emoji.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn test_suffix() {
        let db = Data::new();

        assert_eq!(db.find_suffix("gulo"), Some("গুলো"));
        assert_eq!(db.find_suffix("er"), Some("ের"));
        assert_eq!(db.find_suffix("h"), None);
    }

    #[test]
    fn test_autocorrect() {
        let db = Data::new();

        assert_eq!(db.search_corrected("academy"), Some("oZakaDemi"));
        assert_eq!(db.search_corrected("\\nai\\"), None);
    }
}
