// Phonetic Method
use hashbrown::HashMap;
use std::fs::{read_to_string, write};

use crate::context::Method;
use crate::keycodes::*;
use crate::phonetic::suggestion::PhoneticSuggestion;
use crate::settings::{
    get_settings_phonetic_database_on, get_settings_user_phonetic_selection_data,
};
use crate::suggestion::Suggestion;
use crate::utility::split_string;

pub(crate) struct PhoneticMethod {
    buffer: String,
    suggestion: PhoneticSuggestion,
    // Candidate selections.
    selections: HashMap<String, String>,
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
                HashMap::new()
            };

        PhoneticMethod {
            buffer: String::with_capacity(20),
            suggestion: PhoneticSuggestion::new(layout),
            selections,
            prev_selection: 0,
        }
    }

    /// Returns `Suggestion` struct with suggestions.
    fn create_suggestion(&mut self) -> Suggestion {
        if get_settings_phonetic_database_on() {
            let suggestions = self.suggestion.suggestion_with_dict(&self.buffer);
            self.prev_selection = self
                .suggestion
                .get_prev_selection(&self.buffer, &mut self.selections);

            Suggestion::new(self.buffer.clone(), suggestions, self.prev_selection)
        } else {
            let suggestion = self.suggestion.suggestion_only_phonetic(&self.buffer);

            Suggestion::new_lonely(suggestion)
        }
    }
}

