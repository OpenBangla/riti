// Suggestion making module.

use ahash::RandomState;
use okkhor::parser::Parser;
use regex::Regex;
use std::collections::HashMap;

use crate::config::Config;
use crate::data::Data;
use crate::phonetic::regex::parse;
use crate::suggestion::Rank;
use crate::utility::{push_checked, Utility, smart_quoter, SplittedString};

pub(crate) struct PhoneticSuggestion {
    pub(crate) suggestions: Vec<Rank>,
    // Phonetic buffer. It's used to avoid allocations
    // for phonetic conversion every time.
    pbuffer: String,
    // Regex buffer. It's used to avoid allocations
    // for regex conversion every time.
    regex: String,
    // Cache for storing dictionary searches.
    cache: HashMap<String, Vec<Rank>, RandomState>,
    phonetic: Parser,
    table: HashMap<&'static str, &'static [&'static str], RandomState>,
    // The user's auto-correct entries.
    pub(crate) user_autocorrect: HashMap<String, String, RandomState>,
}

impl PhoneticSuggestion {
    pub(crate) fn new(user_autocorrect: HashMap<String, String, RandomState>) -> Self {
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
        let table = table.into_iter().collect();

        PhoneticSuggestion {
            suggestions: Vec::with_capacity(10),
            pbuffer: String::with_capacity(60),
            regex: String::with_capacity(1024),
            cache: HashMap::with_capacity_and_hasher(20, RandomState::new()),
            phonetic: Parser::new_phonetic(),
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
    fn add_suffix_to_suggestions(&mut self, middle: &str, data: &Data) -> Vec<Rank> {
        // Fill up the list with what we have from the cache.
        let mut list = self.cache.get(middle).cloned().unwrap_or_default();

        if middle.len() > 2 {
            for i in 1..middle.len() {
                let suffix_key = &middle[i..];

                if let Some(suffix) = data.find_suffix(suffix_key) {
                    let key = &middle[..(middle.len() - suffix_key.len())];
                    if let Some(cache) = self.cache.get(key) {
                        for base in cache {
                            let base_rmc = base.to_string().chars().last().unwrap(); // Right most character.
                            let suffix_lmc = suffix.chars().next().unwrap(); // Left most character.
                            let mut word = String::with_capacity(middle.len() * 3);
                            word.push_str(base.to_string());
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

                            let mut new = base.clone();
                            // This changes the suggestion with the suffixed one while keeping the ranking intact.
                            *new.change_item() = word;
                            list.push(new);
                        }
                    }
                }
            }
        }

        list
    }

    /// Make suggestion from given `term` with only phonetic transliteration.
    pub(crate) fn suggest_only_phonetic(&mut self, term: &str) -> String {
        let string = SplittedString::split(term, false);

        self.phonetic
            .convert_into(string.word(), &mut self.pbuffer);

        format!(
            "{}{}{}",
            self.phonetic.convert(string.preceding()),
            self.pbuffer,
            self.phonetic.convert(string.trailing())
        )
    }

    pub(crate) fn suggest(
        &mut self,
        term: &str,
        data: &Data,
        selections: &mut HashMap<String, String, RandomState>,
        config: &Config,
    ) -> (Vec<Rank>, usize) {
        let mut string = SplittedString::split(term, false);
        let mut typed_added = false;

        // Convert preceding and trailing meta characters into Bengali(phonetic representation).
        string.map(|p,t| (self.phonetic.convert(p), self.phonetic.convert(t)));

        // Smart Quoting feature
        if config.get_smart_quote() {
            string = smart_quoter(string);
        }

        self.suggestion_with_dict(&string, data);

        // Emoji addition with corresponding emoticon (if ANSI mode is not enabled).
        if !config.get_ansi_encoding() {
            if let Some(emoji) = data.get_emoji_by_emoticon(term) {
                // Add the emoticon
                // Sometimes the emoticon is captured as preceding meta characters and already included.
                if term != string.preceding() {
                    self.suggestions.push(Rank::last_ranked(term.to_owned(), 1));
                }
                self.suggestions.push(Rank::emoji(emoji.to_owned()));
                // Mark that we have added the typed text already (as the emoticon).
                typed_added = true;
            } else if let Some(emojis) = data.get_emoji_by_name(string.word()) {
                // Emoji addition with it's name
                // Add preceding and trailing meta characters.
                let emojis = emojis.zip(1..).map(|(s, r)| {
                    Rank::emoji_ranked(
                        format!("{}{}{}", string.preceding(), s, string.trailing()),
                        r,
                    )
                });
                self.suggestions.extend(emojis);
            }
        }

        // Include written English word if the feature is enabled and it is not included already.
        if config.get_suggestion_include_english() && !typed_added {
            self.suggestions
                .push(Rank::last_ranked(term.to_string(), 3));
        }

        // Sort the suggestions.
        self.suggestions.sort();

        let selection = self.get_prev_selection(&string, data, selections);

        (self.suggestions.clone(), selection)
    }

    /// Make suggestions from the given `splitted_string`. This will include dictionary and auto-correct suggestion.
    pub(crate) fn suggestion_with_dict(
        &mut self,
        string: &SplittedString,
        data: &Data,
    ) {
        self.suggestions.clear();

        self.phonetic
            .convert_into(string.word(), &mut self.pbuffer);

        let phonetic = self.pbuffer.clone();

        // We always cache the suggestions for future reuse and for adding suffix to the suggestions.
        if !self.cache.contains_key(string.word()) {
            let mut suggestions: Vec<Rank> = Vec::new();

            // Auto Correct item.
            if let Some(correct) = self.search_corrected(string.word(), data) {
                let corrected = self.phonetic.convert(correct);
                // Treat it as the first priority.
                suggestions.push(Rank::first_ranked(corrected));
            }

            self.include_from_dictionary(string.word(), &phonetic, &mut suggestions, data);
            // Add the suggestions into the cache.
            self.cache
                .insert(string.word().to_string(), suggestions);
        }

        let suffixed_suggestions = self.add_suffix_to_suggestions(string.word(), data);

        // Middle Items: Dictionary suggestions
        for suggestion in suffixed_suggestions {
            push_checked(&mut self.suggestions, suggestion);
        }

        // Last Item: Phonetic
        push_checked(&mut self.suggestions, Rank::last_ranked(phonetic, 2));

        // Add those preceding and trailing meta characters.
        if !string.preceding().is_empty() || !string.trailing().is_empty() {
            for item in self.suggestions.iter_mut() {
                *item.change_item() = format!(
                    "{}{}{}",
                    string.preceding(),
                    item.to_string(),
                    string.trailing()
                );
            }
        }
    }

    pub(crate) fn get_prev_selection(
        &self,
        string: &SplittedString,
        data: &Data,
        selections: &mut HashMap<String, String, RandomState>,
    ) -> usize {
        let len = string.word().len();
        let mut selected = String::with_capacity(len * 3);

        if let Some(item) = selections.get(string.word()) {
            selected.push_str(item);
        } else if len >= 2 {
            for i in 1..len {
                let test = &string.word()[len - i..len];

                if let Some(suffix) = data.find_suffix(test) {
                    let key = &string.word()[..len - test.len()];

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
                        selections.insert(string.word().to_string(), selected.to_string());
                    }
                }
            }
        }

        selected = format!("{}{}{}", string.preceding(), selected, string.trailing());

        self.suggestions
            .iter()
            .position(|item| *item.to_string() == selected)
            .unwrap_or_default()
    }

    /// Find words from the dictionary with given `word` and rank them according the `base` word.
    pub(crate) fn include_from_dictionary(
        &mut self,
        word: &str,
        base: &str,
        suggestions: &mut Vec<Rank>,
        data: &Data,
    ) {
        // Build the Regex string.
        parse(word, &mut self.regex);
        let rgx = Regex::new(&self.regex).unwrap();

        suggestions.extend(
            self.table
                .get(word.get(0..1).unwrap_or_default())
                .copied()
                .unwrap_or_default()
                .iter()
                .flat_map(|&item| {
                    data.get_words_for(item)
                        .filter(|i| rgx.is_match(i))
                        .map(|s| Rank::new_suggestion(s.to_owned(), base))
                }),
        );
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
        PhoneticSuggestion::new(HashMap::with_hasher(RandomState::new()))
    }
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;
    use std::collections::HashMap;

    use super::PhoneticSuggestion;
    use crate::config::get_phonetic_method_defaults;
    use crate::data::Data;
    use crate::suggestion::Rank;
    use crate::utility::SplittedString;

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
            ["{আ}", "{🅰️}", "{আঃ}", "{া}", "{এ}", "{অ্যা}", "{অ্যাঁ}", "{a}"]
        );
    }

