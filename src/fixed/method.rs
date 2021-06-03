use edit_distance::edit_distance;
use serde_json::Value;

use super::{chars::*, database::Database, parser::LayoutParser};
use crate::{context::Method, keycodes::keycode_to_char};
use crate::config::{Config, get_fixed_method_defaults};
use crate::suggestion::Suggestion;
use crate::utility::{get_modifiers, split_string, Utility};

const MARKS: &str = "`~!@#$%^+*-_=+\\|\"/;:,./?><()[]{}";

pub(crate) struct FixedMethod {
    buffer: String,
    typed: String,
    suggestions: Vec<String>,
    parser: LayoutParser,
    database: Database,
}

impl Method for FixedMethod {
    fn get_suggestion(&mut self, key: u16, modifier: u8, config: &Config) -> Suggestion {
        let modifier = get_modifiers(modifier);

        if let Some(value) = self.parser.get_char_for_key(key, modifier.into(), config) {
            self.process_key_value(&value, config);
        } else {
            return self.current_suggestion(config);
        }

        // If include english typed word feature is enabled.
        if config.get_fixed_include_english() {
            self.typed.push(keycode_to_char(key));
        }

        self.create_suggestion(config)
    }

    fn candidate_committed(&mut self, _index: usize, _: &Config) {
        self.buffer.clear();
        self.typed.clear();
    }

    fn update_engine(&mut self, _: &Config) {
        //
    }

    fn ongoing_input_session(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.buffer.clear();
        self.typed.clear();
    }

    fn backspace_event(&mut self, config: &Config) -> Suggestion {
        if !self.buffer.is_empty() {
            // Remove the last character from buffer.
            self.buffer.pop();
            self.typed.pop();

            if self.buffer.is_empty() {
                // The buffer is now empty, so return empty suggestion.
                return Suggestion::empty();
            }

            return self.create_suggestion(config);
        } else {
            return Suggestion::empty();
        }
    }
}

impl FixedMethod {
    /// Creates a new instance of `FixedMethod` with the given layout.
    pub(crate) fn new(layout: &Value, config: &Config) -> Self {
        let parser = LayoutParser::new(layout);

        FixedMethod {
            buffer: String::with_capacity(20 * 3), // A Bengali character is 3 bytes in size.
            typed: String::with_capacity(20),
            suggestions: Vec::with_capacity(10),
            parser,
            database: Database::new_with_config(config),
        }
    }

    fn create_suggestion(&mut self, config: &Config) -> Suggestion {
        if config.get_fixed_suggestion() {
            self.create_dictionary_suggestion(config)
        } else {
            Suggestion::new_lonely(self.buffer.clone())
        }
    }

    fn create_dictionary_suggestion(&mut self, config: &Config) -> Suggestion {
        let (first_part, word, last_part) = split_string(&self.buffer, true);

        self.suggestions.clear();

        // Add the user's typed word.
        self.suggestions.push(word.to_string());
        // Add suggestions from the dictionary.
        let mut suggestions = self.database.search_dictionary(&word);

        // Change the Kar joinings if Traditional Kar Joining is set.
        if config.get_fixed_traditional_kar() {
            for suggestion in suggestions.iter_mut() {
                // Check if the word has any of the ligature making Kars.
                if suggestion.chars().any(is_ligature_making_kar) {
                    let mut temp = String::with_capacity(suggestion.capacity());
                    for ch in suggestion.chars() {
                        if is_ligature_making_kar(ch) {
                            temp.push(ZWNJ);
                        }
                        temp.push(ch);
                    }
                    *suggestion = temp;
                }
            }
        }

        self.suggestions.append(&mut suggestions);

        // Sort the suggestions.
        self.suggestions
            .sort_unstable_by(|a, b| edit_distance(&word, a).cmp(&edit_distance(&word, b)));

        // Remove the duplicates if present.
        self.suggestions.dedup();

        // Reduce the number of suggestions
        // and add the typed english word at the end.
        if config.get_fixed_include_english() {
            self.suggestions.truncate(8);
            self.suggestions.push(self.typed.clone());
        } else {
            self.suggestions.truncate(9);
        }

        if !first_part.is_empty() || !last_part.is_empty() {
            for suggestion in self.suggestions.iter_mut() {
                *suggestion = format!("{}{}{}", first_part, suggestion, last_part);
            }
        }

        Suggestion::new(self.buffer.clone(), self.suggestions.clone(), 0)
    }

    fn current_suggestion(&self, config: &Config) -> Suggestion {
        if !self.buffer.is_empty() {
            if config.get_fixed_suggestion() {
                Suggestion::new(self.buffer.clone(), self.suggestions.clone(), 0)
            } else {
                Suggestion::new_lonely(self.buffer.clone())
            }
        } else {
            Suggestion::empty()
        }
    }

