// Suggestion making module.

use ahash::RandomState;
use edit_distance::edit_distance;
use okkhor::parser::Parser;
use std::collections::{hash_map::Entry, HashMap};
use std::fs::read;
use regex::Regex;

use crate::config::{Config, get_phonetic_method_defaults};
use crate::data::Data;
use crate::phonetic::regex::parse;
use crate::utility::{push_checked, split_string, Utility};

pub(crate) struct PhoneticSuggestion {
    pub(crate) suggestions: Vec<String>,
    // Phonetic buffer. It's used to avoid allocations
    // for phonetic conversion every time.
    pbuffer: String,
    // Regex buffer. It's used to avoid allocations
    // for regex conversion every time.
    regex: String,
    // Cache for storing dictionary searches.
    cache: HashMap<String, Vec<String>, RandomState>,
    phonetic: Parser,
    table: HashMap<&'static str, &'static [&'static str], RandomState>,
    // Auto Correct caches.
    corrects: HashMap<String, String>,
    // The user's auto-correct entries.
    pub(crate) user_autocorrect: HashMap<String, String, RandomState>,
}

impl PhoneticSuggestion {
    pub(crate) fn new(config: &Config) -> Self {
        // Load the user's auto-correct entries.
        let user_autocorrect =
            if let Ok(file) = read(config.get_user_phonetic_autocorrect()) {
                serde_json::from_slice(&file).unwrap()
            } else {
                HashMap::with_hasher(RandomState::new())
            };
        
        let table: Vec<(&str, &[&str])> = vec![
            ("a", &["a", "aa", "e", "oi", "o", "nya", "y"]),
            ("b", &["b", "bh"]),
            ("c", &["c", "ch", "k"]),
            ("d", &["d", "dh", "dd", "ddh"]),
            ("e", &["i", "ii", "e", "y"]),
            ("f", &["ph"]),
            ("g", &["g", "gh", "j"]),
            ("h", &["h"]),
            ("i", &["i", "ii", "y"]),
            ("j", &["j", "jh", "z"]),
            ("k", &["k", "kh"]),
            ("l", &["l"]),
            ("m", &["h", "m"]),
            ("n", &["n", "nya", "nga", "nn"]),
            ("o", &["a", "u", "uu", "oi", "o", "ou", "y"]),
            ("p", &["p", "ph"]),
            ("q", &["k"]),
            ("r", &["rri", "h", "r", "rr", "rrh"]),
            ("s", &["s", "sh", "ss"]),
            ("t", &["t", "th", "tt", "tth", "khandatta"]),
            ("u", &["u", "uu", "y"]),
            ("v", &["bh"]),
            ("w", &["o"]),
            ("x", &["e", "k"]),
            ("y", &["i", "y"]),
            ("z", &["h", "j", "jh", "z"]),
        ];
        let table = table
        .into_iter()
        .collect();
        
        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            pbuffer: String::with_capacity(60),
            regex: String::with_capacity(1024),
            cache: HashMap::with_capacity_and_hasher(20, RandomState::new()),
            phonetic: Parser::new_phonetic(),
            corrects: HashMap::with_capacity(10),
            table,
            user_autocorrect,
        }
    }