impl Method for PhoneticMethod {
    fn get_suggestion(&mut self, key: u16, _modifier: u8) -> Suggestion {
        match key {
            // Alphanumeric keys
            VC_GRAVE => self.buffer.push('`'),
            VC_TILDE => self.buffer.push('~'),
            VC_0 => self.buffer.push('0'),
            VC_PAREN_RIGHT => self.buffer.push(')'),
            VC_1 => self.buffer.push('1'),
            VC_EXCLAIM => self.buffer.push('!'),
            VC_2 => self.buffer.push('2'),
            VC_AT => self.buffer.push('@'),
            VC_3 => self.buffer.push('3'),
            VC_HASH => self.buffer.push('#'),
            VC_4 => self.buffer.push('4'),
            VC_DOLLAR => self.buffer.push('$'),
            VC_5 => self.buffer.push('5'),
            VC_PERCENT => self.buffer.push('%'),
            VC_6 => self.buffer.push('6'),
            VC_CIRCUM => self.buffer.push('^'),
            VC_7 => self.buffer.push('7'),
            VC_AMPERSAND => self.buffer.push('&'),
            VC_8 => self.buffer.push('8'),
            VC_ASTERISK => self.buffer.push('*'),
            VC_9 => self.buffer.push('9'),
            VC_PAREN_LEFT => self.buffer.push('('),

            // Alphabet Keys
            VC_Q_SHIFT => self.buffer.push('Q'),
            VC_Q => self.buffer.push('q'),
            VC_W_SHIFT => self.buffer.push('W'),
            VC_W => self.buffer.push('w'),
            VC_E_SHIFT => self.buffer.push('E'),
            VC_E => self.buffer.push('e'),
            VC_R_SHIFT => self.buffer.push('R'),
            VC_R => self.buffer.push('r'),
            VC_T_SHIFT => self.buffer.push('T'),
            VC_T => self.buffer.push('t'),
            VC_Y_SHIFT => self.buffer.push('Y'),
            VC_Y => self.buffer.push('y'),
            VC_U_SHIFT => self.buffer.push('U'),
            VC_U => self.buffer.push('u'),
            VC_I_SHIFT => self.buffer.push('I'),
            VC_I => self.buffer.push('i'),
            VC_O_SHIFT => self.buffer.push('O'),
            VC_O => self.buffer.push('o'),
            VC_P_SHIFT => self.buffer.push('P'),
            VC_P => self.buffer.push('p'),
            VC_A_SHIFT => self.buffer.push('A'),
            VC_A => self.buffer.push('a'),
            VC_S_SHIFT => self.buffer.push('S'),
            VC_S => self.buffer.push('s'),
            VC_D_SHIFT => self.buffer.push('D'),
            VC_D => self.buffer.push('d'),
            VC_F_SHIFT => self.buffer.push('F'),
            VC_F => self.buffer.push('f'),
            VC_G_SHIFT => self.buffer.push('G'),
            VC_G => self.buffer.push('g'),
            VC_H_SHIFT => self.buffer.push('H'),
            VC_H => self.buffer.push('h'),
            VC_J_SHIFT => self.buffer.push('J'),
            VC_J => self.buffer.push('j'),
            VC_K_SHIFT => self.buffer.push('K'),
            VC_K => self.buffer.push('k'),
            VC_L_SHIFT => self.buffer.push('L'),
            VC_L => self.buffer.push('l'),
            VC_Z_SHIFT => self.buffer.push('Z'),
            VC_Z => self.buffer.push('z'),
            VC_X_SHIFT => self.buffer.push('X'),
            VC_X => self.buffer.push('x'),
            VC_C_SHIFT => self.buffer.push('C'),
            VC_C => self.buffer.push('c'),
            VC_V_SHIFT => self.buffer.push('V'),
            VC_V => self.buffer.push('v'),
            VC_B_SHIFT => self.buffer.push('B'),
            VC_B => self.buffer.push('b'),
            VC_N_SHIFT => self.buffer.push('N'),
            VC_N => self.buffer.push('n'),
            VC_M_SHIFT => self.buffer.push('M'),
            VC_M => self.buffer.push('m'),

            VC_MINUS => self.buffer.push('-'),
            VC_UNDERSCORE => self.buffer.push('_'),
            VC_EQUALS => self.buffer.push('='),
            VC_PLUS => self.buffer.push('+'),

            VC_BRACKET_LEFT => self.buffer.push('['),
            VC_BRACKET_RIGHT => self.buffer.push(']'),
            VC_BRACE_LEFT => self.buffer.push('{'),
            VC_BRACE_RIGHT => self.buffer.push('}'),
            VC_BACK_SLASH => self.buffer.push('\\'),
            VC_BAR => self.buffer.push('|'),

            VC_SEMICOLON => self.buffer.push(';'),
            VC_COLON => self.buffer.push(':'),
            VC_APOSTROPHE => self.buffer.push('\''),
            VC_QUOTE => self.buffer.push('\"'),

            VC_COMMA => self.buffer.push(','),
            VC_LESS => self.buffer.push('<'),
            VC_PERIOD => self.buffer.push('.'),
            VC_GREATER => self.buffer.push('>'),
            VC_SLASH => self.buffer.push('/'),
            VC_QUESTION => self.buffer.push('?'),

            // Keypad keys
            VC_KP_0 => self.buffer.push('0'),
            VC_KP_1 => self.buffer.push('1'),
            VC_KP_2 => self.buffer.push('2'),
            VC_KP_3 => self.buffer.push('3'),
            VC_KP_4 => self.buffer.push('4'),
            VC_KP_5 => self.buffer.push('5'),
            VC_KP_6 => self.buffer.push('6'),
            VC_KP_7 => self.buffer.push('7'),
            VC_KP_8 => self.buffer.push('8'),
            VC_KP_9 => self.buffer.push('9'),

            VC_KP_DIVIDE => self.buffer.push('/'),
            VC_KP_MULTIPLY => self.buffer.push('*'),
            VC_KP_SUBTRACT => self.buffer.push('-'),
            VC_KP_ADD => self.buffer.push('+'),
            VC_KP_DECIMAL => self.buffer.push('.'),

            _ => panic!("Got unknown key!"),
        }

        self.create_suggestion()
    }

    fn candidate_committed(&mut self, index: usize) {
        // Check if user has selected a different suggestion
        if self.prev_selection != index && get_settings_phonetic_database_on() {
            let suggestion = split_string(&self.suggestion.suggestions[index]).1.to_string();
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

    fn backspace_event(&mut self) -> Suggestion {
        if !self.buffer.is_empty() {
            // Remove the last character.
            self.buffer.pop();

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

// Implement Default trait on PhoneticMethod for testing convenience.
impl Default for PhoneticMethod {
    fn default() -> Self {
        let loader = crate::loader::LayoutLoader::load_from_settings();

        PhoneticMethod {
            buffer: String::new(),
            suggestion: PhoneticSuggestion::new(loader.layout()),
            selections: HashMap::new(),
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

        assert!(!method.backspace_event().is_empty()); // a
        assert!(method.backspace_event().is_empty()); // Empty
    }
}