    /// Processes the `value` of the pressed key and updates the method's
    /// internal buffer which will be used when creating suggestion.
    fn process_key_value(&mut self, value: &str, config: &Config) {
        let rmc = self.buffer.chars().last().unwrap_or_default(); // Right most character

        // Zo-fola insertion
        if value == "\u{09CD}\u{09AF}" {
            // Check if র is not a part of a Ro-fola, if its not then add an ZWJ before
            // the Zo-fola to have the র‍্য form.
            if rmc == B_R && self.buffer.chars().rev().nth(1).unwrap_or_default() != B_HASANTA {
                self.buffer.push(ZWJ);
            }
            self.buffer.push_str(value);
            return;
        }

        // Old style Reph insertion
        if value == "\u{09B0}\u{09CD}" && config.get_fixed_old_reph() {
            self.insert_old_style_reph();
            return;
        }

        if let Some(character) = value.chars().next() {
            // Kar insertion
            if character.is_kar() {
                // Automatic Vowel Forming
                if config.get_fixed_automatic_vowel()
                    && (self.buffer.is_empty() || rmc.is_vowel() || MARKS.contains(rmc))
                {
                    match character {
                        B_AA_KAR => self.buffer.push(B_AA),
                        B_I_KAR => self.buffer.push(B_I),
                        B_II_KAR => self.buffer.push(B_II),
                        B_U_KAR => self.buffer.push(B_U),
                        B_UU_KAR => self.buffer.push(B_UU),
                        B_RRI_KAR => self.buffer.push(B_RRI),
                        B_E_KAR => self.buffer.push(B_E),
                        B_OI_KAR => self.buffer.push(B_OI),
                        B_O_KAR => self.buffer.push(B_O),
                        B_OU_KAR => self.buffer.push(B_OU),
                        _ => (),
                    }
                    return;
                } else if config.get_fixed_automatic_chandra() && rmc == B_CHANDRA {
                    // Automatic Fix of Chandra Position
                    self.buffer.pop();
                    self.buffer.push(character);
                    self.buffer.push(B_CHANDRA);
                    return;
                } else if rmc == B_HASANTA {
                    // Vowel making with Hasanta + Kar
                    match character {
                        B_AA_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_AA);
                        }
                        B_I_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_I);
                        }
                        B_II_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_II);
                        }
                        B_U_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_U);
                        }
                        B_UU_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_UU);
                        }
                        B_RRI_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_RRI);
                        }
                        B_E_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_E);
                        }
                        B_OI_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_OI);
                        }
                        B_O_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_O);
                        }
                        B_OU_KAR => {
                            self.buffer.pop();
                            self.buffer.push(B_OU);
                        }
                        _ => (),
                    }
                    return;
                } else if config.get_fixed_traditional_kar() && rmc.is_pure_consonant() {
                    // Traditional Kar Joining
                    // In UNICODE it is known as "Blocking Bengali Consonant-Vowel Ligature"
                    if is_ligature_making_kar(character) {
                        self.buffer.push(ZWNJ);
                    }
                    self.buffer.push(character);
                    return;
                } else {
                    self.buffer.push(character);
                    return;
                }
            }

            // Hasanta
            if character == B_HASANTA && rmc == B_HASANTA {
                self.buffer.push(ZWNJ);
                return;
            }
        }

        self.buffer.push_str(value);
    }

    /// Checks if the Reph is moveable by the Reph insertion algorithm.  
    fn is_reph_moveable(&self) -> bool {
        let mut buf_chars = self.buffer.chars().rev();
        let right_most = buf_chars.next().unwrap();
        let right_most = if right_most == B_CHANDRA {
            buf_chars.next().unwrap_or_default()
        } else {
            right_most
        };
        let before_right_most = buf_chars.next().unwrap_or_default();

        right_most.is_pure_consonant()
            || (right_most.is_vowel() && before_right_most.is_pure_consonant())
    }

    /// Inserts Reph into the buffer in old style.
    fn insert_old_style_reph(&mut self) {
        let len = self.buffer.chars().count();
        let reph_moveable = self.is_reph_moveable();

        let mut constant = false;
        let mut vowel = false;
        let mut hasanta = false;
        let mut chandra = false;

        if reph_moveable {
            let mut step = 0;

            for (index, character) in self.buffer.chars().rev().enumerate() {
                if character.is_pure_consonant() {
                    if constant && !hasanta {
                        break;
                    }
                    constant = true;
                    hasanta = false; // reset
                    step += 1;
                    continue;
                } else if character == B_HASANTA {
                    hasanta = true;
                    step += 1;
                    continue;
                } else if character.is_vowel() {
                    if vowel {
                        break;
                    }

                    if index == 0 || chandra {
                        vowel = true;
                        step += 1;
                        continue;
                    }

                    break;
                } else if character == B_CHANDRA {
                    if index == 0 {
                        chandra = true;
                        step += 1;
                        continue;
                    }
                    break;
                }
            }

            let temp: String = self.buffer.chars().skip(len - step).collect();
            self.internal_backspace_step(step);
            self.buffer.push(B_R);
            self.buffer.push(B_HASANTA);
            self.buffer.push_str(&temp);
        } else {
            self.buffer.push(B_R);
            self.buffer.push(B_HASANTA);
        }
    }

    /// Removes the last `n` character from the buffer.
    fn internal_backspace_step(&mut self, n: usize) {
        let len = self
            .buffer
            .chars()
            .rev()
            .take(n)
            .fold(0, |acc, x| acc + x.len_utf8());
        let new_len = self.buffer.len() - len;
        self.buffer.truncate(new_len);
    }
}

