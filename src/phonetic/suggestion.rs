// Suggestion making module.

use ahash::RandomState;
use edit_distance::edit_distance;
use emojicon::Emojicon;
use okkhor::parser::Parser;
use std::collections::{hash_map::Entry, HashMap};

use super::database::Database;
use crate::config::{get_phonetic_method_defaults, Config};
use crate::utility::{push_checked, split_string, Utility};

pub(crate) struct PhoneticSuggestion {
    pub(crate) suggestions: Vec<String>,
    pub(crate) database: Database,
    // Phonetic buffer. It's used to avoid allocations
    // for phonetic conversion every time.
    pbuffer: String,
    // Cache for storing dictionary searches.
    cache: HashMap<String, Vec<String>, RandomState>,
    phonetic: Parser,
    // Auto Correct caches.
    corrects: HashMap<String, String>,
    emojicon: Emojicon,
}

impl PhoneticSuggestion {
    pub(crate) fn new(config: &Config) -> Self {
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new_with_config(&config),
            pbuffer: String::with_capacity(60),
            cache: HashMap::with_capacity_and_hasher(20, RandomState::new()),
            phonetic: Parser::new_phonetic(),
            corrects: HashMap::with_capacity(10),
            emojicon: Emojicon::new(),
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
                            let suffix_lmc = suffix.chars().next().unwrap(); // Left most character.
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
    pub(crate) fn suggest_only_phonetic(&mut self, term: &str) -> String {
        let splitted_string = split_string(term, false);

        self.phonetic
            .convert_into(splitted_string.1, &mut self.pbuffer);

        format!(
            "{}{}{}",
            self.phonetic.convert(splitted_string.0),
            self.pbuffer,
            self.phonetic.convert(splitted_string.2)
        )
    }

    pub(crate) fn suggest(
        &mut self,
        term: &str,
        selections: &mut HashMap<String, String, RandomState>,
        config: &Config,
    ) -> (Vec<String>, usize) {
        let splitted_string = split_string(term, false);
        let mut typed_added = false;

        // Convert preceding and trailing meta characters into Bengali(phonetic representation).
        let splitted_string: (&str, &str, &str) = (
            &self.phonetic.convert(splitted_string.0),
            splitted_string.1,
            &self.phonetic.convert(splitted_string.2),
        );

        self.suggestion_with_dict(&splitted_string);

        // Emoji addition with corresponding emoticon.
        if let Some(emoji) = self.emojicon.get(term) {
            // Add the emoticon
            self.suggestions.insert(0, term.to_owned());
            self.suggestions.insert(0, emoji.to_owned());
            // Mark that we have added the typed text already (as the emoticon).
            typed_added = true;
        } else if let Some(emoji) = gh_emoji::get(splitted_string.1) {
            // Emoji addition with it's name
            // Add paddings
            let emoji = format!("{}{}{}", splitted_string.0, emoji, splitted_string.2);
            if self.suggestions.len() > 3 {
                self.suggestions.insert(3, emoji);
            } else {
                self.suggestions.push(emoji);
            }
        }

        // Include written English word if the feature is enabled and it is not included already.
        if config.get_suggestion_include_english() && !typed_added {
            self.suggestions.push(term.to_string());
        }

        let selection = self.get_prev_selection(&splitted_string, selections);

        (self.suggestions.clone(), selection)
    }

    /// Make suggestions from the given `splitted_string`. This will include dictionary and auto-correct suggestion.
    pub(crate) fn suggestion_with_dict(&mut self, splitted_string: &(&str, &str, &str)) {
        self.suggestions.clear();

        self.phonetic
            .convert_into(splitted_string.1, &mut self.pbuffer);

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
        suffixed_suggestions.sort_unstable_by(|a, b| {
            edit_distance(&self.pbuffer, a).cmp(&edit_distance(&self.pbuffer, b))
        });

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
        push_checked(&mut self.suggestions, self.pbuffer.clone());

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
        selections: &mut HashMap<String, String, RandomState>,
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
                        let suffix_lmc = suffix.chars().next().unwrap();
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
        let config = get_phonetic_method_defaults();
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            database: Database::new_with_config(&config),
            pbuffer: String::with_capacity(60),
            cache: HashMap::with_hasher(RandomState::new()),
            phonetic: Parser::new_phonetic(),
            corrects: HashMap::new(),
            emojicon: Emojicon::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;
    use std::collections::HashMap;

    use super::PhoneticSuggestion;
    use crate::config::get_phonetic_method_defaults;
    use crate::utility::split_string;

    #[test]
    fn test_suggestion_with_english() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let mut config = get_phonetic_method_defaults();
        config.set_suggestion_include_english(true);

        suggestion.suggest(":)", &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["😃", ":)", "ঃ)"]);

        suggestion.suggest("{a}", &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            ["{আ}", "{আঃ}", "{া}", "{🅰️}", "{এ}", "{অ্যা}", "{অ্যাঁ}", "{a}"]
        );
    }

    #[test]
    fn test_suggestion_only_phonetic() {
        let mut suggestion = PhoneticSuggestion::default();

        assert_eq!(suggestion.suggest_only_phonetic("{kotha}"), "{কথা}");
        assert_eq!(suggestion.suggest_only_phonetic(",ah,,"), ",আহ্‌");
    }

    #[test]
    fn test_emoticon_and_emoji() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();

        suggestion.suggest(":)", &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["😃", ":)", "ঃ)"]);

        suggestion.suggest("smile", &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["স্মিলে", "😄"]);

        suggestion.suggest("cool", &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["চুল", "চোল", "চল", "🆒", "চূল", "ছুল", "ছোল", "ছল", "ছুঁল"]);

        suggestion.suggest(".", &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["।"]);
    }

