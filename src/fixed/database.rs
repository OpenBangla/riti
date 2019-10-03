use hashbrown::HashMap;
use rayon::prelude::*;
use regex::Regex;
use std::fs::read_to_string;

use crate::settings::get_settings_database_dir;

pub(crate) struct Database {
    table: HashMap<String, Vec<String>>,
}

impl Database {
    pub(crate) fn new() -> Database {
        Database {
            table: serde_json::from_str(
                &read_to_string(get_settings_database_dir().join("dictionary.json"))
                    .unwrap(),
            )
            .unwrap(),
        }
    }

    /// Find words from the dictionary with given word.
    pub(crate) fn search_dictionary(&self, word: &str) -> Vec<String> {
        let table = match word.chars().nth(0).unwrap_or_default() {
            // Kars
            'া' => "aa",
            'ি' => "i",
            'ী' => "ii",
            'ু' => "u",
            'ূ' => "uu",
            'ৃ' => "rri",
            'ে' => "e",
            'ৈ' => "oi",
            'ো' => "o",
            'ৌ' => "ou",
            // Vowels
            'অ' => "a",
            'আ' => "aa",
            'ই' => "i",
            'ঈ' => "ii",
            'উ' => "u",
            'ঊ' => "uu",
            'ঋ' => "rri",
            'এ' => "e",
            'ঐ' => "oi",
            'ও' => "o",
            'ঔ' => "ou",
            // Consonants
            'ক' => "k",
            'খ' => "kh",
            'গ' => "g",
            'ঘ' => "gh",
            'ঙ' => "nga",
            'চ' => "c",
            'ছ' => "ch",
            'জ' => "j",
            'ঝ' => "jh",
            'ঞ' => "nya",
            'ট' => "tt",
            'ঠ' => "tth",
            'ড' => "dd",
            'ঢ' => "ddh",
            'ণ' => "nn",
            'ত' => "t",
            'থ' => "th",
            'দ' => "d",
            'ধ' => "dh",
            'ন' => "n",
            'প' => "p",
            'ফ' => "ph",
            'ব' => "b",
            'ভ' => "bh",
            'ম' => "m",
            'য' => "z",
            'র' => "r",
            'ল' => "l",
            'শ' => "sh",
            'ষ' => "ss",
            'স' => "s",
            'হ' => "h",
            'ড়' => "rr",
            'ঢ়' => "rrh",
            'য়' => "y",
            'ৎ' => "khandatta",
            // Otherwise we don't have any suggestions to search from, so return an empty vector.
            _ => return Vec::new(),
        };

        let need_chars_upto = match word.chars().count() {
            1 => 0,
            2..=3 => 1,
            _ => 5
        };

        let regex = format!("^{}[অআইঈউঊঋএঐওঔঌৡািীুূৃেৈোৌকখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহৎড়ঢ়য়ংঃঁ\u{09CD}]{{0,{}}}$", word, need_chars_upto);
        let rgx = Regex::new(&regex).unwrap();

        self.table[table]
            .par_iter()
            .filter(|i| rgx.is_match(i))
            .cloned()
            .collect()
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
            db.search_dictionary("ই"),
            ["ই"]
        );
        assert_eq!(
            db.search_dictionary("আমা"),
            ["আমা", "আমান", "আমার", "আমায়"]
        );
        assert_eq!(
            db.search_dictionary("1"),
            Vec::<String>::new()
        );
    }
}
