// Phonetic Method
use rustc_hash::FxHashMap;
use std::fs::{read_to_string, write};

use crate::context::Method;
use crate::keycodes::*;
use crate::phonetic::suggestion::PhoneticSuggestion;
use crate::settings::{
    get_settings_enter_closes_preview_window, get_settings_phonetic_database_on,
    get_settings_preview_window_horizontal, get_settings_user_phonetic_selection_data,
};
use crate::suggestion::Suggestion;
use crate::utility::{split_string, get_modifiers};

pub(crate) struct PhoneticMethod {
    buffer: String,
    // Was the key handled?
    handled: bool,
    suggestion: PhoneticSuggestion,
    selection_changed: bool,
    // Candidate selections.
    selections: FxHashMap<String, String>,
    // Previously selected candidate index of the current suggestion list.
    prev_selection: usize,
}

impl PhoneticMethod {
    /// Creates a new `PhoneticMethod` struct.
    pub(crate) fn new(layout: &serde_json::Value) -> Self {
        let selections =
            if let Ok(file) = read_to_string(get_settings_user_phonetic_selection_data()) {
                serde_json::from_str(&file).unwrap()
            } else {
                FxHashMap::default()
            };

        PhoneticMethod {
            buffer: String::new(),
            handled: false,
            suggestion: PhoneticSuggestion::new(layout),
            selection_changed: false,
            selections,
            prev_selection: 0,
        }
    }

    /// Returns `Suggestion` struct with suggestions.
    fn create_suggestion(&mut self) -> Suggestion {
        if get_settings_phonetic_database_on() {
            let suggestions = self.suggestion.suggestion_with_dict(&self.buffer);
            self.prev_selection = self.suggestion.get_prev_selection(&self.buffer, &mut self.selections);

            Suggestion::new(self.buffer.clone(), suggestions, self.prev_selection)
        } else {
            let suggestion = self.suggestion.suggestion_only_phonetic(&self.buffer);

            Suggestion::new_lonely(suggestion)
        }
    }

    fn current_suggestion(&self) -> Suggestion {
        if !self.buffer.is_empty() {
            if get_settings_phonetic_database_on() {
                Suggestion::new(
                    self.buffer.clone(),
                    self.suggestion.suggestions.clone(),
                    self.prev_selection,
                )
            } else {
                Suggestion::new_lonely(self.suggestion.suggestion_only_phonetic(&self.buffer))
            }
        } else {
            Suggestion::empty()
        }
    }
}

impl Method for PhoneticMethod {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion {
        let (shift, _) = get_modifiers(modifier);