    #[test]
    fn test_suggestion() {
        let mut suggestion = PhoneticSuggestion::default();

        suggestion.suggestion_with_dict(&split_string("a", false));
        assert_eq!(suggestion.suggestions, ["আ", "আঃ", "া", "এ", "অ্যা", "অ্যাঁ"]);

        suggestion.suggestion_with_dict(&split_string("as", false));
        assert_eq!(suggestion.suggestions, ["আস", "আশ", "এস", "আঁশ"]);

        suggestion.suggestion_with_dict(&split_string("asgulo", false));
        assert_eq!(
            suggestion.suggestions,
            ["আসগুলো", "আশগুলো", "এসগুলো", "আঁশগুলো", "আসগুল"]
        );

        suggestion.suggestion_with_dict(&split_string("(as)", false));
        assert_eq!(suggestion.suggestions, ["(আস)", "(আশ)", "(এস)", "(আঁশ)"]);
    }

    #[test]
    fn test_suffix_suggestion() {
        let mut suggestion = PhoneticSuggestion::default();

        suggestion.suggestion_with_dict(&split_string("a", false));
        suggestion.suggestion_with_dict(&split_string("ap", false));
        suggestion.suggestion_with_dict(&split_string("apn", false));
        suggestion.suggestion_with_dict(&split_string("apni", false));
        assert_eq!(suggestion.suggestions, ["আপনি", "আপনই", "আপ্নি"]);

        suggestion.suggestion_with_dict(&split_string("am", false));
        suggestion.suggestion_with_dict(&split_string("ami", false));
        assert_eq!(suggestion.suggestions, ["আমি", "আমই", "এমই"]);

        suggestion.suggestion_with_dict(&split_string("kkhet", false));
        assert_eq!(
            suggestion.suggestions,
            ["ক্ষেত", "খেত", "খ্যাত", "খেট", "খ্যাঁত", "খেঁট", "খ্যাঁট"]
        );

        suggestion.suggestion_with_dict(&split_string("kkhetr", false));
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

        suggestion.suggestion_with_dict(&split_string("kkhetre", false));
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

        suggestion.suggestion_with_dict(&split_string("form", false));
        assert_eq!(suggestion.suggestions, ["ফর্ম", "ফরম"]);

        suggestion.suggestion_with_dict(&split_string("forma", false));
        assert_eq!(suggestion.suggestions, ["ফরমা", "ফর্মা"]);

        suggestion.suggestion_with_dict(&split_string("format", false));
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত"]);

        suggestion.suggestion_with_dict(&split_string("formate", false));
        assert_eq!(suggestion.suggestions, ["ফরম্যাটে", "ফরমাতে", "ফর্মাতে"]);

        suggestion.suggestion_with_dict(&split_string("formatt", false));
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত্ত"]);

        suggestion.suggestion_with_dict(&split_string("formatte", false));
        assert_eq!(suggestion.suggestions, ["ফরম্যাটতে", "ফরম্যাটে", "ফরমাত্তে"]);

        suggestion.suggestion_with_dict(&split_string("atm", false));
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "অ্যাটম"]);

        suggestion.suggestion_with_dict(&split_string("atme", false));
        assert_eq!(suggestion.suggestions, ["এটিএমে", "আত্মে", "অ্যাটমে"]);
        // Cache check
        suggestion.suggestion_with_dict(&split_string("atm", false));
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "অ্যাটম"]);
    }

    #[test]
    fn test_suffix() {
        let mut cache = HashMap::with_hasher(RandomState::new());
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
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        selections.insert("onno".to_string(), "অন্য".to_string());
        selections.insert("i".to_string(), "ই".to_string());
        selections.insert("hothat".to_string(), "হঠাৎ".to_string());
        selections.insert("ebong".to_string(), "এবং".to_string());

        // Avoid meta characters
        suggestion.suggestions = vec!["*অন্ন?!".to_string(), "*অন্য?!".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("*onno?!", false), &mut selections),
            1
        );

        // With Suffix
        suggestion.suggestions = vec!["ইএই".to_string(), "ইয়েই".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("iei", false), &mut selections),
            1
        );

        suggestion.suggestions = vec![
            "হোথাতে".to_string(),
            "হথাতে".to_string(),
            "হঠাতে".to_string(),
        ];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("hothate", false), &mut selections),
            2
        );

        suggestion.suggestions = vec!["এবংমালা".to_string(), "এবঙমালা".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("ebongmala", false), &mut selections),
            1
        );

        // With Suffix + Avoid meta characters
        suggestion.suggestions = vec!["*অন্নগুলো?!".to_string(), "*অন্যগুলো?!".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("*onnogulo?!", false), &mut selections),
            1
        );
    }

    #[test]
    fn test_suggest_special_chars_selections() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();
        selections.insert("sesh".to_string(), "শেষ".to_string());

        let (suggestions, selection) = suggestion.suggest("sesh", &mut selections, &config);
        assert_eq!(suggestions, ["সেস", "শেষ", "সেশ"]);
        assert_eq!(selection, 1);

        let (suggestions, selection) = suggestion.suggest("sesh.", &mut selections, &config);
        assert_eq!(suggestions, ["সেস।", "শেষ।", "সেশ।"]);
        assert_eq!(selection, 1);

        let (suggestions, _) = suggestion.suggest("sesh:", &mut selections, &config);
        assert_eq!(suggestions, ["সেস", "শেষ", "সেশঃ"]);

        let (suggestions, selection) = suggestion.suggest("sesh:`", &mut selections, &config);
        assert_eq!(suggestions, ["সেস:", "শেষ:", "সেশ:"]);
        assert_eq!(selection, 1);

        let (suggestions, _) = suggestion.suggest("6t``", &mut selections, &config);
        assert_eq!(suggestions, ["৬ৎ"]);
    }
}

#[cfg(feature = "bench")]
mod benches {
    extern crate test;

    use super::PhoneticSuggestion;
    use crate::utility::split_string;
    use test::Bencher;

    #[bench]
    fn bench_phonetic_a(b: &mut Bencher) {
        let mut suggestion = PhoneticSuggestion::default();
        let term = split_string("a", false);

        b.iter(|| {
            suggestion.cache.clear();
            suggestion.suggestion_with_dict(&term);
        })
    }

    #[bench]
    fn bench_phonetic_kkhet(b: &mut Bencher) {
        let mut suggestion = PhoneticSuggestion::default();
        let term = split_string("kkhet", false);

        b.iter(|| {
            suggestion.cache.clear();
            suggestion.suggestion_with_dict(&term);
        })
    }

    #[bench]
    fn bench_phonetic_bistari(b: &mut Bencher) {
        let mut suggestion = PhoneticSuggestion::default();
        let term = split_string("bistari", false);

        b.iter(|| {
            suggestion.cache.clear();
            suggestion.suggestion_with_dict(&term);
        })
    }
}
