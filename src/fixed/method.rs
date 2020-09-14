use edit_distance::edit_distance;
use serde_json::Value;

use super::{chars::*, database::Database, parser::LayoutParser};
use crate::context::Method;
use crate::loader::LayoutLoader;
use crate::settings::*;
use crate::suggestion::Suggestion;
use crate::utility::{get_modifiers, split_string, Utility};

const MARKS: &str = "`~!@#$%^+*-_=+\\|\"/;:,./?><()[]{}";

pub(crate) struct FixedMethod {
    buffer: String,
    suggestions: Vec<String>,
    parser: LayoutParser,
    database: Database,
}

impl Method for FixedMethod {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion {
        let modifier = get_modifiers(modifier);

        if let Some(value) = self.parser.get_char_for_key(key, modifier.into()) {
            self.process_key_value(&value);
        } else {
            return self.current_suggestion();
        }

        self.create_suggestion()
    }

    fn candidate_committed(&mut self, _index: usize) {
        self.buffer.clear();
    }

    fn update_engine(&mut self) {
        //
    }

    fn ongoing_input_session(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.buffer.clear();
    }

    fn backspace_event(&mut self) -> Suggestion {
        if !self.buffer.is_empty() {
            // Remove the last character from buffer.
            self.internal_backspace();

            if self.buffer.is_empty() {
                // The buffer is now empty, so return empty suggestion.
                return Suggestion::empty();
            }

            return self.create_suggestion();
        } else {
            return Suggestion::empty();
        }
    }
}

impl FixedMethod {
    /// Creates a new instance of `FixedMethod` with the given layout.
    pub(crate) fn new(layout: &Value) -> Self {
        let parser = LayoutParser::new(layout);

        FixedMethod {
            buffer: String::new(),
            suggestions: Vec::new(),
            parser,
            database: Database::new(),
        }
    }

    fn create_suggestion(&mut self) -> Suggestion {
        if get_settings_fixed_database_on() {
            self.create_dictionary_suggestion()
        } else {
            Suggestion::new_lonely(self.buffer.clone())
        }
    }

    fn create_dictionary_suggestion(&mut self) -> Suggestion {
        let (first_part, word, last_part) = split_string(&self.buffer);

        self.suggestions.clear();

        // Add the user's typed word.
        self.suggestions.push(word.to_string());
        // Add suggestions from the dictionary.
        self.suggestions
            .append(&mut self.database.search_dictionary(&word));

        // Sort the suggestions.
        self.suggestions
            .sort_unstable_by(|a, b| edit_distance(&word, a).cmp(&edit_distance(&word, b)));

        // Remove the duplicates if present.
        self.suggestions.dedup();

        // Reduce the number of suggestions.
        self.suggestions.truncate(9);

        if !first_part.is_empty() || !last_part.is_empty() {
            for suggestion in self.suggestions.iter_mut() {
                *suggestion = format!("{}{}{}", first_part, suggestion, last_part);
            }
        }

        Suggestion::new(self.buffer.clone(), self.suggestions.clone(), 0)
    }