        match key {
            // Alphanumeric keys
            VC_GRAVE => {
                self.buffer.push('`');
                self.handled = true;
            }
            VC_TILDE => {
                self.buffer.push('~');
                self.handled = true;
            }
            VC_0 => {
                self.buffer.push('0');
                self.handled = true;
            }
            VC_PAREN_RIGHT => {
                self.buffer.push(')');
                self.handled = true;
            }
            VC_1 => {
                self.buffer.push('1');
                self.handled = true;
            }
            VC_EXCLAIM => {
                self.buffer.push('!');
                self.handled = true;
            }
            VC_2 => {
                self.buffer.push('2');
                self.handled = true;
            }
            VC_AT => {
                self.buffer.push('@');
                self.handled = true;
            }
            VC_3 => {
                self.buffer.push('3');
                self.handled = true;
            }
            VC_HASH => {
                self.buffer.push('#');
                self.handled = true;
            }
            VC_4 => {
                self.buffer.push('4');
                self.handled = true;
            }
            VC_DOLLAR => {
                self.buffer.push('$');
                self.handled = true;
            }
            VC_5 => {
                self.buffer.push('5');
                self.handled = true;
            }
            VC_PERCENT => {
                self.buffer.push('%');
                self.handled = true;
            }
            VC_6 => {
                self.buffer.push('6');
                self.handled = true;
            }
            VC_CIRCUM => {
                self.buffer.push('^');
                self.handled = true;
            }
            VC_7 => {
                self.buffer.push('7');
                self.handled = true;
            }
            VC_AMPERSAND => {
                self.buffer.push('&');
                self.handled = true;
            }
            VC_8 => {
                self.buffer.push('8');
                self.handled = true;
            }
            VC_ASTERISK => {
                self.buffer.push('*');
                self.handled = true;
            }
            VC_9 => {
                self.buffer.push('9');
                self.handled = true;
            }
            VC_PAREN_LEFT => {
                self.buffer.push('(');
                self.handled = true;
            }

            // Alphabet Keys
            VC_Q_SHIFT => {
                self.buffer.push('Q');
                self.handled = true;
            }
            VC_Q => {
                self.buffer.push('q');
                self.handled = true;
            }
            VC_W_SHIFT => {
                self.buffer.push('W');
                self.handled = true;
            }
            VC_W => {
                self.buffer.push('w');
                self.handled = true;
            }
            VC_E_SHIFT => {
                self.buffer.push('E');
                self.handled = true;
            }
            VC_E => {
                self.buffer.push('e');
                self.handled = true;
            }
            VC_R_SHIFT => {
                self.buffer.push('R');
                self.handled = true;
            }
            VC_R => {
                self.buffer.push('r');
                self.handled = true;
            }
            VC_T_SHIFT => {
                self.buffer.push('T');
                self.handled = true;
            }
            VC_T => {
                self.buffer.push('t');
                self.handled = true;
            }
            VC_Y_SHIFT => {
                self.buffer.push('Y');
                self.handled = true;
            }
            VC_Y => {
                self.buffer.push('y');
                self.handled = true;
            }
            VC_U_SHIFT => {
                self.buffer.push('U');
                self.handled = true;
            }
            VC_U => {
                self.buffer.push('u');
                self.handled = true;
            }
            VC_I_SHIFT => {
                self.buffer.push('I');
                self.handled = true;
            }
            VC_I => {
                self.buffer.push('i');
                self.handled = true;
            }
            VC_O_SHIFT => {
                self.buffer.push('O');
                self.handled = true;
            }
            VC_O => {
                self.buffer.push('o');
                self.handled = true;
            }
            VC_P_SHIFT => {
                self.buffer.push('P');
                self.handled = true;
            }
            VC_P => {
                self.buffer.push('p');
                self.handled = true;
            }
            VC_A_SHIFT => {
                self.buffer.push('A');
                self.handled = true;
            }
            VC_A => {
                self.buffer.push('a');
                self.handled = true;
            }
            VC_S_SHIFT => {
                self.buffer.push('S');
                self.handled = true;
            }
            VC_S => {
                self.buffer.push('s');
                self.handled = true;
            }
            VC_D_SHIFT => {
                self.buffer.push('D');
                self.handled = true;
            }
            VC_D => {
                self.buffer.push('d');
                self.handled = true;
            }
            VC_F_SHIFT => {
                self.buffer.push('F');
                self.handled = true;
            }
            VC_F => {
                self.buffer.push('f');
                self.handled = true;
            }
            VC_G_SHIFT => {
                self.buffer.push('G');
                self.handled = true;
            }
            VC_G => {
                self.buffer.push('g');
                self.handled = true;
            }
            VC_H_SHIFT => {
                self.buffer.push('H');
                self.handled = true;
            }
            VC_H => {
                self.buffer.push('h');
                self.handled = true;
            }
            VC_J_SHIFT => {
                self.buffer.push('J');
                self.handled = true;
            }
            VC_J => {
                self.buffer.push('j');
                self.handled = true;
            }
            VC_K_SHIFT => {
                self.buffer.push('K');
                self.handled = true;
            }
            VC_K => {
                self.buffer.push('k');
                self.handled = true;
            }
            VC_L_SHIFT => {
                self.buffer.push('L');
                self.handled = true;
            }
            VC_L => {
                self.buffer.push('l');
                self.handled = true;
            }
            VC_Z_SHIFT => {
                self.buffer.push('Z');
                self.handled = true;
            }
            VC_Z => {
                self.buffer.push('z');
                self.handled = true;
            }
            VC_X_SHIFT => {
                self.buffer.push('X');
                self.handled = true;
            }
            VC_X => {
                self.buffer.push('x');
                self.handled = true;
            }
            VC_C_SHIFT => {
                self.buffer.push('C');
                self.handled = true;
            }
            VC_C => {
                self.buffer.push('c');
                self.handled = true;
            }
            VC_V_SHIFT => {
                self.buffer.push('V');
                self.handled = true;
            }
            VC_V => {
                self.buffer.push('v');
                self.handled = true;
            }
            VC_B_SHIFT => {
                self.buffer.push('B');
                self.handled = true;
            }
            VC_B => {
                self.buffer.push('b');
                self.handled = true;
            }
            VC_N_SHIFT => {
                self.buffer.push('N');
                self.handled = true;
            }
            VC_N => {
                self.buffer.push('n');
                self.handled = true;
            }
            VC_M_SHIFT => {
                self.buffer.push('M');
                self.handled = true;
            }
            VC_M => {
                self.buffer.push('m');
                self.handled = true;
            }

            VC_MINUS => {
                self.buffer.push('-');
                self.handled = true;
            }
            VC_UNDERSCORE => {
                self.buffer.push('_');
                self.handled = true;
            }
            VC_EQUALS => {
                self.buffer.push('=');
                self.handled = true;
            }
            VC_PLUS => {
                self.buffer.push('+');
                self.handled = true;
            }

            VC_BRACKET_LEFT => {
                self.buffer.push('[');
                self.handled = true;
            }
            VC_BRACKET_RIGHT => {
                self.buffer.push(']');
                self.handled = true;
            }
            VC_BRACE_LEFT => {
                self.buffer.push('{');
                self.handled = true;
            }
            VC_BRACE_RIGHT => {
                self.buffer.push('}');
                self.handled = true;
            }
            VC_BACK_SLASH => {
                self.buffer.push('\\');
                self.handled = true;
            }
            VC_BAR => {
                self.buffer.push('|');
                self.handled = true;
            }

            VC_SEMICOLON => {
                self.buffer.push(';');
                self.handled = true;
            }
            VC_COLON => {
                self.buffer.push(':');
                self.handled = true;
            }
            VC_APOSTROPHE => {
                self.buffer.push('\'');
                self.handled = true;
            }
            VC_QUOTE => {
                self.buffer.push('\"');
                self.handled = true;
            }

            VC_COMMA => {
                self.buffer.push(',');
                self.handled = true;
            }
            VC_LESS => {
                self.buffer.push('<');
                self.handled = true;
            }
            VC_PERIOD => {
                self.buffer.push('.');
                self.handled = true;
            }
            VC_GREATER => {
                self.buffer.push('>');
                self.handled = true;
            }
            VC_SLASH => {
                self.buffer.push('/');
                self.handled = true;
            }
            VC_QUESTION => {
                self.buffer.push('?');
                self.handled = true;
            }

            // Keypad keys
            VC_KP_0 => {
                self.buffer.push('0');
                self.handled = true;
            }
            VC_KP_1 => {
                self.buffer.push('1');
                self.handled = true;
            }
            VC_KP_2 => {
                self.buffer.push('2');
                self.handled = true;
            }
            VC_KP_3 => {
                self.buffer.push('3');
                self.handled = true;
            }
            VC_KP_4 => {
                self.buffer.push('4');
                self.handled = true;
            }
            VC_KP_5 => {
                self.buffer.push('5');
                self.handled = true;
            }
            VC_KP_6 => {
                self.buffer.push('6');
                self.handled = true;
            }
            VC_KP_7 => {
                self.buffer.push('7');
                self.handled = true;
            }
            VC_KP_8 => {
                self.buffer.push('8');
                self.handled = true;
            }
            VC_KP_9 => {
                self.buffer.push('9');
                self.handled = true;
            }

            VC_KP_DIVIDE => {
                self.buffer.push('/');
                self.handled = true;
            }
            VC_KP_MULTIPLY => {
                self.buffer.push('*');
                self.handled = true;
            }
            VC_KP_SUBTRACT => {
                self.buffer.push('-');
                self.handled = true;
            }
            VC_KP_ADD => {
                self.buffer.push('+');
                self.handled = true;
            }
            VC_KP_DECIMAL => {
                self.buffer.push('.');
                self.handled = true;
            }

            // Special Key
            (VC_SHIFT, _) | (VC_CONTROL, _) => {
                if !self.buffer.is_empty() {
                    self.handled = true;
                } else {
                    self.handled = false;
                }

                return self.current_suggestion();
            }
            VC_ENTER | VC_SPACE => {
                if key == VC_ENTER
                    && get_settings_enter_closes_preview_window()
                    && get_settings_phonetic_database_on()
                    && !self.buffer.is_empty()
                {
                    self.handled = true;
                } else {
                    self.handled = false;
                }

                let suggestion = self.current_suggestion();
                self.buffer.clear();

                return suggestion;
            }

            VC_RIGHT | VC_LEFT => {
                if !self.buffer.is_empty()
                    && get_settings_preview_window_horizontal()
                    && get_settings_phonetic_database_on()
                {
                    self.selection_changed = true;
                    self.handled = true;
                } else {
                    self.handled = false;
                }

                return self.current_suggestion();
            }

            VC_UP | VC_DOWN => {
                if !self.buffer.is_empty()
                    && !get_settings_preview_window_horizontal()
                    && get_settings_phonetic_database_on()
                {
                    self.selection_changed = true;
                    self.handled = true;
                } else {
                    self.handled = false;
                }

                return self.current_suggestion();
            }

            VC_TAB => {
                if !self.buffer.is_empty() && get_settings_phonetic_database_on() {
                    self.handled = true;
                    self.selection_changed = true;
                } else {
                    self.handled = false;
                }

                return self.current_suggestion();
            }

            _ => {
                self.handled = false;
                let suggestion = self.current_suggestion();
                self.buffer.clear();
                return suggestion;
            }
        }

