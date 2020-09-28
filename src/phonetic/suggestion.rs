// Suggestion making module.

use edit_distance::edit_distance;
use hashbrown::{hash_map::Entry, HashMap};
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
    corrects: HashMap<String, String>,
}

impl PhoneticSuggestion {
    pub(crate) fn new(layout: &serde_json::Value) -> Self {
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new(),
            cache: HashMap::with_capacity(20),
            phonetic: PhoneticParser::new(layout),
            corrects: HashMap::with_capacity(10),
        }
    }

    /// Add suffix(গুলো, মালা, etc.) to the dictionary suggestions and return them.
    ///
    /// This function gets the suggestion list from the stored cache.
    ///
    /// Handles Auto Corrected words specially. It includes them into
    /// the `self.corrects` directly to let them be one of the first suggestions.
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
                            let mut word = String::with_capacity(middle.len() * 3);
                            word.push_str(base);
                            match base_rmc {
                                ch if ch.is_vowel() && suffix_lmc.is_kar() => {
                                    // Insert য় in between.
                                    word.push('য়');
                                }
                                'ৎ' => {
                                    // Replace ৎ with ত
                                    word.pop();
                                    word.push('ত');
                                }
                                'ং' => {
                                    // Replace ং with ঙ
                                    word.pop();
                                    word.push('ঙ');
                                }
                                _ => (),
                            }
                            word.push_str(suffix);
                            // Check if the base was an auto corrected word.
                            // If it is, then add the suffixed word into the `self.corrects` cache
                            // to let it be one of the first suggestions.
                            if self.corrects.values().any(|v| v == base) {
                                if let Entry::Vacant(value) =
                                    self.corrects.entry(middle.to_string())
                                {
                                    value.insert(word);
                                } else {
                                    // Entry is already filled, so add the word in the general list.
                                    list.push(word);
                                }
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
    pub(crate) fn suggest_only_phonetic(&self, term: &str) -> String {
        let splitted_string = split_string(term);

        format!(
            "{}{}{}",
            self.phonetic.convert(splitted_string.0),
            self.phonetic.convert(splitted_string.1),
            self.phonetic.convert(splitted_string.2)
        )
    }

    pub(crate) fn suggest(
        &mut self,
        term: &str,
        selections: &mut HashMap<String, String>,
    ) -> (Vec<String>, usize) {
        let splitted_string = split_string(term);

        // Convert preceding and trailing meta characters into Bengali(phonetic representation).
        let splitted_string: (&str, &str, &str) = (
            &self.phonetic.convert(splitted_string.0),
            splitted_string.1,
            &self.phonetic.convert(splitted_string.2),
        );

        self.suggestion_with_dict(&splitted_string);

        // Emoticons Auto Corrects
        if let Some(emoticon) = self.database.search_corrected(term) {
            if emoticon == term {
                self.suggestions.insert(0, emoticon.to_string());
            }
        }

        // Include written English word if the feature is enabled.
        if settings::get_settings_phonetic_include_english()
            // Watch out for emoticons!
            && !self.suggestions.iter().any(|i| i == term)
        {
            self.suggestions.push(term.to_string());
        }

        let selection = self.get_prev_selection(&splitted_string, selections);

        (self.suggestions.clone(), selection)
    }

    /// Make suggestions from the given `splitted_string`. This will include dictionary and auto-correct suggestion.
    pub(crate) fn suggestion_with_dict(&mut self, splitted_string: &(&str, &str, &str)) {
        self.suggestions.clear();

        let phonetic = self.phonetic.convert(splitted_string.1);

        // We always cache the suggestions for future reuse and for adding suffix to the suggestions.
        if !self.cache.contains_key(splitted_string.1) {
            let mut suggestions: Vec<String> = Vec::new();

            if let Some(correct) = self.database.search_corrected(splitted_string.1) {
                let corrected = self.phonetic.convert(correct);
                // Add it in the corrected cache.
                self.corrects
                    .insert(splitted_string.1.to_string(), corrected.clone());
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

        // First Item: Auto Correct
        // Get the corrected one from the auto correct cache.
        if let Some(corrected) = self.corrects.get(splitted_string.1) {
            self.suggestions.push(corrected.clone());
        }

        // Middle Items: Dictionary suggestions
        for suggestion in suffixed_suggestions {
            push_checked(&mut self.suggestions, suggestion);
        }

        // Last Item: Phonetic
        push_checked(&mut self.suggestions, phonetic);

        // Add those preceding and trailing meta characters.
        if !splitted_string.0.is_empty() || !splitted_string.2.is_empty() {
            for item in self.suggestions.iter_mut() {
                *item = format!("{}{}{}", splitted_string.0, item, splitted_string.2);
            }
        }
    }

    pub(crate) fn get_prev_selection(
        &self,
        splitted_string: &(&str, &str, &str),
        selections: &mut HashMap<String, String>,
    ) -> usize {
        let len = splitted_string.1.len();
        let mut selected = String::with_capacity(len * 3);

        if let Some(item) = selections.get(splitted_string.1) {
            selected.push_str(item);
        } else if len >= 2 {
            for i in 1..len {
                let test = &splitted_string.1[len - i..len];

                if let Some(suffix) = self.database.find_suffix(test) {
                    let key = &splitted_string.1[..len - test.len()];

                    if let Some(base) = selections.get(key) {
                        let rmc = base.chars().last().unwrap();
                        let suffix_lmc = suffix.chars().nth(0).unwrap();
                        selected.push_str(base);

                        match rmc {
                            ch if ch.is_vowel() && suffix_lmc.is_kar() => {
                                // Insert য় in between.
                                selected.push('য়');
                            }
                            'ৎ' => {
                                // Replace ৎ with ত
                                selected.pop();
                                selected.push('ত');
                            }
                            'ং' => {
                                // Replace ং with ঙ
                                selected.pop();
                                selected.push('ঙ');
                            }
                            _ => (),
                        }
                        selected.push_str(suffix);

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
            corrects: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;

    use super::PhoneticSuggestion;
    use crate::settings::{tests::set_default_phonetic, ENV_PHONETIC_INCLUDE_ENGLISH};
    use crate::utility::split_string;

    //#[test] TODO: Enable this test after the environ variable data race issue is mitigated.
    fn test_suggestion_with_english() {
        set_default_phonetic();
        std::env::set_var(ENV_PHONETIC_INCLUDE_ENGLISH, "true");

        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::new();

        suggestion.suggest(":)", &mut selections);
        assert_eq!(suggestion.suggestions, [":)", "ঃ)"]);

        suggestion.suggest("{a}", &mut selections);
        assert_eq!(
            suggestion.suggestions,
            ["{আ}", "{আঃ}", "{া}", "{এ}", "{অ্যা}", "{অ্যাঁ}", "{a}"]
        );
    }

    #[test]
    fn test_suggestion_only_phonetic() {
        set_default_phonetic();

        let suggestion = PhoneticSuggestion::default();

        assert_eq!(suggestion.suggest_only_phonetic("{kotha}"), "{কথা}");
        assert_eq!(suggestion.suggest_only_phonetic(",ah,,"), ",আহ্‌");
    }

    #[test]
    fn test_emoticon() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::new();

        suggestion.suggest(":)", &mut selections);
        assert_eq!(suggestion.suggestions, [":)", "ঃ)"]);

        suggestion.suggest(".", &mut selections);
        assert_eq!(suggestion.suggestions, ["।"]);
    }

    #[test]
    fn test_suggestion() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();

        suggestion.suggestion_with_dict(&split_string("a"));
        assert_eq!(suggestion.suggestions, ["আ", "আঃ", "া", "এ", "অ্যা", "অ্যাঁ"]);

        suggestion.suggestion_with_dict(&split_string("as"));
        assert_eq!(suggestion.suggestions, ["আস", "আশ", "এস", "আঁশ"]);

        suggestion.suggestion_with_dict(&split_string("asgulo"));
        assert_eq!(
            suggestion.suggestions,
            ["আসগুলো", "আশগুলো", "এসগুলো", "আঁশগুলো", "আসগুল"]
        );

        suggestion.suggestion_with_dict(&split_string("(as)"));
        assert_eq!(suggestion.suggestions, ["(আস)", "(আশ)", "(এস)", "(আঁশ)"]);
    }

    #[test]
    fn test_suffix_suggestion() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();

        suggestion.suggestion_with_dict(&split_string("a"));
        suggestion.suggestion_with_dict(&split_string("ap"));
        suggestion.suggestion_with_dict(&split_string("apn"));
        suggestion.suggestion_with_dict(&split_string("apni"));
        assert_eq!(suggestion.suggestions, ["আপনি", "আপনই", "আপ্নি"]);

        suggestion.suggestion_with_dict(&split_string("am"));
        suggestion.suggestion_with_dict(&split_string("ami"));
        assert_eq!(suggestion.suggestions, ["আমি", "আমই", "এমই"]);

        suggestion.suggestion_with_dict(&split_string("kkhet"));
        assert_eq!(
            suggestion.suggestions,
            ["ক্ষেত", "খেত", "খ্যাত", "খেট", "খ্যাঁত", "খেঁট", "খ্যাঁট"]
        );

        suggestion.suggestion_with_dict(&split_string("kkhetr"));
        assert_eq!(
            suggestion.suggestions,
            [
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

        suggestion.suggestion_with_dict(&split_string("kkhetre"));
        assert_eq!(
            suggestion.suggestions,
            [
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

        suggestion.suggestion_with_dict(&split_string("form"));
        assert_eq!(suggestion.suggestions, ["ফর্ম", "ফরম"]);

        suggestion.suggestion_with_dict(&split_string("forma"));
        assert_eq!(suggestion.suggestions, ["ফরমা", "ফর্মা"]);

        suggestion.suggestion_with_dict(&split_string("format"));
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত"]);

        suggestion.suggestion_with_dict(&split_string("formate"));
        assert_eq!(suggestion.suggestions, ["ফরম্যাটে", "ফরমাতে", "ফর্মাতে"]);

        suggestion.suggestion_with_dict(&split_string("formatt"));
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত্ত"]);

        suggestion.suggestion_with_dict(&split_string("formatte"));
        assert_eq!(suggestion.suggestions, ["ফরম্যাটতে", "ফরম্যাটে", "ফরমাত্তে"]);

        suggestion.suggestion_with_dict(&split_string("atm"));
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "অ্যাটম"]);

        suggestion.suggestion_with_dict(&split_string("atme"));
        assert_eq!(suggestion.suggestions, ["এটিএমে", "আত্মে", "অ্যাটমে"]);
        // Cache check
        suggestion.suggestion_with_dict(&split_string("atm"));
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "অ্যাটম"]);
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
            ["কম্পিউটার"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions("computere"),
            ["কম্পিউটারে"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions("computergulo"),
            ["কম্পিউটারগুলো"]
        );
        // kar => য়
        assert_eq!(suggestion.add_suffix_to_suggestions("iei"), vec!["ইয়েই"]);
        // ৎ => ত
        assert_eq!(suggestion.add_suffix_to_suggestions("hothate"), ["হঠাতে"]);
        // ং => ঙ
        assert_eq!(
            suggestion.add_suffix_to_suggestions("ebongmala"),
            ["এবঙমালা"]
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
        assert_eq!(
            suggestion.get_prev_selection(&split_string("*onno?!"), &mut selections),
            1
        );

        // With Suffix
        suggestion.suggestions = vec!["ইএই".to_string(), "ইয়েই".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("iei"), &mut selections),
            1
        );

        suggestion.suggestions = vec![
            "হোথাতে".to_string(),
            "হথাতে".to_string(),
            "হঠাতে".to_string(),
        ];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("hothate"), &mut selections),
            2
        );

        suggestion.suggestions = vec!["এবংমালা".to_string(), "এবঙমালা".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("ebongmala"), &mut selections),
            1
        );

        // With Suffix + Avoid meta characters
        suggestion.suggestions = vec!["*অন্নগুলো?!".to_string(), "*অন্যগুলো?!".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("*onnogulo?!"), &mut selections),
            1
        );
    }

    #[test]
    fn test_suggest_special_chars_selections() {
        set_default_phonetic();

        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::new();
        selections.insert("sesh".to_string(), "শেষ".to_string());

        let (suggestions, selection) = suggestion.suggest("sesh", &mut selections);
        assert_eq!(suggestions, ["সেস", "শেষ", "সেশ"]);
        assert_eq!(selection, 1);

        let (suggestions, selection) = suggestion.suggest("sesh.", &mut selections);
        assert_eq!(suggestions, ["সেস।", "শেষ।", "সেশ।"]);
        assert_eq!(selection, 1);

        let (suggestions, selection) = suggestion.suggest("sesh:", &mut selections);
        assert_eq!(suggestions, ["সেসঃ", "শেষঃ", "সেশঃ"]);
        assert_eq!(selection, 1);

        let (suggestions, selection) = suggestion.suggest("sesh:`", &mut selections);
        assert_eq!(suggestions, ["সেস:", "শেষ:", "সেশ:"]);
        assert_eq!(selection, 1);

        let (suggestions, _) = suggestion.suggest("6t``", &mut selections);
        assert_eq!(suggestions, ["৬ৎ"]);
    }
}
