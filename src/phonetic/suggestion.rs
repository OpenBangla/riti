// Suggestion making module.

use crate::phonetic::database::Database;
use crate::utility::Utility;
use rustc_hash::FxHashMap;

pub(crate) struct PhoneticSuggestion {
    suggestions: Vec<String>,
    database: Database,
    // Cache for storing dictionary searches.
    cache: FxHashMap<String, Vec<String>>,
}

impl PhoneticSuggestion {
    pub(crate) fn new() -> Self {
        PhoneticSuggestion {
            suggestions: Vec::new(),
            database: Database::new(),
            cache: FxHashMap::default(),
        }
    }

    fn add_suffix(&self, list: Vec<String>, splitted: &(String, String, String)) -> Vec<String> {
        let middle = &splitted.1;
        let mut temp = list.clone();
        if middle.len() > 2 {
            for i in 1..middle.len() {
                let suffix_key = &middle[i..];
                let suffix = self.database.find_suffix(suffix_key);
                if suffix != "" {
                    let key = &middle[0..(middle.len() - suffix_key.len())];
                    if self.cache.contains_key(key) {
                        for item in &self.cache[key] {
                            let item_rmc = item.chars().last().unwrap().to_string(); // Right most character.
                            let suffix_lmc = suffix.chars().nth(0).unwrap().to_string(); // Left most character.
                            if item_rmc.is_vowel() && suffix_lmc.is_kar() {
                                let word = format!("{}{}{}", item, "\u{09DF}", suffix);
                                temp.push(word);
                            } else {
                                if item_rmc == "\u{09CE}" {
                                    let word = format!("{}{}{}", &item[0..item.len()-1], "\u{09A4}", suffix);
                                    temp.push(word);
                                } else if item_rmc == "\u{0982}" {
                                    let word = format!("{}{}{}", &item[0..item.len()-1], "\u{0999}", suffix);
                                    temp.push(word);
                                } else {
                                    let word = format!("{}{}", item, suffix);
                                    temp.push(word);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        temp
    }

    /// Make suggestion from the given `word`.
    pub(crate) fn suggest(&self, word: &str) -> Vec<String> {
        let splitted_string = word.split_string();
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;
    use super::PhoneticSuggestion;
    use crate::phonetic::database::Database;
    #[test]
    fn test_suffix() {
        let mut cache: FxHashMap<String, Vec<String>> = FxHashMap::default();
        cache.insert("computer".to_string(), vec!["কম্পিউটার".to_string()]);
        let suggestion = PhoneticSuggestion { suggestions: Vec::new(), database: Database::new(), cache };
        assert_eq!(suggestion.add_suffix(Vec::new(), &("".to_string(), "computere".to_string(), "".to_string())), vec!["কম্পিউটারে"]);
        assert_eq!(suggestion.add_suffix(Vec::new(), &("".to_string(), "computergulo".to_string(), "".to_string())), vec!["কম্পিউটারগুলো"]);
    }
}