        self.create_suggestion()
    }

    fn candidate_committed(&mut self, index: usize) {
        // Check if user has selected a different suggestion
        if self.selection_changed && get_settings_phonetic_database_on() {
            let suggestion = self.suggestion.suggestions[index].clone();
            self.selections
                .insert(split_string(&self.buffer).1.to_string(), suggestion);
            write(
                get_settings_user_phonetic_selection_data(),
                serde_json::to_string(&self.selections).unwrap(),
            )
            .unwrap();
        }

        // Reset to defaults
        self.buffer.clear();
        self.selection_changed = false;
        self.handled = false;
    }

    fn key_handled(&self) -> bool {
        self.handled
    }

    fn update_engine(&mut self) {
        self.suggestion.database.update();
    }

    fn ongoing_input_session(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.buffer.clear();
    }

    fn backspace_event(&mut self) -> bool {
        if !self.buffer.is_empty() {
            // Remove the last character.
            self.buffer = self.buffer[0..self.buffer.len() - 1].to_string();

            true
        } else {
            false
        }
    }
}

// Implement Default trait on PhoneticMethod for testing convenience.
impl Default for PhoneticMethod {
    fn default() -> Self {
        let loader = crate::loader::LayoutLoader::load_from_settings();

        PhoneticMethod {
            buffer: String::new(),
            handled: false,
            suggestion: PhoneticSuggestion::new(loader.layout()),
            selection_changed: false,
            selections: FxHashMap::default(),
            prev_selection: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneticMethod;
    use crate::context::Method;
    use crate::settings::tests::set_default_phonetic;

    #[test]
    fn test_backspace() {
        set_default_phonetic();

        let mut method = PhoneticMethod {
            buffer: "ab".to_string(),
            ..Default::default()
        };

        assert!(method.backspace_event()); // a
        assert!(method.backspace_event()); // " "
        assert!(!method.backspace_event()); // Empty
    }
}
