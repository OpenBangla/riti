// Phonetic Method

use crate::context::{Method, IM_COMMIT, IM_DEFAULT, IM_KEY_ACCEPTED};
use crate::keycodes::*;
use crate::phonetic::suggestion::PhoneticSuggestion;
use crate::suggestion::Suggestion;
use crate::utility::get_modifiers;

pub(crate) struct PhoneticMethod {
    buffer: String,
    // Was the key handled?
    handled: bool,
    suggestion: PhoneticSuggestion,
    selection_changed: bool,
}

impl PhoneticMethod {
    /// Creates a new `PhoneticMethod` struct.
    pub(crate) fn new() -> Self {
        PhoneticMethod {
            buffer: String::new(),
            handled: false,
            suggestion: PhoneticSuggestion::new(),
            selection_changed: false,
        }
    }

    /// Returns `Suggestion` struct with suggestions.
    fn create_suggestion(&mut self) -> Suggestion {
        let suggestions = self.suggestion.suggest(&self.buffer);

        Suggestion::new(self.buffer.clone(), suggestions)
    }
}

impl Method for PhoneticMethod {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion {
        let (shift, ctrl, alt) = get_modifiers(modifier);

        // Reject key which has ctrl, alt combinations.
        if ctrl || alt {
            self.handled = false;
            return Suggestion::empty();
        }

        match (key, shift) {
            // Alphabet Keys
            (VC_Q, true) => {
                self.buffer.push('Q');
                self.handled = true;
            }
            (VC_Q, false) => {
                self.buffer.push('q');
                self.handled = true;
            }
            (VC_W, true) => {
                self.buffer.push('W');
                self.handled = true;
            }
            (VC_W, false) => {
                self.buffer.push('w');
                self.handled = true;
            }
            (VC_E, true) => {
                self.buffer.push('E');
                self.handled = true;
            }
            (VC_E, false) => {
                self.buffer.push('e');
                self.handled = true;
            }
            (VC_R, true) => {
                self.buffer.push('R');
                self.handled = true;
            }
            (VC_R, false) => {
                self.buffer.push('r');
                self.handled = true;
            }
            (VC_T, true) => {
                self.buffer.push('T');
                self.handled = true;
            }
            (VC_T, false) => {
                self.buffer.push('t');
                self.handled = true;
            }
            (VC_Y, true) => {
                self.buffer.push('Y');
                self.handled = true;
            }
            (VC_Y, false) => {
                self.buffer.push('y');
                self.handled = true;
            }
            (VC_U, true) => {
                self.buffer.push('U');
                self.handled = true;
            }
            (VC_U, false) => {
                self.buffer.push('u');
                self.handled = true;
            }
            (VC_I, true) => {
                self.buffer.push('I');
                self.handled = true;
            }
            (VC_I, false) => {
                self.buffer.push('i');
                self.handled = true;
            }
            (VC_O, true) => {
                self.buffer.push('O');
                self.handled = true;
            }
            (VC_O, false) => {
                self.buffer.push('o');
                self.handled = true;
            }
            (VC_P, true) => {
                self.buffer.push('P');
                self.handled = true;
            }
            (VC_P, false) => {
                self.buffer.push('p');
                self.handled = true;
            }
            (VC_A, true) => {
                self.buffer.push('A');
                self.handled = true;
            }
            (VC_A, false) => {
                self.buffer.push('a');
                self.handled = true;
            }
            (VC_S, true) => {
                self.buffer.push('S');
                self.handled = true;
            }
            (VC_S, false) => {
                self.buffer.push('s');
                self.handled = true;
            }
            (VC_D, true) => {
                self.buffer.push('D');
                self.handled = true;
            }
            (VC_D, false) => {
                self.buffer.push('d');
                self.handled = true;
            }
            (VC_F, true) => {
                self.buffer.push('F');
                self.handled = true;
            }
            (VC_F, false) => {
                self.buffer.push('f');
                self.handled = true;
            }
            (VC_G, true) => {
                self.buffer.push('G');
                self.handled = true;
            }
            (VC_G, false) => {
                self.buffer.push('g');
                self.handled = true;
            }
            (VC_H, true) => {
                self.buffer.push('H');
                self.handled = true;
            }
            (VC_H, false) => {
                self.buffer.push('h');
                self.handled = true;
            }
            (VC_J, true) => {
                self.buffer.push('J');
                self.handled = true;
            }
            (VC_J, false) => {
                self.buffer.push('j');
                self.handled = true;
            }
            (VC_K, true) => {
                self.buffer.push('K');
                self.handled = true;
            }
            (VC_K, false) => {
                self.buffer.push('k');
                self.handled = true;
            }
            (VC_L, true) => {
                self.buffer.push('L');
                self.handled = true;
            }
            (VC_L, false) => {
                self.buffer.push('l');
                self.handled = true;
            }
            (VC_Z, true) => {
                self.buffer.push('Z');
                self.handled = true;
            }
            (VC_Z, false) => {
                self.buffer.push('z');
                self.handled = true;
            }
            (VC_X, true) => {
                self.buffer.push('X');
                self.handled = true;
            }
            (VC_X, false) => {
                self.buffer.push('x');
                self.handled = true;
            }
            (VC_C, true) => {
                self.buffer.push('C');
                self.handled = true;
            }
            (VC_C, false) => {
                self.buffer.push('c');
                self.handled = true;
            }
            (VC_V, true) => {
                self.buffer.push('V');
                self.handled = true;
            }
            (VC_V, false) => {
                self.buffer.push('v');
                self.handled = true;
            }
            (VC_B, true) => {
                self.buffer.push('B');
                self.handled = true;
            }
            (VC_B, false) => {
                self.buffer.push('b');
                self.handled = true;
            }
            (VC_N, true) => {
                self.buffer.push('N');
                self.handled = true;
            }
            (VC_N, false) => {
                self.buffer.push('n');
                self.handled = true;
            }
            (VC_M, true) => {
                self.buffer.push('M');
                self.handled = true;
            }
            (VC_M, false) => {
                self.buffer.push('m');
                self.handled = true;
            }
            // Special Key
            (VC_BACKSPACE, _) => {
                if !self.buffer.is_empty() {
                    // Remove the last character.
                    self.buffer = self.buffer[0..self.buffer.len() - 1].to_string();
                    self.handled = true;

                    if self.buffer.is_empty() {
                        // The buffer is now empty, so return empty suggestion.
                        return Suggestion::empty();
                    }
                } else {
                    self.handled = false;
                    return Suggestion::empty();
                }
            }
            _ => panic!("Unknown key code!"),
        }

