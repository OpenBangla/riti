use serde_json::Value;
use std::env::var;
use std::fs::read_to_string;

use crate::context::Method;
use crate::fixed::chars::*;
use crate::fixed::parser::LayoutParser;
use crate::keycodes::*;
use crate::suggestion::Suggestion;
use crate::utility::get_modifiers;
use crate::utility::Utility;
use crate::ENV_LAYOUT;

const MARKS: &str = "`~!@#$%^+*-_=+\\|\"/;:,./?><()[]{}";

pub(crate) struct FixedMethod {
    buffer: String,
    handled: bool,
    parser: LayoutParser,
    layout: String,
}

impl Method for FixedMethod {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion {
        let modifier = get_modifiers(modifier);
        let (_, ctrl, alt) = modifier;
        // Don't catch Ctrl or Alt without AltGr combination.
        if (ctrl && !alt) || (!ctrl && alt) {
            // Handle edge cases
            if key == VC_SHIFT || key == VC_ALT {
                if !self.buffer.is_empty() {
                    self.handled = true;
                    return self.create_suggestion();
                } else {
                    self.handled = false;
                    return Suggestion::empty();
                }
            } else {
                self.handled = false;
                return Suggestion::empty();
            }
        }

        if key == VC_SHIFT || key == VC_CONTROL || key == VC_ALT {
            if !self.buffer.is_empty() {
                self.handled = true;
                return self.create_suggestion();
            } else {
                self.handled = false;
                return Suggestion::empty();
            }
        } else if key == VC_BACKSPACE {
            if !self.buffer.is_empty() {
                // Remove the last character from buffer.
                self.internal_backspace();
                self.handled = true;

                if !self.buffer.is_empty() {
                    return self.create_suggestion();
                } else {
                    // The buffer is now empty, so return empty suggestion.
                    return Suggestion::empty();
                }
            } else {
                self.handled = false;
                return Suggestion::empty();
            }
        }

        if let Some(value) = self.parser.get_char_for_key(key, modifier.into()) {
            self.process_key_value(&value);
            self.handled = true;
        } else {
            self.handled = false;
            return Suggestion::empty();
        }

        self.create_suggestion()
    }

    fn handle_special_key(&mut self, _key: u16) -> u8 {
        0
    }

    fn key_handled(&self) -> bool {
        self.handled
    }

    fn update_engine(&mut self) {
        let layout = var(ENV_LAYOUT).unwrap();

        // Check if the layout was changed.
        if self.layout != layout {
            let file = serde_json::from_str::<Value>(&read_to_string(&layout).unwrap()).unwrap();
            let parser = LayoutParser::new(file.get("layout").unwrap());
            self.parser = parser;
        }
    }
}

impl FixedMethod {
    /// Creates a new instance of `FixedMethod` with the layout which
    /// is set in the `RITI_LAYOUT_FILE` environment variable.
    pub(crate) fn new() -> Self {
        let layout = var(ENV_LAYOUT).unwrap();
        let file = serde_json::from_str::<Value>(&read_to_string(&layout).unwrap()).unwrap();
        let parser = LayoutParser::new(file.get("layout").unwrap());

        FixedMethod {
            buffer: String::new(),
            handled: false,
            parser,
            layout,
        }
    }

    fn create_suggestion(&self) -> Suggestion {
        Suggestion::new("".to_string(), vec![self.buffer.clone()])
    }

    /// Processes the `value` of the pressed key and updates the method's 
    /// internal buffer which will be used when creating suggestion.
    fn process_key_value(&mut self, character: &str) {
        let rmc = self.buffer.chars().last().unwrap(); // Right most character
        // Automatic Vowel Forming
        if character.chars().nth(0).unwrap().is_kar()
            && (self.buffer.is_empty() || rmc.is_vowel() || MARKS.contains(rmc))
        {
            match character {
                B_AA_KAR => self.buffer += B_AA,
                B_I_KAR => self.buffer += B_I,
                B_II_KAR => self.buffer += B_II,
                B_U_KAR => self.buffer += B_U,
                B_UU_KAR => self.buffer += B_UU,
                B_RRI_KAR => self.buffer += B_RRI,
                B_E_KAR => self.buffer += B_E,
                B_OI_KAR => self.buffer += B_OI,
                B_O_KAR => self.buffer += B_O,
                B_OU_KAR => self.buffer += B_OU,
                _ => unreachable!(),
            }
        }

        // Vowel making with Hasanta + Kar
        if self.buffer.chars().last().unwrap() == B_HASANTA.chars().nth(0).unwrap() {
            if character == B_AA_KAR {
                self.internal_backspace();
                self.buffer += B_AA;
            } else if character == B_I_KAR {
                self.internal_backspace();
                self.buffer += B_I;
            } else if character == B_II_KAR {
                self.internal_backspace();
                self.buffer += B_II;
            } else if character == B_U_KAR {
                self.internal_backspace();
                self.buffer += B_U;
            } else if character == B_UU_KAR {
                self.internal_backspace();
                self.buffer += B_UU;
            } else if character == B_RRI_KAR {
                self.internal_backspace();
                self.buffer += B_RRI;
            } else if character == B_E_KAR {
                self.internal_backspace();
                self.buffer += B_E;
            } else if character == B_OI_KAR {
                self.internal_backspace();
                self.buffer += B_OI;
            } else if character == B_O_KAR {
                self.internal_backspace();
                self.buffer += B_O;
            } else if character == B_OU_KAR {
                self.internal_backspace();
                self.buffer += B_OU;
            } else if character == B_HASANTA {
                self.buffer += ZWNJ;
            }
        }
    }

    /// Removes the last character from the buffer.
    fn internal_backspace(&mut self) {
        let len = self.buffer.chars().count() - 1;
        self.buffer = self.buffer.chars().take(len).collect();
    }
}

// Implement Default trait on FixedMethod for testing convenience.
impl Default for FixedMethod {
    fn default() -> Self {
        FixedMethod::new()
    }
}

#[cfg(test)]
mod tests {

    use super::FixedMethod;
    use crate::context::Method;

    use crate::keycodes::VC_BACKSPACE;
    use crate::ENV_LAYOUT;
    use std::env::set_var;
    #[test]
    fn test_backspace() {
        set_var(
            ENV_LAYOUT,
            format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/Probhat.json"),
        );
        
        let mut method = FixedMethod {
            buffer: "আমি".to_string(),
            ..Default::default()
        };

        assert!(!method.get_suggestion(VC_BACKSPACE, 0).is_empty()); // আম
        assert!(!method.get_suggestion(VC_BACKSPACE, 0).is_empty()); // আ
        assert!(method.get_suggestion(VC_BACKSPACE, 0).is_empty()); // Empty
    }
}