    #[test]
    fn test_suggestion_ansi() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let mut config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        config.set_suggestion_include_english(true);
        config.set_ansi_encoding(true);

        suggestion.suggest(":)", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["ঃ)"]);

        suggestion.suggest(";)", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, [";)"]);

        suggestion.suggest("{a}", &data, &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            ["{আ}", "{আঃ}", "{া}", "{এ}", "{অ্যা}", "{অ্যাঁ}"]
        );
    }

    #[test]
    fn test_suggestion_smart_quotes() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let mut config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        config.set_suggestion_include_english(true);
        config.set_smart_quote(true);

        suggestion.suggest("\"e\"", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["“এ”", "“ে”", "\"e\""]);

        config.set_smart_quote(false);

        suggestion.suggest("\"e\"", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["\"এ\"", "\"ে\"", "\"e\""]);
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
        assert_eq!(suggestion.suggestions, ["😀", "😄", "স্মিলে"]);

        suggestion.suggest("cool", &data, &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            ["চুল", "😎", "🆒", "চোল", "চল", "চূল", "ছুল", "ছোল", "ছল", "ছুঁল"]
        );

        suggestion.suggest(".", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["।"]);
    }

    #[test]
    fn test_suggestion() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);

        suggestion.suggest("a", &data, &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            ["আ", "🅰️", "আঃ", "া", "এ", "অ্যা", "অ্যাঁ"]
        );

        suggestion.suggest("as", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["আস", "আশ", "এস", "আঁশ"]);

        suggestion.suggest("asgulo", &data, &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            ["আসগুলো", "আশগুলো", "এসগুলো", "আঁশগুলো", "আসগুল"]
        );

        suggestion.suggest("(as)", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["(আস)", "(আশ)", "(এস)", "(আঁশ)"]);
    }

    #[test]
    fn test_suffix_suggestion() {
        let mut suggestion = PhoneticSuggestion::default();
        let mut selections = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);

        suggestion.suggest("a", &data, &mut selections, &config);
        suggestion.suggest("ap", &data, &mut selections, &config);
        suggestion.suggest("apn", &data, &mut selections, &config);
        suggestion.suggest("apni", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["আপনি", "আপনই", "আপ্নি"]);

        suggestion.suggest("am", &data, &mut selections, &config);
        suggestion.suggest("ami", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["আমি", "আমই", "এমই"]);

        suggestion.suggest("kkhet", &data, &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            ["ক্ষেত", "খেত", "খ্যাত", "খেট", "খ্যাঁত", "খেঁট", "খ্যাঁট"]
        );

        suggestion.suggest("kkhetr", &data, &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            [
                "ক্ষেত্র",
                "ক্ষেতর",
                "খেতর",
                "খ্যাতর",
                "খেটর",
                "খ্যাঁতর",
                "খেঁটর",
                "খ্যাঁটর",
            ]
        );

        /* TODO: Fix this
        suggestion.suggest("kkhetre", &data, &mut selections, &config);
        assert_eq!(
            suggestion.suggestions,
            [
                "ক্ষেত্রে",
                "ক্ষেতরে",
                "খেতরে",
                "খ্যাতরে",
                "খেটরে",
                "খ্যাঁতরে",
                "খেঁটরে",
                "খ্যাঁটরে",
            ]
        );*/

        suggestion.suggest("form", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["ফর্ম", "ফরম"]);

        suggestion.suggest("forma", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["ফরমা", "ফর্মা"]);

        suggestion.suggest("format", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত"]);

        suggestion.suggest("formate", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["ফরম্যাটে", "ফরমাতে", "ফর্মাতে"]);

        suggestion.suggest("formatt", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["ফরম্যাট", "ফরমাত্ত"]);

        suggestion.suggest("formatte", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["ফরম্যাটতে", "ফরম্যাটে", "ফরমাত্তে"]);

        suggestion.suggest("atm", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "🏧", "অ্যাটম"]);

        suggestion.suggest("atme", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["এটিএমে", "আত্মে", "অ্যাটমে"]);
        // Cache check
        suggestion.suggest("atm", &data, &mut selections, &config);
        assert_eq!(suggestion.suggestions, ["এটিএম", "আত্ম", "🏧", "অ্যাটম"]);
    }

    #[test]
    fn test_suffix() {
        let mut cache = HashMap::with_hasher(RandomState::new());
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);

        cache.insert(
            "computer".to_string(),
            vec![Rank::first_ranked("কম্পিউটার".to_string())],
        );
        cache.insert("i".to_string(), vec![Rank::first_ranked("ই".to_string())]);
        cache.insert(
            "hothat".to_string(),
            vec![Rank::first_ranked("হঠাৎ".to_string())],
        );
        cache.insert(
            "ebong".to_string(),
            vec![Rank::first_ranked("এবং".to_string())],
        );

        let mut suggestion = PhoneticSuggestion {
            cache,
            ..Default::default()
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
        assert_eq!(
            suggestion.add_suffix_to_suggestions("iei", &data),
            vec!["ইয়েই"]
        );
        // ৎ => ত
        assert_eq!(
            suggestion.add_suffix_to_suggestions("hothate", &data),
            ["হঠাতে"]
        );
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
        suggestion.suggestions = vec![
            Rank::Other("*অন্ন?!".to_string(), 0),
            Rank::Other("*অন্য?!".to_string(), 0),
        ];
        assert_eq!(
            suggestion.get_prev_selection(&SplittedString::split("*onno?!", false), &data, &mut selections),
            1
        );

        // With Suffix
        suggestion.suggestions = vec![
            Rank::Other("ইএই".to_string(), 1),
            Rank::Other("ইয়েই".to_string(), 2),
        ];
        assert_eq!(
            suggestion.get_prev_selection(&SplittedString::split("iei", false), &data, &mut selections),
            1
        );

        suggestion.suggestions = vec![
            Rank::Other("হোথাতে".to_string(), 0),
            Rank::Other("হথাতে".to_string(), 0),
            Rank::Other("হঠাতে".to_string(), 0),
        ];
        assert_eq!(
            suggestion.get_prev_selection(&SplittedString::split("hothate", false), &data, &mut selections),
            2
        );

        suggestion.suggestions = vec![
            Rank::Other("এবংমালা".to_string(), 0),
            Rank::Other("এবঙমালা".to_string(), 0),
        ];
        assert_eq!(
            suggestion.get_prev_selection(
                &SplittedString::split("ebongmala", false),
                &data,
                &mut selections
            ),
            1
        );

        // With Suffix + Avoid meta characters
        suggestion.suggestions = vec![
            Rank::Other("*অন্নগুলো?!".to_string(), 0),
            Rank::Other("*অন্যগুলো?!".to_string(), 0),
        ];
        assert_eq!(
            suggestion.get_prev_selection(
                &SplittedString::split("*onnogulo?!", false),
                &data,
                &mut selections
            ),
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

        let (suggestions, selection) =
            suggestion.suggest("sesh:`", &data, &mut selections, &config);
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

        suggestion.include_from_dictionary("a", "a", &mut suggestions, &data);
        assert_eq!(suggestions, ["অ্যা", "অ্যাঁ", "আ", "আঃ", "া", "এ"]);
        suggestions.clear();

        suggestion.include_from_dictionary("(", "", &mut suggestions, &data);
        assert_eq!(suggestions, Vec::<Rank>::new());
    }
}

#[cfg(feature = "bench")]
mod benches {
    extern crate test;

    use super::PhoneticSuggestion;
    use crate::{config::get_phonetic_method_defaults, data::Data, utility::SplittedString};
    use test::{black_box, Bencher};

    #[bench]
    fn bench_phonetic_a(b: &mut Bencher) {
        let mut suggestion = PhoneticSuggestion::default();
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        let term = SplittedString::split("a", false);

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
        let term = SplittedString::split("kkhet", false);

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
        let term = SplittedString::split("bistari", false);

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
            suggestion.include_from_dictionary("a", "", &mut suggestions, &data);
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
            suggestion.include_from_dictionary("arO", "", &mut suggestions, &data);
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
            suggestion.include_from_dictionary("bistari", "", &mut suggestions, &data);
            black_box(suggestions);
        })
    }
}
