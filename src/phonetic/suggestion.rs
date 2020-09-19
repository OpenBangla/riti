// Suggestion making module.

use std::fmt::Write;

use edit_distance::edit_distance;
use hashbrown::HashMap;
use rupantor::parser::PhoneticParser;

use super::database::Database;
use crate::settings;
use crate::utility::{push_checked, split_string, Utility};

pub(crate) struct PhoneticSuggestion {
    pub(crate) suggestions: Vec<String>,
    pub(crate) database: Database,
    // Cache for storing dictionary searches.
    cache: HashMap<String, Vec<String>>,
    phonetic: PhoneticParser,
    // Auto Correct caches.
    corrects: Vec<String>,
}

impl PhoneticSuggestion {
    pub(crate) fn new(layout: &serde_json::Value) -> Self {
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new(),
            cache: HashMap::new(),
            phonetic: PhoneticParser::new(layout),
            corrects: Vec::new(),
        }
    }

    /// Add suffix(গুলো, মালা, etc.) to the dictionary suggestions and return them.
    ///
    /// This function gets the suggestion list from the stored cache.
    ///
    /// Handles Auto Corrected words specially. It includes them into
    /// the `self.suggestions` directly to let them be one of the first suggestions.
    fn add_suffix_to_suggestions(&mut self, middle: &str) -> Vec<String> {
        // Fill up the list with what we have from the cache.
        let mut list = self.cache.get(middle).cloned().unwrap_or_default();

        if middle.len() > 2 {
            for i in 1..middle.len() {
                let suffix_key = &middle[i..];

                if let Some(suffix) = self.database.find_suffix(suffix_key) {
                    let key = &middle[..(middle.len() - suffix_key.len())];
                    if let Some(cache) = self.cache.get(key) {
                        for base in cache {
                            let base_rmc = base.chars().last().unwrap(); // Right most character.
                            let suffix_lmc = suffix.chars().nth(0).unwrap(); // Left most character.
                            let mut word = base.clone();
                            match base_rmc {
                                ch if ch.is_vowel() && suffix_lmc.is_kar() => {
                                    // Insert য় in between.
                                    write!(&mut word, "{}{}", "য়", suffix).unwrap();
                                }
                                'ৎ' => {
                                    // Replace ৎ with ত
                                    word.pop();
                                    write!(&mut word, "{}{}", "ত", suffix).unwrap();
                                }
                                'ং' => {
                                    // Replace ং with ঙ
                                    word.pop();
                                    write!(&mut word, "{}{}", "ঙ", suffix).unwrap();
                                }
                                _ => word.push_str(&suffix),
                            }
                            // Check if the base was an auto corrected word.
                            // If it is, then add the suffixed word into the suggestion list
                            // to let it be one of the first ones.
                            if self.corrects.contains(base) {
                                push_checked(&mut self.suggestions, word);
                            } else {
                                list.push(word);
                            }
                        }
                    }
                }
            }
        }

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

        // We always cache the suggestions for future reuse and for adding suffix to the suggestions.
        if !self.cache.contains_key(splitted_string.1) {
            let mut suggestions: Vec<String> = Vec::new();

            if let Some(autocorrect) = self.database.search_corrected(splitted_string.1) {
                let corrected = self.phonetic.convert(&autocorrect);
                // Let the Auto Correct to be the first suggestion.
                self.suggestions.push(corrected.clone());
                // Add it in the corrected list.
                self.corrects.push(corrected.clone());
                suggestions.push(corrected);
            }

            suggestions.append(&mut self.database.search_dictionary(splitted_string.1));
            // Add the suggestions into the cache.
            self.cache
                .insert(splitted_string.1.to_string(), suggestions);
        }

        let mut suffixed_suggestions = self.add_suffix_to_suggestions(splitted_string.1);

        // Sort the list.
        suffixed_suggestions
            .sort_unstable_by(|a, b| edit_distance(&phonetic, a).cmp(&edit_distance(&phonetic, b)));

        for suggestion in suffixed_suggestions {
            push_checked(&mut self.suggestions, suggestion);
        }

        // Last Item: Phonetic
        push_checked(&mut self.suggestions, phonetic);

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
            // Watch out for emoticons!
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
                    let key = &splitted_string.1[..len - test.len()];

                    if let Some(base) = selections.get(key) {
                        selected = base.clone();
                        let rmc = base.chars().last().unwrap();
                        let suffix_lmc = suffix.chars().nth(0).unwrap();

                        match rmc {
                            ch if ch.is_vowel() && suffix_lmc.is_kar() => {
                                // Insert য় in between.
                                write!(&mut selected, "{}{}", 'য়', suffix).unwrap();
                            }
                            'ৎ' => {
                                // Replace ৎ with ত
                                selected.pop();
                                write!(&mut selected, "{}{}", 'ত', suffix).unwrap();
                            }
                            'ং' => {
                                // Replace ং with ঙ
                                selected.pop();
                                write!(&mut selected, "{}{}", 'ঙ', suffix).unwrap();
                            }
                            _ => selected.push_str(&suffix),
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
            corrects: Vec::new(),
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
    }

    #[test]
    fn test_suffix_suggestion() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();

        suggestion.suggestion_with_dict("a");
        suggestion.suggestion_with_dict("ap");
        suggestion.suggestion_with_dict("apn");
        assert_eq!(
            suggestion.suggestion_with_dict("apni"),
            vec!["আপনি", "আপনই", "আপ্নি"]
        );

        suggestion.suggestion_with_dict("am");
        assert_eq!(
            suggestion.suggestion_with_dict("ami"),
            vec!["আমি", "আমই", "এমই"]
        );

        assert_eq!(
            suggestion.suggestion_with_dict("kkhet"),
            vec!["ক্ষেত", "খেত", "খ্যাত", "খেট", "খ্যাঁত", "খেঁট", "খ্যাঁট"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("kkhetr"),
            vec![
                "ক্ষেত্র",
                "ক্ষেতর",
                "খেতর",
                "খ্যাতর",
                "খেটর",
                "খেঁটর",
                "খ্যাঁটর",
                "খ্যাঁতর"
            ]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("kkhetre"),
            vec![
                "ক্ষেত্রে",
                "ক্ষেতরে",
                "খেতরে",
                "খ্যাতরে",
                "খেটরে",
                "খেঁটরে",
                "খ্যাঁটরে",
                "খ্যাঁতরে"
            ]
        );

        assert_eq!(suggestion.suggestion_with_dict("form"), vec!["ফর্ম", "ফরম"]);
        assert_eq!(suggestion.suggestion_with_dict("forma"), ["ফরমা", "ফর্মা"]);
        assert_eq!(
            suggestion.suggestion_with_dict("format"),
            vec!["ফরম্যাট", "ফরমাত"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("formate"),
            vec!["ফরম্যাটে", "ফরমাতে", "ফর্মাতে"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("formatt"),
            vec!["ফরম্যাট", "ফরমাত্ত"]
        );
        assert_eq!(
            suggestion.suggestion_with_dict("formatte"),
            vec!["ফরম্যাটতে", "ফরম্যাটে", "ফরমাত্তে"]
        );

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
        cache.insert("i".to_string(), vec!["ই".to_string()]);
        cache.insert("hothat".to_string(), vec!["হঠাৎ".to_string()]);
        cache.insert("ebong".to_string(), vec!["এবং".to_string()]);

        let mut suggestion = PhoneticSuggestion {
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
        // kar => য়
        assert_eq!(suggestion.add_suffix_to_suggestions("iei"), vec!["ইয়েই"]);
        // ৎ => ত
        assert_eq!(
            suggestion.add_suffix_to_suggestions("hothate"),
            vec!["হঠাতে"]
        );
        // ং => ঙ
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
        selections.insert("i".to_string(), "ই".to_string());
        selections.insert("hothat".to_string(), "হঠাৎ".to_string());
        selections.insert("ebong".to_string(), "এবং".to_string());

        // Avoid meta characters
        suggestion.suggestions = vec!["*অন্ন?!".to_string(), "*অন্য?!".to_string()];
        assert_eq!(suggestion.get_prev_selection("*onno?!", &mut selections), 1);

        // With Suffix
        suggestion.suggestions = vec!["ইএই".to_string(), "ইয়েই".to_string()];
        assert_eq!(suggestion.get_prev_selection("iei", &mut selections), 1);

        suggestion.suggestions = vec![
            "হোথাতে".to_string(),
            "হথাতে".to_string(),
            "হঠাতে".to_string(),
        ];
        assert_eq!(suggestion.get_prev_selection("hothate", &mut selections), 2);

        suggestion.suggestions = vec!["এবংমালা".to_string(), "এবঙমালা".to_string()];
        assert_eq!(
            suggestion.get_prev_selection("ebongmala", &mut selections),
            1
        );

        // With Suffix + Avoid meta characters
        suggestion.suggestions = vec!["*অন্নগুলো?!".to_string(), "*অন্যগুলো?!".to_string()];
        assert_eq!(
            suggestion.get_prev_selection("*onnogulo?!", &mut selections),
            1
        );
    }
}
