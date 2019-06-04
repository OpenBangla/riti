use hashbrown::HashMap;
use rayon::prelude::*;
use regex::Regex;
use rustc_hash::FxHashMap;

use crate::hashmap;
use crate::phonetic::regex::PhoneticRegex;

lazy_static! {
    static ref DICTIONARY_TABLE: HashMap<&'static str, Vec<&'static str>> = hashmap! [
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
}

pub(crate) struct Database {
    regex: PhoneticRegex,
    table: HashMap<String, Vec<String>>,
    suffix: FxHashMap<String, String>,
    autocorrect: FxHashMap<String, String>,
}

impl Database {
    pub(crate) fn new() -> Database {
        Database {
            regex: PhoneticRegex::new(),
            table: serde_json::from_str(include_str!("dictionary.json")).unwrap(),
            suffix: serde_json::from_str(include_str!("suffix.json")).unwrap(),
            autocorrect: serde_json::from_str(include_str!("autocorrect.json")).unwrap(),
        }
    }

    /// Find words from the dictionary with given word.
    pub(crate) fn search_dictionary(&self, word: &str) -> Vec<String> {
        let rgx = Regex::new(&self.regex.parse(word)).unwrap();

        DICTIONARY_TABLE
            .get(&word[0..1])
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

    pub(crate) fn find_suffix(&self, string: &str) -> Option<String> {
        self.suffix.get(string).cloned()
    }

    /// Get the phonetically corrected string from auto-correct dictionary.
    pub(crate) fn get_corrected(&self, string: &str) -> Option<String> {
        self.autocorrect.get(string).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::Database;

    #[test]
    fn test_database() {
        let db = Database::new();

        assert_eq!(
            db.search_dictionary("a"),
            [
                "অ্যা",
                "অ্যাঁ",
                "আ",
                "আঃ",
                "া",
                "এ",
            ]
        );
        assert_eq!(db.search_dictionary("("), Vec::<String>::new());
    }

    #[test]
    fn test_suffix() {
        let db = Database::new();

        assert_eq!(db.find_suffix("gulo"), Some("গুলো".to_string()));
        assert_eq!(db.find_suffix("er"), Some("ের".to_string()));
        assert_eq!(db.find_suffix("h"), None);
    }

    #[test]
    fn test_autocorrect() {
        let db = Database::new();

        assert_eq!(db.get_corrected("academy"), Some("oZakaDemi".to_string()));
        assert_eq!(db.get_corrected("\\nai\\"), None);
    }
}