    fn current_suggestion(&self) -> Suggestion {
        if !self.buffer.is_empty() {
            if get_settings_fixed_database_on() {
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
    fn process_key_value(&mut self, value: &str) {
        let rmc = self.buffer.chars().last().unwrap_or_default(); // Right most character

        // Zo-fola insertion
        if value == "\u{09CD}\u{09AF}" {
            // Check if র is not a part of a Ro-fola, if its not then add an ZWJ before
            // the Zo-fola to have the র‍্য form.
            if rmc == B_R && self.buffer.chars().rev().nth(1).unwrap_or_default() != B_HASANTA {
                self.buffer = format!("{}{}{}", self.buffer, ZWJ, value);
                return;
            } else {
                self.buffer.push_str(value);
                return;
            }
        }

        // Old style Reph insertion
        if value == "\u{09B0}\u{09CD}" && get_settings_fixed_old_reph() {
            self.insert_old_style_reph();
            return;
        }

        if let Some(character) = value.chars().nth(0) {
            // Kar insertion
            if character.is_kar() {
                // Automatic Vowel Forming
                if get_settings_fixed_automatic_vowel()
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
                        _ => unreachable!(),
                    }
                    return;
                } else if get_settings_fixed_automatic_chandra() && rmc == B_CHANDRA {
                    // Automatic Fix of Chandra Position
                    self.internal_backspace();
                    self.buffer = format!("{}{}{}", self.buffer, character, B_CHANDRA);
                    return;
                } else if rmc == B_HASANTA {
                    // Vowel making with Hasanta + Kar
                    match character {
                        B_AA_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_AA);
                        }
                        B_I_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_I);
                        }
                        B_II_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_II);
                        }
                        B_U_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_U);
                        }
                        B_UU_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_UU);
                        }
                        B_RRI_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_RRI);
                        }
                        B_E_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_E);
                        }
                        B_OI_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_OI);
                        }
                        B_O_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_O);
                        }
                        B_OU_KAR => {
                            self.internal_backspace();
                            self.buffer.push(B_OU);
                        }
                        _ => unreachable!(),
                    }
                    return;
                } else if get_settings_fixed_traditional_kar() && rmc.is_pure_consonant() {
                    // Traditional Kar Joining
                    // In UNICODE it is known as "Blocking Bengali Consonant-Vowel Ligature"
                    self.buffer = format!("{}{}{}", self.buffer, ZWNJ, character);
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
    ///
    /// `rmc`: Right most character.
    #[rustfmt::skip]
    fn is_reph_moveable(&self, rmc: char) -> bool {
        if rmc.is_pure_consonant() {
            return true;
        } else if rmc.is_vowel()
            && self.buffer.chars().rev().nth(1).unwrap_or_default().is_pure_consonant()
        {
            return true;
        } else if rmc == B_CHANDRA {
            if self.buffer.chars().rev().nth(1).unwrap_or_default().is_pure_consonant()
            {
                return true;
            } else if self.buffer.chars().rev().nth(1).unwrap_or_default().is_vowel()
                && self.buffer.chars().rev().nth(2).unwrap_or_default().is_pure_consonant()
            {
                return true;
            }
        }

        false
    }

    /// Inserts Reph into the buffer in old style.
    fn insert_old_style_reph(&mut self) {
        let rmc = self.buffer.chars().last().unwrap();
        let len = self.buffer.chars().count();   
        let reph_moveable = self.is_reph_moveable(rmc);

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
            self.buffer = format!("{}{}{}{}", self.buffer, B_R, B_HASANTA, temp);
        } else {
            self.buffer = format!("{}{}{}", self.buffer, B_R, B_HASANTA);
        }
    }

    /// Removes the last character from the buffer.
    fn internal_backspace(&mut self) {
        let len = self.buffer.chars().count() - 1;
        self.buffer = self.buffer.chars().take(len).collect();
    }

    /// Removes the last `n` character from the buffer.
    fn internal_backspace_step(&mut self, n: usize) {
        let len = self.buffer.chars().count() - n;
        self.buffer = self.buffer.chars().take(len).collect();
    }
}

