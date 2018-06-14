use regex::Regex;
use serde_json;
use std::collections::HashMap;

use regexparser::RegexParser;

pub struct Database {
    parser: RegexParser,
    table: HashMap<String, Vec<String>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            parser: RegexParser::new(),
            table: serde_json::from_str(include_str!("dictionary.json")).unwrap(),
        }
    }

    pub fn find(&self, word: &str) -> Vec<String> {
        let regex = Regex::new(&self.parser.parse(word)).unwrap();

        let table = match &word[0..1] {
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
                    if regex.is_match(i) {
                        Some(i.to_owned())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    #[test]
    fn test_database() {
        let db = Database::new();
        assert_eq!(
            db.find("a"),
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
}