    /// Add suffix(গুলো, মালা, etc.) to the dictionary suggestions and return them.
    ///
    /// This function gets the suggestion list from the stored cache.
    ///
    /// Handles Auto Corrected words specially. It includes them into
    /// the `self.corrects` directly to let them be one of the first suggestions.
    fn add_suffix_to_suggestions(&mut self, middle: &str, data: &Data) -> Vec<String> {
        // Fill up the list with what we have from the cache.
        let mut list = self.cache.get(middle).cloned().unwrap_or_default();

        if middle.len() > 2 {
            for i in 1..middle.len() {
                let suffix_key = &middle[i..];

                if let Some(suffix) = data.find_suffix(suffix_key) {
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
        data: &Data,
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

        self.suggestion_with_dict(&splitted_string, data);

        // Emoji addition with corresponding emoticon.
        if let Some(emoji) = data.get_emoji_by_emoticon(term) {
            // Add the emoticon
            // Sometimes the emoticon is captured as preceding meta characters and already included.
            if term != splitted_string.0 {
                self.suggestions.insert(0, term.to_owned());
            }
            self.suggestions.insert(0, emoji.to_owned());
            // Mark that we have added the typed text already (as the emoticon).
            typed_added = true;
        } else if let Some(emojis) = data.get_emoji_by_name(splitted_string.1) {
            // Emoji addition with it's name
            // Add preceding and trailing meta characters.
            let emojis = emojis.map(|s| format!("{}{}{}", splitted_string.0, s, splitted_string.2));
            if self.suggestions.len() > 3 {
                let mut remaining = self.suggestions.split_off(3);
                self.suggestions.extend(emojis);
                self.suggestions.append(&mut remaining);
            } else {
                self.suggestions.extend(emojis);
            }
        }

        // Include written English word if the feature is enabled and it is not included already.
        if config.get_suggestion_include_english() && !typed_added {
            self.suggestions.push(term.to_string());
        }

        let selection = self.get_prev_selection(&splitted_string, data, selections);

        (self.suggestions.clone(), selection)
    }

    /// Make suggestions from the given `splitted_string`. This will include dictionary and auto-correct suggestion.
    pub(crate) fn suggestion_with_dict(&mut self, splitted_string: &(&str, &str, &str), data: &Data) {
        self.suggestions.clear();

        self.phonetic
            .convert_into(splitted_string.1, &mut self.pbuffer);

        // We always cache the suggestions for future reuse and for adding suffix to the suggestions.
        if !self.cache.contains_key(splitted_string.1) {
            let mut suggestions: Vec<String> = Vec::new();

            if let Some(correct) = self.search_corrected(splitted_string.1, data) {
                let corrected = self.phonetic.convert(correct);
                // Add it in the corrected cache.
                self.corrects
                    .insert(splitted_string.1.to_string(), corrected.clone());
                suggestions.push(corrected);
            }

            self.include_from_dictionary(splitted_string.1, &mut suggestions, data);
            // Add the suggestions into the cache.
            self.cache
                .insert(splitted_string.1.to_string(), suggestions);
        }

        let mut suffixed_suggestions = self.add_suffix_to_suggestions(splitted_string.1, data);

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
        data: &Data,
        selections: &mut HashMap<String, String, RandomState>,
    ) -> usize {
        let len = splitted_string.1.len();
        let mut selected = String::with_capacity(len * 3);

        if let Some(item) = selections.get(splitted_string.1) {
            selected.push_str(item);
        } else if len >= 2 {
            for i in 1..len {
                let test = &splitted_string.1[len - i..len];

                if let Some(suffix) = data.find_suffix(test) {
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

    /// Find words from the dictionary with given word.
    pub(crate) fn include_from_dictionary(&mut self, word: &str, suggestions: &mut Vec<String>, data: &Data) {
        // Build the Regex string.
        parse(word, &mut self.regex);
        let rgx = Regex::new(&self.regex).unwrap();

        suggestions.extend(self.table
            .get(word.get(0..1).unwrap_or_default())
            .copied()
            .unwrap_or_default()
            .iter()
            .flat_map(|&item| data.get_words_for(item).filter(|i| rgx.is_match(i)).cloned()));
    }

    /// Search for a `term` in AutoCorrect dictionary.
    ///
    /// This looks in the user defined AutoCorrect entries first.
    fn search_corrected<'a>(&'a self, term: &str, data: &'a Data) -> Option<&'a str> {
        self.user_autocorrect
            .get(term)
            .map(String::as_str)
            .or_else(|| data.search_corrected(term))
    }
}

// Implement Default trait on PhoneticSuggestion, actually for testing convenience.
impl Default for PhoneticSuggestion {
    fn default() -> Self {
        let config = get_phonetic_method_defaults();
        PhoneticSuggestion {
            user_autocorrect: HashMap::with_hasher(RandomState::new()),
            ..PhoneticSuggestion::new(&config)
        }
    }
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;
    use std::collections::HashMap;

    use super::PhoneticSuggestion;
    use crate::config::get_phonetic_method_defaults;
    use crate::data::Data;
    use crate::utility::split_string;

    #[test]
    fn test_suggestion_with_english() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let mut config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        config.set_suggestion_include_english(true);

        suggestion.suggest(":)", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["😃", ":)", "ঃ)"]);

        suggestion.suggest(";)", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["😉", ";)"]);

        suggestion.suggest("{a}", &data, &mut selections, &config);
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
        let data = Data::new(&config);

        suggestion.suggest(":)", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["😃", ":)", "ঃ)"]);

        suggestion.suggest(";)", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["😉", ";)"]);

        suggestion.suggest("smile", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["স্মিলে", "😀", "😄"]);

        suggestion.suggest("cool", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["চুল", "চোল", "চল", "😎", "🆒", "চূল", "ছুল", "ছোল", "ছল", "ছুঁল"]);

        suggestion.suggest(".", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["।"]);
    }

    #[test]
    fn test_suggestion() {
        let mut suggestion = PhoneticSuggestion::default();
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);

        suggestion.suggestion_with_dict(&split_string("a", false), &data);
        assert_eq!(suggestion.suggestions, ["আ", "আঃ", "া", "এ", "অ্যা", "অ্যাঁ"]);

        suggestion.suggestion_with_dict(&split_string("as", false), &data);
        assert_eq!(suggestion.suggestions, ["আস", "আশ", "এস", "আঁশ"]);

        suggestion.suggestion_with_dict(&split_string("asgulo", false), &data);
        assert_eq!(
            suggestion.suggestions,
            ["আসগুলো", "আশগুলো", "এসগুলো", "আঁশগুলো", "আসগুল"]
        );

        suggestion.suggestion_with_dict(&split_string("(as)", false), &data);
        assert_eq!(suggestion.suggestions, ["(আস)", "(আশ)", "(এস)", "(আঁশ)"]);
    }

    #[test]
    fn test_suffix_suggestion() {
        let mut suggestion = PhoneticSuggestion::default();
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);

        suggestion.suggestion_with_dict(&split_string("a", false), &data);
        suggestion.suggestion_with_dict(&split_string("ap", false), &data);
        suggestion.suggestion_with_dict(&split_string("apn", false), &data);
        suggestion.suggestion_with_dict(&split_string("apni", false), &data);
        assert_eq!(suggestion.suggestions, ["আপনি", "আপনই", "আপ্নি"]);

        suggestion.suggestion_with_dict(&split_string("am", false), &data);
        suggestion.suggestion_with_dict(&split_string("ami", false), &data);
        assert_eq!(suggestion.suggestions, ["আমি", "আমই", "এমই"]);

        suggestion.suggestion_with_dict(&split_string("kkhet", false), &data);
        assert_eq!(
            suggestion.suggestions,
            ["ক্ষেত", "খেত", "খ্যাত", "খেট", "খ্যাঁত", "খেঁট", "খ্যাঁট"]
        );

        suggestion.suggestion_with_dict(&split_string("kkhetr", false), &data);
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

        suggestion.suggestion_with_dict(&split_string("kkhetre", false), &data);
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

        suggestion.suggestion_with_dict(&split_string("form", false), &data);
        assert_eq!(suggestion.suggestions, ["ফর্ম", "ফরম"]);

        suggestion.suggestion_with_dict(&split_string("forma", false), &data);
        assert_eq!(suggestion.suggestions, ["ফরমা", "ফর্মা"]);

        suggestion.suggestion_with_dict(&split_string("format", false), &data);
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত"]);

        suggestion.suggestion_with_dict(&split_string("formate", false), &data);
        assert_eq!(suggestion.suggestions, ["ফরম্যাটে", "ফরমাতে", "ফর্মাতে"]);

        suggestion.suggestion_with_dict(&split_string("formatt", false), &data);
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত্ত"]);

        suggestion.suggestion_with_dict(&split_string("formatte", false), &data);
        assert_eq!(suggestion.suggestions, ["ফরম্যাটতে", "ফরম্যাটে", "ফরমাত্তে"]);

        suggestion.suggestion_with_dict(&split_string("atm", false), &data);
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "অ্যাটম"]);

        suggestion.suggestion_with_dict(&split_string("atme", false), &data);
        assert_eq!(suggestion.suggestions, ["এটিএমে", "আত্মে", "অ্যাটমে"]);
        // Cache check
        suggestion.suggestion_with_dict(&split_string("atm", false), &data);
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "অ্যাটম"]);
    }

