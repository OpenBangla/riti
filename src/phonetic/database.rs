use ahash::RandomState;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

use crate::config::Config;
use crate::phonetic::regex::parse;

pub(crate) struct Database {
    regex: String,
    map: HashMap<&'static str, Vec<&'static str>, RandomState>,
    table: HashMap<String, Vec<String>, RandomState>,
    suffix: HashMap<String, String, RandomState>,
    autocorrect: HashMap<String, String, RandomState>,
    // The user's auto-correct entries.
    user_autocorrect: HashMap<String, String, RandomState>,
}

impl Database {
    pub(crate) fn new_with_config(config: &Config) -> Database {
        // Load the user's auto-correct entries.
        let user_autocorrect =
            if let Ok(file) = read_to_string(config.get_user_phonetic_autocorrect()) {
                serde_json::from_str(&file).unwrap()
            } else {
                HashMap::with_hasher(RandomState::new())
            };

        let map = vec![
            ("a", vec!["a", "aa", "e", "oi", "o", "nya", "y"]),
            ("b", vec!["b", "bh"]),
            ("c", vec!["c", "ch", "k"]),
            ("d", vec!["d", "dh", "dd", "ddh"]),
            ("e", vec!["i", "ii", "e", "y"]),
            ("f", vec!["ph"]),
            ("g", vec!["g", "gh", "j"]),
            ("h", vec!["h"]),
            ("i", vec!["i", "ii", "y"]),
            ("j", vec!["j", "jh", "z"]),
            ("k", vec!["k", "kh"]),
            ("l", vec!["l"]),
            ("m", vec!["h", "m"]),
            ("n", vec!["n", "nya", "nga", "nn"]),
            ("o", vec!["a", "u", "uu", "oi", "o", "ou", "y"]),
            ("p", vec!["p", "ph"]),
            ("q", vec!["k"]),
            ("r", vec!["rri", "h", "r", "rr", "rrh"]),
            ("s", vec!["s", "sh", "ss"]),
            ("t", vec!["t", "th", "tt", "tth", "khandatta"]),
            ("u", vec!["u", "uu", "y"]),
            ("v", vec!["bh"]),
            ("w", vec!["o"]),
            ("x", vec!["e", "k"]),
            ("y", vec!["i", "y"]),
            ("z", vec!["h", "j", "jh", "z"]),
        ]
        .into_iter()
        .collect();

        Database {
            regex: String::with_capacity(1024),
            map,
            table: serde_json::from_str(&read_to_string(config.get_database_path()).unwrap())
                .unwrap(),
            suffix: serde_json::from_str(&read_to_string(config.get_suffix_data_path()).unwrap())
                .unwrap(),
            autocorrect: serde_json::from_str(
                &read_to_string(config.get_autocorrect_data()).unwrap(),
            )
            .unwrap(),
            user_autocorrect,
        }
    }

    /// Find words from the dictionary with given word.
    pub(crate) fn search_dictionary(&mut self, word: &str) -> Vec<String> {
        // Build the Regex string.
        parse(word, &mut self.regex);
        let rgx = Regex::new(&self.regex).unwrap();

        self.map
            .get(word.get(0..1).unwrap_or_default())
            .unwrap_or(&Vec::new())
            .iter()
            .flat_map(|&item| self.table[item].iter().filter(|i| rgx.is_match(i)).cloned())
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
    pub(crate) fn update(&mut self, config: &Config) {
        self.user_autocorrect =
            if let Ok(file) = read_to_string(config.get_user_phonetic_autocorrect()) {
                serde_json::from_str(&file).unwrap()
            } else {
                HashMap::with_hasher(RandomState::new())
            };
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use crate::config::get_phonetic_method_defaults;

    #[test]
    fn test_database() {
        let config = get_phonetic_method_defaults();
        let mut db = Database::new_with_config(&config);

        assert_eq!(
            db.search_dictionary("a"),
            ["অ্যা", "অ্যাঁ", "আ", "আঃ", "া", "এ",]
        );
        assert_eq!(db.search_dictionary("("), Vec::<String>::new());
    }

    #[test]
    fn test_suffix() {
        let config = get_phonetic_method_defaults();
        let db = Database::new_with_config(&config);

        assert_eq!(db.find_suffix("gulo"), Some("গুলো"));
        assert_eq!(db.find_suffix("er"), Some("ের"));
        assert_eq!(db.find_suffix("h"), None);
    }

    #[test]
    fn test_autocorrect() {
        let config = get_phonetic_method_defaults();
        let db = Database::new_with_config(&config);

        assert_eq!(db.search_corrected("academy"), Some("oZakaDemi"));
        assert_eq!(db.search_corrected("\\nai\\"), None);
    }
}

#[cfg(feature = "bench")]
mod benches {
    extern crate test;

    use super::Database;
    use crate::config::get_phonetic_method_defaults;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_phonetic_database_a(b: &mut Bencher) {
        let config = get_phonetic_method_defaults();
        let mut db = Database::new_with_config(&config);
        b.iter(|| {
            let res = db.search_dictionary("a");
            black_box(res);
        })
    }

    #[bench]
    fn bench_phonetic_database_aro(b: &mut Bencher) {
        let config = get_phonetic_method_defaults();
        let mut db = Database::new_with_config(&config);
        b.iter(|| {
            let res = db.search_dictionary("arO");
            black_box(res);
        })
    }

    #[bench]
    fn bench_phonetic_database_bistari(b: &mut Bencher) {
        let config = get_phonetic_method_defaults();
        let mut db = Database::new_with_config(&config);
        b.iter(|| {
            let res = db.search_dictionary("bistari");
            black_box(res);
        })
    }
}