// Implement Default trait on FixedMethod for testing convenience.
// This constructor uses the layout file specified in the
// environment variable `RITI_LAYOUT_FILE`.
impl Default for FixedMethod {
    fn default() -> Self {
        let loader = LayoutLoader::load_from_settings();
        let parser = LayoutParser::new(loader.layout());

        FixedMethod {
            buffer: String::new(),
            suggestions: Vec::new(),
            parser,
            database: Database::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::set_var;

    use super::FixedMethod;
    use crate::context::Method;
    use crate::fixed::chars::*;
    use crate::settings::{self, tests::set_defaults_fixed};

    #[test]
    fn test_suggestions() {
        set_defaults_fixed();

        let mut method = FixedMethod::default();

        method.buffer = "[".to_string();
        assert_eq!(
            method.create_dictionary_suggestion().get_suggestions(),
            ["["]
        );

        method.buffer = "[আমি]".to_string();
        assert_eq!(
            method.create_dictionary_suggestion().get_suggestions(),
            ["[আমি]", "[আমিন]", "[আমির]", "[আমিষ]"]
        );

        // User written word should be the first one.
        method.buffer = "কম্পিউ".to_string();
        assert_eq!(
            method.create_dictionary_suggestion().get_suggestions(),
            ["কম্পিউ", "কম্পিউটার", "কম্পিউটিং", "কম্পিউটেশন", "কম্পিউটার্স"]
        );
    }

    #[test]
    fn test_backspace() {
        set_defaults_fixed();
        set_var(settings::ENV_LAYOUT_FIXED_DATABASE_ON, "false");

        let mut method = FixedMethod {
            buffer: "আমি".to_string(),
            ..Default::default()
        };

        assert!(!method.backspace_event().is_empty()); // আম
        assert!(!method.backspace_event().is_empty()); // আ
        assert!(method.backspace_event().is_empty()); // Empty
    }

    #[test]
    fn test_reph_insertion() {
        set_defaults_fixed();

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
        set_defaults_fixed();

        let mut method = FixedMethod::default();

        // Automatic Vowel Forming
        method.buffer = "".to_string();
        method.process_key_value(&B_AA_KAR.to_string());
        assert_eq!(method.buffer, B_AA.to_string());

        method.buffer = "আ".to_string();
        method.process_key_value(&B_I_KAR.to_string());
        assert_eq!(method.buffer, "আই".to_string());

        // Automatic Chandra position
        method.buffer = "কঁ".to_string();
        method.process_key_value(&B_AA_KAR.to_string());
        assert_eq!(method.buffer, "কাঁ".to_string());

        // Traditional Kar joining
        method.buffer = "র".to_string();
        method.process_key_value(&B_U_KAR.to_string());
        assert_eq!(method.buffer, "র‌ু".to_string());

        // Without Traditional Kar joining
        set_var(settings::ENV_LAYOUT_FIXED_KAR, "false");
        method.buffer = "র".to_string();
        method.process_key_value(&B_U_KAR.to_string());
        assert_eq!(method.buffer, "রু".to_string());

        // Vowel making with Hasanta
        method.buffer = "্".to_string();
        method.process_key_value(&B_U_KAR.to_string());
        assert_eq!(method.buffer, "উ".to_string());

        // Double Hasanta for Hasanta + ZWNJ
        method.buffer = B_HASANTA.to_string();
        method.process_key_value(&B_HASANTA.to_string());
        assert_eq!(method.buffer, "\u{09CD}\u{200C}".to_string());

        // Others
        method.buffer = "ক".to_string();
        method.process_key_value(&B_KH.to_string());
        assert_eq!(method.buffer, "কখ".to_string());

        method.buffer = "ক".to_string();
        method.process_key_value(&B_AA_KAR.to_string());
        assert_eq!(method.buffer, "কা".to_string());
    }

    #[test]
    fn test_z_zofola() {
        set_defaults_fixed();
        set_var(settings::ENV_LAYOUT_FIXED_DATABASE_ON, "false");

        let mut method = FixedMethod::default();

        method.buffer = "র্".to_string();
        method.process_key_value("য");
        assert_eq!(method.buffer, "র্য");

        method.buffer = "র".to_string();
        method.process_key_value("্য");
        assert_eq!(method.buffer, "র‍্য");

        // When the last characters constitute the Ro-fola
        method.buffer = "ক্র".to_string();
        method.process_key_value("্য");
        assert_eq!(method.buffer, "ক্র্য");

        method.buffer = "খ্".to_string();
        method.process_key_value("য");
        assert_eq!(method.buffer, "খ্য");

        method.buffer = "খ".to_string();
        method.process_key_value("্য");
        assert_eq!(method.buffer, "খ্য");
    }
}