        self.create_suggestion()
    }

    fn handle_special_key(&mut self, key: u16) -> u8 {
        match key {
            VC_SPACE => {
                if !self.buffer.is_empty() {
                    self.buffer.clear();
                    IM_COMMIT
                } else {
                    IM_DEFAULT
                }
            }
            VC_TAB => {
                if !self.buffer.is_empty() {
                    self.selection_changed = true;
                    IM_KEY_ACCEPTED
                } else {
                    IM_DEFAULT
                }
            }

            _ => IM_DEFAULT,
        }
    }

    fn key_handled(&self) -> bool {
        self.handled
    }
}

// Implement Default trait on PhoneticMethod for testing convenience.
impl Default for PhoneticMethod {
    fn default() -> Self {
        PhoneticMethod::new()
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneticMethod;
    use crate::context::Method;
    use crate::keycodes::VC_BACKSPACE;

    #[test]
    fn test_backspace() {
        let mut method = PhoneticMethod {
            buffer: "ab".to_string(),
            ..Default::default()
        };

        assert!(!method.get_suggestion(VC_BACKSPACE, 0).is_empty());
        assert!(method.key_handled());

        assert!(method.get_suggestion(VC_BACKSPACE, 0).is_empty());
        assert!(method.key_handled());

        assert!(method.get_suggestion(VC_BACKSPACE, 0).is_empty());
        assert!(!method.key_handled());
    }
}
