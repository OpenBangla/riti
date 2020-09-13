// Suggestion making module.

use edit_distance::edit_distance;
use rupantor::parser::PhoneticParser;
use hashbrown::HashMap;

use super::database::Database;
use crate::settings;
use crate::utility::{split_string, Utility};

pub(crate) struct PhoneticSuggestion {
    pub(crate) suggestions: Vec<String>,
    pub(crate) database: Database,
    // Cache for storing dictionary searches.
    cache: HashMap<String, Vec<String>>,
    phonetic: PhoneticParser,
}

impl PhoneticSuggestion {
    pub(crate) fn new(layout: &serde_json::Value) -> Self {
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new(),
            cache: HashMap::new(),
            phonetic: PhoneticParser::new(layout),
        }
    }

    /// Add suffix(গুলো, মালা...etc) to the dictionary suggestions and return them.
    /// This function gets the suggestion list from the stored cache.
    fn add_suffix_to_suggestions(&self, middle: &str) -> Vec<String> {
        // Fill up the list with what we have from dictionary.
        let mut list = self.cache.get(middle).cloned().unwrap_or_default();

        if middle.len() > 2 {
            for i in 1..middle.len() {
                let suffix_key = &middle[i..];

                if let Some(suffix) = self.database.find_suffix(suffix_key) {
                    let key = &middle[0..(middle.len() - suffix_key.len())];
                    if let Some(cache) = self.cache.get(key) {
                        for item in cache {
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

    /// Make suggestion from given `term` with only phonetic transliteration.
    pub(crate) fn suggestion_only_phonetic(&self, term: &str) -> String {
        let splitted_string = split_string(term);

        format!(
            "{}{}{}",
            self.phonetic.convert(splitted_string.0),
            self.phonetic.convert(splitted_string.1),
            self.phonetic.convert(splitted_string.2)
        )
    }

    /// Make suggestions from the given `term`. This will include dictionary and auto-correct suggestion.
    pub(crate) fn suggestion_with_dict(&mut self, term: &str) -> Vec<String> {
        self.suggestions.clear();
        let splitted_string = split_string(term);

        // Convert preceding and trailing meta characters into Bengali(phonetic representation).
        let splitted_string: (&str, &str, &str) = (
            &self.phonetic.convert(splitted_string.0),
            splitted_string.1,
            &self.phonetic.convert(splitted_string.2),
        );

        let phonetic = self.phonetic.convert(splitted_string.1);

        if !self.cache.contains_key(splitted_string.1) {
            let mut dictionary = self.database.search_dictionary(splitted_string.1);

            dictionary.sort_unstable_by(|a, b| {
                edit_distance(&phonetic, a).cmp(&edit_distance(&phonetic, b))
            });

            if let Some(autocorrect) = self.database.search_corrected(splitted_string.1) {
                let corrected = self.phonetic.convert(&autocorrect);
                dictionary.insert(0, corrected);
            }

            self.cache.insert(splitted_string.1.to_string(), dictionary);
        }

        self.suggestions = self.add_suffix_to_suggestions(splitted_string.1);

        // Last Item: Phonetic. Check if it already contains.
        if !self.suggestions.contains(&phonetic) {
            self.suggestions.push(phonetic);
        }

        for item in self.suggestions.iter_mut() {
            *item = format!("{}{}{}", splitted_string.0, item, splitted_string.2);
        }

        // Emoticons Auto Corrects
        if let Some(emoticon) = self.database.search_corrected(term) {
            if emoticon == term {
                self.suggestions.insert(0, emoticon);
            }
        }

        // Include written English word if the feature is enabled.
        if settings::get_settings_phonetic_include_english()
            && !self.suggestions.iter().any(|i| i == term)
        {
            self.suggestions.push(term.to_string());
        }

        self.suggestions.clone()
    }

    pub(crate) fn get_prev_selection(
        &self,
        buffer: &str,
        selections: &mut HashMap<String, String>,
    ) -> usize {
        let splitted_string = split_string(buffer);
        let mut selected = String::new();
        let len = splitted_string.1.len();

        if let Some(item) = selections.get(splitted_string.1) {
            selected = item.clone();
        } else if len >= 2 {
            for i in 1..len {
                let test = &splitted_string.1[len - i..len];

                if let Some(suffix) = self.database.find_suffix(test) {
                    let key = &splitted_string.1[0..len - test.len()];

                    if let Some(word) = selections.get(key) {
                        let rmc = word.chars().last().unwrap();
                        let suffix_lmc = suffix.chars().nth(0).unwrap();

                        if rmc.is_vowel() && suffix_lmc.is_kar() {
                            selected = format!("{}{}{}", word, '\u{09DF}', suffix);
                        // \u{09DF} = B_Y
                        } else {
                            if rmc == '\u{09CE}' {
                                // \u{09CE} = ৎ
                                selected = format!(
                                    "{}{}{}",
                                    word.trim_end_matches('\u{09CE}'),
                                    '\u{09A4}',
                                    suffix
                                ); // \u{09A4} = ত
                            } else if rmc == '\u{0982}' {
                                // \u{0982} = ঃ
                                selected = format!(
                                    "{}{}{}",
                                    word.trim_end_matches('\u{0982}'),
                                    '\u{0999}',
                                    suffix
                                ); // \u09a4 = b_NGA
                            } else {
                                selected = format!("{}{}", word, suffix);
                            }
                        }

                        // Save this for future reuse.
                        selections.insert(splitted_string.1.to_string(), selected.to_string());
                    }
                }
            }
        }

        selected = format!("{}{}{}", splitted_string.0, selected, splitted_string.2);

        self.suggestions
            .iter()
            .position(|item| *item == selected)
            .unwrap_or_default()
    }
}

// Implement Default trait on PhoneticSuggestion, actually for testing convenience.
impl Default for PhoneticSuggestion {
    fn default() -> Self {
        let loader = crate::loader::LayoutLoader::load_from_settings();
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new(),
            cache: HashMap::new(),
            phonetic: PhoneticParser::new(loader.layout()),
        }
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;

    use super::PhoneticSuggestion;
    use crate::settings::{tests::set_default_phonetic, ENV_PHONETIC_INCLUDE_ENGLISH};

    //#[test] TODO: Enable this test after the environ variable data race issue is mitigated.
    fn test_suggestion_with_english() {
        set_default_phonetic();
        std::env::set_var(ENV_PHONETIC_INCLUDE_ENGLISH, "true");

        let mut suggestion = PhoneticSuggestion::default();

        assert_eq!(suggestion.suggestion_with_dict(":)"), vec![":)", "ঃ)"]);
        assert_eq!(
            suggestion.suggestion_with_dict("{a}"),
            vec!["{আ}", "{আঃ}", "{া}", "{এ}", "{অ্যা}", "{অ্যাঁ}", "{a}"]
        );
    }

    #[test]
    fn test_suggestion_only_phonetic() {
        set_default_phonetic();

        let suggestion = PhoneticSuggestion::default();

        assert_eq!(suggestion.suggestion_only_phonetic("{kotha}"), "{কথা}");
        assert_eq!(suggestion.suggestion_only_phonetic(",ah,,"), ",আহ্‌");
    }

    #[test]
    fn test_emoticon() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();

        assert_eq!(suggestion.suggestion_with_dict(":)"), vec![":)", "ঃ)"]);
        assert_eq!(suggestion.suggestion_with_dict("."), vec!["।"]);
    }

    #[test]
    fn test_suggestion() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();

        assert_eq!(
            suggestion.suggestion_with_dict("a"),
            vec!["আ", "আঃ", "া", "এ", "অ্যা", "অ্যাঁ"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("as"),
            vec!["আস", "আশ", "এস", "আঁশ"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("asgulo"),
            vec!["আসগুলো", "আশগুলো", "এসগুলো", "আঁশগুলো", "আসগুল"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("(as)"),
            vec!["(আস)", "(আশ)", "(এস)", "(আঁশ)"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("format"),
            vec!["ফরম্যাট", "ফরমাত"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("formate"),
            vec!["ফরম্যাটে", "ফরমাতে"]
        );

        // Suffix suggestion validation.
        assert_eq!(suggestion.suggestion_with_dict("apn"), vec!["আপন", "আপ্ন"]);
        assert_eq!(
            suggestion.suggestion_with_dict("apni"),
            vec!["আপনি", "আপনই", "আপ্নি"]
        );

        assert_eq!(suggestion.suggestion_with_dict("am"), vec!["আম", "এম"]);
        assert_eq!(
            suggestion.suggestion_with_dict("ami"),
            vec!["আমি", "আমই", "এমই"]
        );

        // Auto Correct suggestion should be the first one & Suffix suggestion validation.
        assert_eq!(
            suggestion.suggestion_with_dict("atm"),
            vec!["এটিএম", "আত্ম", "অ্যাটম"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("atme"),
            vec!["এটিএমে", "আত্মে", "অ্যাটমে"]
        );
    }

    #[test]
    fn test_suffix() {
        set_default_phonetic();

        let mut cache = HashMap::new();
        cache.insert("computer".to_string(), vec!["কম্পিউটার".to_string()]);
        cache.insert("ebong".to_string(), vec!["এবং".to_string()]);

        let suggestion = PhoneticSuggestion {
            cache,
            ..Default::default()
        };

        assert_eq!(
            suggestion.add_suffix_to_suggestions("computer"),
            vec!["কম্পিউটার"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions("computere"),
            vec!["কম্পিউটারে"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions("computergulo"),
            vec!["কম্পিউটারগুলো"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions("ebongmala"),
            vec!["এবঙমালা"]
        );
    }

    #[test]
    fn test_prev_selected() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::new();
        selections.insert("onno".to_string(), "অন্য".to_string());

        // Avoid meta characters
        suggestion.suggestions = vec!["*অন্ন?!".to_string(), "*অন্য?!".to_string()];
        assert_eq!(suggestion.get_prev_selection("*onno?!", &mut selections), 1);

        // With Suffix + Avoid meta characters
        suggestion.suggestions = vec!["*অন্নগুলো?!".to_string(), "*অন্যগুলো?!".to_string()];
        assert_eq!(
            suggestion.get_prev_selection("*onnogulo?!", &mut selections),
            1
        );
    }
}
