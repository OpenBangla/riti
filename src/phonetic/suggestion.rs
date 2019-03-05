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
                            let item_rmc = item.chars().last().unwrap(); // Right most character.
                            let suffix_lmc = suffix.chars().nth(0).unwrap(); // Left most character.
                            if item_rmc.is_vowel() && suffix_lmc.is_kar() {
                                let word = format!("{}{}{}", item, "\u{09DF}", suffix);
                                temp.push(word);
                            } else {
                                if item_rmc == '\u{09CE}' {
                                    // Khandatta
                                    let word = format!(
                                        "{}{}{}",
                                        item.trim_end_matches('\u{09CE}'),
                                        "\u{09A4}",
                                        suffix
                                    );
                                    temp.push(word);
                                } else if item_rmc == '\u{0982}' {
                                    // Anushar
                                    let word = format!(
                                        "{}{}{}",
                                        item.trim_end_matches('\u{0982}'),
                                        "\u{0999}",
                                        suffix
                                    );
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
        let splitted_string = split_string(word);
        Vec::new()
    }
}

/// Split the string into three parts.
/// This function splits preceding and trailing meta characters.
fn split_string(input: &str) -> (String, String, String) {
    let meta = "-]~!@#%&*()_=+[{}'\";<>/?|.,";
    let mut first_index = 0;
    let mut last_index = 0;
    let mut encountered_alpha = false;

    for (index, c) in input.chars().enumerate() {
        if !meta.contains(c) {
            first_index = index;
            encountered_alpha = true;
            break;
        }
    }

    // Corner case: If we haven't yet encountered an alpha or
    // a numeric character, then the string has no middle part
    // or last part we need. So return "" for them ;)
    if !encountered_alpha {
        return (input[..].to_owned(), "".to_owned(), "".to_owned());
    }

    for (index, c) in input.chars().rev().enumerate() {
        if !meta.contains(c) {
            last_index = input.len() - index - 1;
            break;
        }
    }

    let first_part = input[0..first_index].to_owned();
    let middle_part = input[first_index..=last_index].to_owned();
    let last_part = input[last_index + 1..].to_owned();

    (first_part, middle_part, last_part)
}

#[cfg(test)]
mod tests {
    use super::split_string;
    use super::PhoneticSuggestion;
    use crate::phonetic::database::Database;
    use rustc_hash::FxHashMap;
    #[test]
    fn test_suffix() {
        let mut cache: FxHashMap<String, Vec<String>> = FxHashMap::default();
        cache.insert(
            "computer".to_string(),
            vec!["কম্পিউটার".to_string()],
        );
        cache.insert("ebong".to_string(), vec!["এবং".to_string()]);

        let suggestion = PhoneticSuggestion {
            suggestions: Vec::new(),
            database: Database::new(),
            cache,
        };
        assert_eq!(
            suggestion.add_suffix(
                Vec::new(),
                &("".to_string(), "computere".to_string(), "".to_string())
            ),
            vec!["কম্পিউটারে"]
        );
        assert_eq!(
            suggestion.add_suffix(
                Vec::new(),
                &("".to_string(), "computergulo".to_string(), "".to_string())
            ),
            vec!["কম্পিউটারগুলো"]
        );
        assert_eq!(
            suggestion.add_suffix(
                Vec::new(),
                &("".to_string(), "ebongmala".to_string(), "".to_string())
            ),
            vec!["এবঙমালা"]
        );
    }

    #[test]
    fn test_split_string() {
        assert_eq!(
            split_string("[][][][]"),
            ("[][][][]".to_owned(), "".to_owned(), "".to_owned())
        );
        assert_eq!(
            split_string("t*"),
            ("".to_owned(), "t".to_owned(), "*".to_owned())
        );
        assert_eq!(
            split_string("1"),
            ("".to_owned(), "1".to_owned(), "".to_owned())
        );
        assert_eq!(
            split_string("#\"percent%sign\"#"),
            (
                "#\"".to_owned(),
                "percent%sign".to_owned(),
                "\"#".to_owned()
            )
        );
        assert_eq!(
            split_string("text"),
            ("".to_owned(), "text".to_owned(), "".to_owned())
        );
        assert_eq!(
            split_string(":)"),
            ("".to_owned(), ":".to_owned(), ")".to_owned())
        );
    }
}