    #[test]
    fn test_suffix() {
        let mut cache = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);

        cache.insert("computer".to_string(), vec!["কম্পিউটার".to_string()]);
        cache.insert("i".to_string(), vec!["ই".to_string()]);
        cache.insert("hothat".to_string(), vec!["হঠাৎ".to_string()]);
        cache.insert("ebong".to_string(), vec!["এবং".to_string()]);

        let mut suggestion = PhoneticSuggestion {
            cache,
            ..PhoneticSuggestion::new(&config)
        };

        assert_eq!(
            suggestion.add_suffix_to_suggestions("computer", &data),
            ["কম্পিউটার"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions("computere", &data),
            ["কম্পিউটারে"]
        );
        assert_eq!(
            suggestion.add_suffix_to_suggestions("computergulo", &data),
            ["কম্পিউটারগুলো"]
        );
        // kar => য়
        assert_eq!(suggestion.add_suffix_to_suggestions("iei", &data), vec!["ইয়েই"]);
        // ৎ => ত
        assert_eq!(suggestion.add_suffix_to_suggestions("hothate", &data), ["হঠাতে"]);
        // ং => ঙ
        assert_eq!(
            suggestion.add_suffix_to_suggestions("ebongmala", &data),
            ["এবঙমালা"]
        );
    }

