use std::{collections::HashMap, fs::read, path::PathBuf};

use ahash::RandomState;
use emojicon::{BengaliEmoji, Emojicon};

use crate::config::Config;

/// Data which is shared between the methods.
pub(crate) struct Data {
    table: HashMap<String, Vec<String>, RandomState>,
    suffix: HashMap<String, String, RandomState>,
    autocorrect: HashMap<String, String, RandomState>,
    emojicon: Emojicon,
    bengali_emoji: BengaliEmoji,
}

impl Data {
    pub(crate) fn new(config: &Config) -> Data {
        // If the database directory is not set, initialize with empty values.
        if *config.get_database_dir() == PathBuf::default() {
            Data {
                table: HashMap::default(),
                suffix: HashMap::default(),
                autocorrect: HashMap::default(),
                emojicon: Emojicon::new(),
                bengali_emoji: BengaliEmoji::new(),
            }
        } else {
            Data {
                table: serde_json::from_slice(&read(config.get_database_path()).unwrap()).unwrap(),
                suffix: serde_json::from_slice(&read(config.get_suffix_data_path()).unwrap())
                    .unwrap(),
                autocorrect: serde_json::from_slice(&read(config.get_autocorrect_data()).unwrap())
                    .unwrap(),
                emojicon: Emojicon::new(),
                bengali_emoji: BengaliEmoji::new(),
            }
        }
    }

    pub(crate) fn get_words_for(&self, table: &str) -> impl Iterator<Item = &String> {
        // TODO: use `unwrap_or_default` when it's supported by the MSRV.
        self.table
            .get(table)
            .map(|i| i.iter())
            .unwrap_or_else(|| [].iter())
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
    use crate::config::get_phonetic_method_defaults;

    #[test]
    fn test_suffix() {
        let config = get_phonetic_method_defaults();
        let db = Data::new(&config);

        assert_eq!(db.find_suffix("gulo"), Some("গুলো"));
        assert_eq!(db.find_suffix("er"), Some("ের"));
        assert_eq!(db.find_suffix("h"), None);
    }

    #[test]
    fn test_autocorrect() {
        let config = get_phonetic_method_defaults();
        let db = Data::new(&config);

        assert_eq!(db.search_corrected("academy"), Some("oZakaDemi"));
        assert_eq!(db.search_corrected("\\nai\\"), None);
    }
}