// Implement Default trait on FixedMethod for testing convenience.
impl Default for FixedMethod {
    fn default() -> Self {
        let config = get_fixed_method_defaults();
        let layout = config.get_layout().unwrap();
        let parser = LayoutParser::new(&layout);

        FixedMethod {
            buffer: String::new(),
            typed: String::new(),
            suggestions: Vec::new(),
            parser,
            database: Database::new_with_config(&config),
        }
    }
}

/// Is the provided `c` is a ligature making Kar?
fn is_ligature_making_kar(c: char) -> bool {
    c == B_U_KAR || c == B_UU_KAR || c == B_RRI_KAR
}

#[cfg(test)]
mod tests {
    use super::FixedMethod;
    use crate::{context::Method, keycodes::{VC_A, VC_M, VC_I}};
    use crate::fixed::chars::*;
    use crate::config::get_fixed_method_defaults;

    #[test]
    fn test_suggestions() {
        let mut method = FixedMethod::default();
        let config = get_fixed_method_defaults();

        method.buffer = "[".to_string();
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["["]
        );

        method.buffer = "[আমি]".to_string();
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["[আমি]", "[আমিন]", "[আমির]", "[আমিষ]"]
        );

        method.buffer = "আমি:".to_string();
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["আমি:", "আমিন:", "আমির:", "আমিষ:"]
        );

        method.buffer = "আমি।".to_string();
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["আমি।", "আমিন।", "আমির।", "আমিষ।"]
        );

        // User written word should be the first one.
        method.buffer = "কম্পিউ".to_string();
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["কম্পিউ", "কম্পিউটার", "কম্পিউটিং", "কম্পিউটেশন", "কম্পিউটার্স"]
        );
    }

    #[test]
    fn test_suggestions_with_english_word() {
        let mut method = FixedMethod::default();
        let mut config = get_fixed_method_defaults();
        config.set_fixed_include_english(true);

        method.get_suggestion(VC_A, 0, &config);
        method.get_suggestion(VC_M, 0, &config);
        method.get_suggestion(VC_I, 0, &config);

        assert_eq!(method.typed, "ami");
        assert_eq!(method.current_suggestion(&config).get_suggestions(), &["আমি", "আমিন", "আমির", "আমিষ", "ami"]);
    }

    #[test]
    fn test_backspace() {
        let mut method = FixedMethod {
            buffer: "আমি".to_string(),
            typed: "ami".to_string(),
            ..Default::default()
        };

        let mut config = get_fixed_method_defaults();
        config.set_fixed_suggestion(false);

        assert!(!method.backspace_event(&config).is_empty()); // আম
        assert!(!method.backspace_event(&config).is_empty()); // আ
        assert!(method.backspace_event(&config).is_empty()); // Empty
        assert!(method.buffer.is_empty());
        assert!(method.typed.is_empty());
    }

    #[test]
    fn test_reph_insertion() {
        let mut method = FixedMethod::default();

        method.buffer = "অক".to_string();
        method.insert_old_style_reph();
        assert_eq!(method.buffer, "অর্ক".to_string());

        method.buffer = "ক".to_string();
        method.insert_old_style_reph();
        assert_eq!(method.buffer, "র্ক".to_string());

        method.buffer = "কত".to_string();
        method.insert_old_style_reph();
        assert_eq!(method.buffer, "কর্ত".to_string());

        method.buffer = "অক্কা".to_string();
        method.insert_old_style_reph();
        assert_eq!(method.buffer, "অর্ক্কা".to_string());

        method.buffer = "কক্ষ্ম".to_string();
        method.insert_old_style_reph();
        assert_eq!(method.buffer, "কর্ক্ষ্ম".to_string());

        method.buffer = "কব্যা".to_string();
        method.insert_old_style_reph();
        assert_eq!(method.buffer, "কর্ব্যা".to_string());

        method.buffer = "কব্যাঁ".to_string();
        method.insert_old_style_reph();
        assert_eq!(method.buffer, "কর্ব্যাঁ".to_string());
    }

    #[test]
    fn test_features() {
        let mut method = FixedMethod::default();
        let mut config = get_fixed_method_defaults();

        // Automatic Vowel Forming
        method.buffer = "".to_string();
        method.process_key_value(&B_AA_KAR.to_string(), &config);
        assert_eq!(method.buffer, B_AA.to_string());

        method.buffer = "আ".to_string();
        method.process_key_value(&B_I_KAR.to_string(), &config);
        assert_eq!(method.buffer, "আই".to_string());

        // Automatic Chandra position
        method.buffer = "কঁ".to_string();
        method.process_key_value(&B_AA_KAR.to_string(), &config);
        assert_eq!(method.buffer, "কাঁ".to_string());

        // Traditional Kar joining
        method.buffer = "র".to_string();
        method.process_key_value(&B_U_KAR.to_string(), &config);
        assert_eq!(method.buffer, "র‌ু".to_string());

        // Without Traditional Kar joining
        config.set_fixed_traditional_kar(false);

        method.buffer = "র".to_string();
        method.process_key_value(&B_U_KAR.to_string(), &config);
        assert_eq!(method.buffer, "রু".to_string());

        // Vowel making with Hasanta
        method.buffer = "্".to_string();
        method.process_key_value(&B_U_KAR.to_string(), &config);
        assert_eq!(method.buffer, "উ".to_string());

        // Double Hasanta for Hasanta + ZWNJ
        method.buffer = B_HASANTA.to_string();
        method.process_key_value(&B_HASANTA.to_string(), &config);
        assert_eq!(method.buffer, "\u{09CD}\u{200C}".to_string());

        // Others
        method.buffer = "ক".to_string();
        method.process_key_value(&B_KH.to_string(), &config);
        assert_eq!(method.buffer, "কখ".to_string());

        method.buffer = "ক".to_string();
        method.process_key_value(&B_AA_KAR.to_string(), &config);
        assert_eq!(method.buffer, "কা".to_string());
    }

    #[test]
    fn test_z_zofola() {
        let mut method = FixedMethod::default();
        let mut config = get_fixed_method_defaults();
        config.set_fixed_suggestion(false);

        method.buffer = "র্".to_string();
        method.process_key_value("য", &config);
        assert_eq!(method.buffer, "র্য");

        method.buffer = "র".to_string();
        method.process_key_value("্য", &config);
        assert_eq!(method.buffer, "র‍্য");

        // When the last characters constitute the Ro-fola
        method.buffer = "ক্র".to_string();
        method.process_key_value("্য", &config);
        assert_eq!(method.buffer, "ক্র্য");

        method.buffer = "খ্".to_string();
        method.process_key_value("য", &config);
        assert_eq!(method.buffer, "খ্য");

        method.buffer = "খ".to_string();
        method.process_key_value("্য", &config);
        assert_eq!(method.buffer, "খ্য");
    }

    #[test]
    fn test_suggestion_traditional_kar() {
        let mut method = FixedMethod::default();
        let mut config = get_fixed_method_defaults();

        /* With Traditional Kar Joining */
        method.process_key_value("হ", &config);
        method.process_key_value("ৃ", &config);
        method.process_key_value("দ", &config);
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["হ‌ৃদ", "হ‌ৃদি", "হ‌ৃদয়"]
        );
        method.buffer.clear();

        method.process_key_value("হ", &config);
        method.process_key_value("ু", &config);
        method.process_key_value("ল", &config);
        method.process_key_value("া", &config);
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["হ‌ুলা", "হ‌ুলানো", "হ‌ুলাহ‌ুলি"]
        );
        method.buffer.clear();

        method.process_key_value("র", &config);
        method.process_key_value("ূ", &config);
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["র‌ূ", "র‌ূপ", "র‌ূহ"]
        );
        method.buffer.clear();

        /* Without Traditional Kar Joining */
        config.set_fixed_traditional_kar(false);

        method.process_key_value("হ", &config);
        method.process_key_value("ৃ", &config);
        method.process_key_value("দ", &config);
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["হৃদ", "হৃদি", "হৃদয়"]
        );
        method.buffer.clear();

        method.process_key_value("হ", &config);
        method.process_key_value("ু", &config);
        method.process_key_value("ল", &config);
        method.process_key_value("া", &config);
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["হুলা", "হুলানো", "হুলাহুলি"]
        );
        method.buffer.clear();

        method.process_key_value("র", &config);
        method.process_key_value("ূ", &config);
        assert_eq!(
            method.create_dictionary_suggestion(&config).get_suggestions(),
            ["রূ", "রূপ", "রূহ"]
        );
        method.buffer.clear();
    }
}