    #[test]
    fn test_prev_selected() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);

        selections.insert("onno".to_string(), "অন্য".to_string());
        selections.insert("i".to_string(), "ই".to_string());
        selections.insert("hothat".to_string(), "হঠাৎ".to_string());
        selections.insert("ebong".to_string(), "এবং".to_string());

        // Avoid meta characters
        suggestion.suggestions = vec!["*অন্ন?!".to_string(), "*অন্য?!".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("*onno?!", false), &data, &mut selections),
            1
        );

        // With Suffix
        suggestion.suggestions = vec!["ইএই".to_string(), "ইয়েই".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("iei", false), &data, &mut selections),
            1
        );

        suggestion.suggestions = vec![
            "হোথাতে".to_string(),
            "হথাতে".to_string(),
            "হঠাতে".to_string(),
        ];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("hothate", false), &data, &mut selections),
            2
        );

        suggestion.suggestions = vec!["এবংমালা".to_string(), "এবঙমালা".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("ebongmala", false), &data, &mut selections),
            1
        );

        // With Suffix + Avoid meta characters
        suggestion.suggestions = vec!["*অন্নগুলো?!".to_string(), "*অন্যগুলো?!".to_string()];
        assert_eq!(
            suggestion.get_prev_selection(&split_string("*onnogulo?!", false), &data, &mut selections),
            1
        );
    }

    #[test]
    fn test_suggest_special_chars_selections() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        selections.insert("sesh".to_string(), "শেষ".to_string());

        let (suggestions, selection) = suggestion.suggest("sesh", &data, &mut selections, &config);
        assert_eq!(suggestions, ["সেস", "শেষ", "সেশ"]);
        assert_eq!(selection, 1);

        let (suggestions, selection) = suggestion.suggest("sesh.", &data, &mut selections, &config);
        assert_eq!(suggestions, ["সেস।", "শেষ।", "সেশ।"]);
        assert_eq!(selection, 1);

        let (suggestions, _) = suggestion.suggest("sesh:", &data, &mut selections, &config);
        assert_eq!(suggestions, ["সেস", "শেষ", "সেশঃ"]);

        let (suggestions, selection) = suggestion.suggest("sesh:`", &data, &mut selections, &config);
        assert_eq!(suggestions, ["সেস:", "শেষ:", "সেশ:"]);
        assert_eq!(selection, 1);

        let (suggestions, _) = suggestion.suggest("6t``", &data, &mut selections, &config);
        assert_eq!(suggestions, ["৬ৎ"]);
    }

    #[test]
    fn test_database() {
        let config = get_phonetic_method_defaults();
        let mut suggestion = PhoneticSuggestion::default();
        let data = Data::new(&config);
        let mut suggestions = Vec::new();

        suggestion.include_from_dictionary("a", &mut suggestions, &data);
        assert_eq!(
            suggestions,
            ["অ্যা", "অ্যাঁ", "আ", "আঃ", "া", "এ",]
        );
        suggestions.clear();

        suggestion.include_from_dictionary("(", &mut suggestions, &data);
        assert_eq!(suggestions, Vec::<String>::new());
    }
}

