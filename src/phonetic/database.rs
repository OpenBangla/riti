use hashbrown::HashMap;
use rayon::prelude::*;
use regex::Regex;
use std::fs::read_to_string;

use crate::hashmap;
use crate::phonetic::regex::parse;
use crate::settings::{get_settings_database_dir, get_settings_user_phonetic_autocorrect};

pub(crate) struct Database {
    map: HashMap<&'static str, Vec<&'static str>>,
    table: HashMap<String, Vec<String>>,
    suffix: HashMap<String, String>,
    autocorrect: HashMap<String, String>,
    // The user's auto-correct entries.
    user_autocorrect: HashMap<String, String>,
}

impl Database {
    pub(crate) fn new() -> Database {
        // Load the user's auto-correct entries.
        let user_autocorrect =
            if let Ok(file) = read_to_string(get_settings_user_phonetic_autocorrect()) {
                serde_json::from_str(&file).unwrap()
            } else {
                HashMap::new()
            };

        let map = hashmap! [
            "a" => vec!["a", "aa", "e", "oi", "o", "nya", "y"],
            "b" => vec!["b", "bh"],
            "c" => vec!["c", "ch", "k"],
            "d" => vec!["d", "dh", "dd", "ddh"],
            "e" => vec!["i", "ii", "e", "y"],
            "f" => vec!["ph"],
            "g" => vec!["g", "gh", "j"],
            "h" => vec!["h"],
            "i" => vec!["i", "ii", "y"],
            "j" => vec!["j", "jh", "z"],
            "k" => vec!["k", "kh"],
            "l" => vec!["l"],
            "m" => vec!["h", "m"],
            "n" => vec!["n", "nya", "nga", "nn"],
            "o" => vec!["a", "u", "uu", "oi", "o", "ou", "y"],
            "p" => vec!["p", "ph"],
            "q" => vec!["k"],
            "r" => vec!["rri", "h", "r", "rr", "rrh"],
            "s" => vec!["s", "sh", "ss"],
            "t" => vec!["t", "th", "tt", "tth", "khandatta"],
            "u" => vec!["u", "uu", "y"],
            "v" => vec!["bh"],
            "w" => vec!["o"],
            "x" => vec!["e", "k"],
            "y" => vec!["i", "y"],
            "z" => vec!["h", "j", "jh", "z"]
        ];

        Database {
            map,
            table: serde_json::from_str(
                &read_to_string(get_settings_database_dir().join("dictionary.json")).unwrap(),
            )
            .unwrap(),
            suffix: serde_json::from_str(
                &read_to_string(get_settings_database_dir().join("suffix.json")).unwrap(),
            )
            .unwrap(),
            autocorrect: serde_json::from_str(
                &read_to_string(get_settings_database_dir().join("autocorrect.json")).unwrap(),
            )
            .unwrap(),
            user_autocorrect,
        }
    }

    /// Find words from the dictionary with given word.
    pub(crate) fn search_dictionary(&self, word: &str) -> Vec<String> {
        let rgx = Regex::new(&parse(word)).unwrap();

        self.map
            .get(word.get(0..1).unwrap_or_default())
            .unwrap_or(&Vec::new())
            .par_iter()
            .flat_map(|&item| {
                self.table[item]
                    .par_iter()
                    .filter(|i| rgx.is_match(i))
                    .cloned()
            })
            .collect()
    }

    pub(crate) fn find_suffix(&self, string: &str) -> Option<&str> {
        self.suffix.get(string).map(String::as_str)
    }

    /// Search for a `term` in AutoCorrect dictionary.
    ///
    /// This looks in the user defined AutoCorrect entries first.
    pub(crate) fn search_corrected(&self, term: &str) -> Option<&str> {
        self.user_autocorrect
            .get(term)
            .or_else(|| self.autocorrect.get(term))
            .map(String::as_str)
    }

    /// Update the user defined AutoCorrect dictionary.
    pub(crate) fn update(&mut self) {
        self.user_autocorrect =
            if let Ok(file) = read_to_string(get_settings_user_phonetic_autocorrect()) {
                serde_json::from_str(&file).unwrap()
            } else {
                HashMap::new()
            };
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use crate::settings::tests::set_default_phonetic;

    #[test]
    fn test_database() {
        set_default_phonetic();

        let db = Database::new();

        assert_eq!(
            db.search_dictionary("a"),
            ["অ্যা", "অ্যাঁ", "আ", "আঃ", "া", "এ",]
        );
        assert_eq!(db.search_dictionary("("), Vec::<String>::new());
    }

    #[test]
    fn test_suffix() {
        set_default_phonetic();

        let db = Database::new();

        assert_eq!(db.find_suffix("gulo"), Some("গুলো"));
        assert_eq!(db.find_suffix("er"), Some("ের"));
        assert_eq!(db.find_suffix("h"), None);
    }

    #[test]
    fn test_autocorrect() {
        set_default_phonetic();

        let db = Database::new();

        assert_eq!(
            db.search_corrected("academy"),
            Some("oZakaDemi")
        );
        assert_eq!(db.search_corrected("\\nai\\"), None);
    }
}
