use regex::Regex;
use serde_json;
use rustc_hash::FxHashMap;

pub struct Database {
    table: FxHashMap<String, Vec<String>>,
    suffix: FxHashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            table: serde_json::from_str(include_str!("dictionary.json")).unwrap(),
            suffix: serde_json::from_str(include_str!("suffix.json")).unwrap(),
        }
    }

    /// Find words from the dictionary with given hint and a Regex
    pub fn find(&self, hint: &str, regex: &str) -> Vec<String> {
        let rgx = Regex::new(regex).unwrap();

        let table = match &hint[0..1] {
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
            "z" => vec!["h", "j", "jh", "z"],
            _ => vec![],
        };

        table
            .iter()
            .flat_map(|&item| {
                self.table[item].iter().filter_map(|i| {
                    if rgx.is_match(i) {
                        Some(i.to_owned())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn find_suffix(&self, string: &str) -> String {
        self.suffix.get(string).unwrap_or(&String::new()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::phonetic_regex::PhoneticRegex;
    use crate::database::Database;
    #[test]
    fn test_database() {
        let db = Database::new();
        let regex = PhoneticRegex::new().parse("a");
        assert_eq!(
            db.find("a", &regex),
            [
                "অ্যা",
                "অ্যাঁ",
                "আ",
                "আঃ",
                "া",
                "এ",
            ]
        );
    }

    #[test]
    fn test_suffix() {
        let db = Database::new();
        assert_eq!(db.find_suffix("gulo"), "গুলো");
        assert_eq!(db.find_suffix("er"), "ের");
        assert_eq!(db.find_suffix("h"), "");
    }
}
