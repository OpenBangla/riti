// Suggestion making module.

use edit_distance::edit_distance;
use rupantor::parser::PhoneticParser;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

use super::database::Database;
use super::autocorrect::AutoCorrect;
use crate::utility::Utility;

pub(crate) struct PhoneticSuggestion {
    suggestions: Vec<String>,
    database: Database,
    pub(crate) autocorrect: AutoCorrect,
    // Cache for storing dictionary searches.
    cache: FxHashMap<String, Vec<String>>,
    phonetic: PhoneticParser,
}

impl PhoneticSuggestion {
    pub(crate) fn new(layout: &serde_json::Value) -> Self {
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new(),
            autocorrect: AutoCorrect::new(),
            cache: FxHashMap::default(),
            phonetic: PhoneticParser::new(layout),
        }
    }

    /// Add suffix(গুলো, মালা...etc) to the dictionary suggestions and return them.
    /// This function gets the suggestion list from the stored cache.
    fn add_suffix_to_suggestions(&self, splitted: &(&str, &str, &str)) -> Vec<String> {
        let middle = splitted.1;
        // Fill up the list with what we have from dictionary.
        let mut list = self.cache.get(splitted.1).cloned().unwrap_or_default();

        if middle.len() > 2 {
            for i in 1..middle.len() {
                let suffix_key = &middle[i..];
                if let Some(suffix) = self.database.find_suffix(suffix_key) {
                    let key = &middle[0..(middle.len() - suffix_key.len())];
                    if self.cache.contains_key(key) {
                        for item in &self.cache[key] {
                            let item_rmc = item.chars().last().unwrap(); // Right most character.
                            let suffix_lmc = suffix.chars().nth(0).unwrap(); // Left most character.
                            if item_rmc.is_vowel() && suffix_lmc.is_kar() {
                                let word = format!("{}{}{}", item, "\u{09DF}", suffix);
                                list.push(word);
                            } else {
                                if item_rmc == '\u{09CE}' {
                                    // Khandatta
                                    let word = format!(
                                        "{}{}{}",
                                        item.trim_end_matches('\u{09CE}'),
                                        "\u{09A4}",
                                        suffix
                                    );
                                    list.push(word);
                                } else if item_rmc == '\u{0982}' {
                                    // Anushar
                                    let word = format!(
                                        "{}{}{}",
                                        item.trim_end_matches('\u{0982}'),
                                        "\u{0999}",
                                        suffix
                                    );
                                    list.push(word);
                                } else {
                                    let word = format!("{}{}", item, suffix);
                                    list.push(word);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Remove duplicates.
        list.dedup();
        list
    }

    /// Make suggestions from the given `term`.
    pub(crate) fn suggest(&mut self, term: &str) -> Vec<String> {
        self.suggestions.clear();
        let splitted_string = split_string(term);

        // Convert preceding and trailing meta characters into Bengali(phonetic representation).
        let splitted_string: (&str, &str, &str) = (&self.phonetic.convert(splitted_string.0), splitted_string.1, &self.phonetic.convert(splitted_string.2));

        let phonetic = self.phonetic.convert(splitted_string.1);

        if !self.cache.contains_key(splitted_string.1) {
            let mut dictionary = self.database.search_dictionary(splitted_string.1);
            // Auto Correct
            if let Some(corrected) = self.autocorrect.search(splitted_string.1) {
                let word = self.phonetic.convert(&corrected);
                self.suggestions.push(word.clone());
                // Add it to the cache for adding suffix later.
                dictionary.push(word);
            }
            // Cache it.
            self.cache.insert(splitted_string.1.to_string(), dictionary);
        }

        let mut suggestions_with_suffix = self.add_suffix_to_suggestions(&splitted_string);

        suggestions_with_suffix.sort_by(|a, b| {
            let dist1 = edit_distance(&phonetic, a);
            let dist2 = edit_distance(&phonetic, b);

            if dist1 < dist2 {
                Ordering::Less
            } else if dist1 > dist2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        self.suggestions.append(&mut suggestions_with_suffix);

        // Last Item: Phonetic. Check if it already contains.
        if !self.suggestions.contains(&phonetic) {
            self.suggestions.push(phonetic);
        }

        for item in self.suggestions.iter_mut() {
            *item = format!("{}{}{}", splitted_string.0, item, splitted_string.2);
        }

        // Emoticons Auto Corrects
        if let Some(emoticon) = self.autocorrect.search(term) {
            if emoticon == term {
                self.suggestions.insert(0, emoticon);
            }
        }

        self.suggestions.clone()
    }

    /// Returns a slice of the `suggestions` vector.
    pub(crate) fn suggestions(&self) -> &[String] {
        &self.suggestions
    }
}

// Implement Default trait on PhoneticSuggestion, actually for testing convenience.
impl Default for PhoneticSuggestion {
    fn default() -> Self {
        let loader = crate::loader::LayoutLoader::load_from_settings();
        
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new(),
            autocorrect: AutoCorrect::new(),
            cache: FxHashMap::default(),
            phonetic: PhoneticParser::new(loader.layout()),
        }
    }
}

/// Split the string into three parts.
/// This function splits preceding and trailing meta characters.
fn split_string(input: &str) -> (&str, &str, &str) {
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
        return (&input[..], "", "");
    }

    for (index, c) in input.chars().rev().enumerate() {
        if !meta.contains(c) {
            last_index = input.len() - index - 1;
            break;
        }
    }

    let first_part = &input[0..first_index];
    let middle_part = &input[first_index..=last_index];
    let last_part = &input[last_index + 1..];

    (first_part, middle_part, last_part)
}

#[cfg(test)]
mod tests {
    use std::env::set_var;
    use rustc_hash::FxHashMap;

    use super::split_string;
    use super::PhoneticSuggestion;
    use crate::settings::ENV_LAYOUT_FILE;

    #[test]
    fn test_emoticon() {
        set_var(
            ENV_LAYOUT_FILE,
            format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/avrophonetic.json"),
        );

        let mut suggestion = PhoneticSuggestion::default();

        assert_eq!(suggestion.suggest(":)"), vec![":)", "ঃ)"]);
        assert_eq!(suggestion.suggest("."), vec!["।"]);
    }

    #[test]
    fn test_suggestion() {
        set_var(
            ENV_LAYOUT_FILE,
            format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/avrophonetic.json"),
        );

        let mut suggestion = PhoneticSuggestion::default();

        assert_eq!(
            suggestion.suggest("a"),
            vec![
                "আ",
                "আঃ",
                "া",
                "এ",
                "অ্যা",
                "অ্যাঁ"
            ]
        );
        assert_eq!(
            suggestion.suggest("as"),
            vec!["আস", "আশ", "এস", "আঁশ"]
        );
        assert_eq!(
            suggestion.suggest("asgulo"),
            vec![
                "আসগুলো",
                "আশগুলো",
                "এসগুলো",
                "আঁশগুলো",
                "আসগুল"
            ]
        );
        assert_eq!(
            suggestion.suggest("(as)"),
            vec!["(আস)", "(আশ)", "(এস)", "(আঁশ)"]
        );

        // Suffix suggestion validation
        assert_eq!(suggestion.suggest("apn"), vec!["আপন", "আপ্ন"]);
        assert_eq!(suggestion.suggest("apni"), vec!["আপনি", "আপনই", "আপ্নি"]);

        assert_eq!(suggestion.suggest("am"), vec!["আম", "এম"]);
        assert_eq!(suggestion.suggest("ami"), vec!["আমি", "আমই", "এমই"]);
    }

    #[test]
    fn test_suffix() {
        set_var(
            ENV_LAYOUT_FILE,
            format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/avrophonetic.json"),
        );

        let mut cache: FxHashMap<String, Vec<String>> = FxHashMap::default();
        cache.insert(
            "computer".to_string(),
            vec!["কম্পিউটার".to_string()],
        );
        cache.insert("ebong".to_string(), vec!["এবং".to_string()]);

        let suggestion = PhoneticSuggestion {
            cache,
            ..Default::default()
        };

        assert_eq!(
            suggestion.add_suffix_to_suggestions(&("", "computer", "")),
            vec!["কম্পিউটার"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions(&("", "computere", "")),
            vec!["কম্পিউটারে"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions(&("", "computergulo", "")),
            vec!["কম্পিউটারগুলো"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions(&("", "ebongmala", "")),
            vec!["এবঙমালা"]
        );
    }

    #[test]
    fn test_split_string() {
        assert_eq!(split_string("[][][][]"), ("[][][][]", "", ""));
        assert_eq!(split_string("t*"), ("", "t", "*"));
        assert_eq!(split_string("1"), ("", "1", ""));
        assert_eq!(
            split_string("#\"percent%sign\"#"),
            ("#\"", "percent%sign", "\"#")
        );
        assert_eq!(split_string("text"), ("", "text", ""));
        assert_eq!(split_string(":)"), ("", ":", ")"));
    }
}