#[cfg(feature = "bench")]
mod benches {
    extern crate test;

    use super::PhoneticSuggestion;
    use crate::{data::Data, utility::split_string, config::get_phonetic_method_defaults};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_phonetic_a(b: &mut Bencher) {
        let mut suggestion = PhoneticSuggestion::default();
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        let term = split_string("a", false);

        b.iter(|| {
            suggestion.cache.clear();
            suggestion.suggestion_with_dict(&term, &data);
        })
    }

    #[bench]
    fn bench_phonetic_kkhet(b: &mut Bencher) {
        let mut suggestion = PhoneticSuggestion::default();
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        let term = split_string("kkhet", false);

        b.iter(|| {
            suggestion.cache.clear();
            suggestion.suggestion_with_dict(&term, &data);
        })
    }

    #[bench]
    fn bench_phonetic_bistari(b: &mut Bencher) {
        let mut suggestion = PhoneticSuggestion::default();
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        let term = split_string("bistari", false);

        b.iter(|| {
            suggestion.cache.clear();
            suggestion.suggestion_with_dict(&term, &data);
        })
    }

    #[bench]
    fn bench_phonetic_database_a(b: &mut Bencher) {
        let config = get_phonetic_method_defaults();
        let mut suggestion = PhoneticSuggestion::default();
        let data = Data::new(&config);
        b.iter(|| {
            let mut suggestions = Vec::new();
            suggestion.include_from_dictionary("a", &mut suggestions, &data);
            black_box(suggestions);
        })
    }

    #[bench]
    fn bench_phonetic_database_aro(b: &mut Bencher) {
        let config = get_phonetic_method_defaults();
        let mut suggestion = PhoneticSuggestion::default();
        let data = Data::new(&config);
        b.iter(|| {
            let mut suggestions = Vec::new();
            suggestion.include_from_dictionary("arO", &mut suggestions, &data);
            black_box(suggestions);
        })
    }

    #[bench]
    fn bench_phonetic_database_bistari(b: &mut Bencher) {
        let config = get_phonetic_method_defaults();
        let mut suggestion = PhoneticSuggestion::default();
        let data = Data::new(&config);
        b.iter(|| {
            let mut suggestions = Vec::new();
            suggestion.include_from_dictionary("bistari", &mut suggestions, &data);
            black_box(suggestions);
        })
    }
}
