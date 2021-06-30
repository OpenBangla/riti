// Phonetic Method
use std::collections::HashMap;
use std::fs::{read_to_string, write};
use ahash::RandomState;

use crate::context::Method;
use crate::config::{Config, get_phonetic_method_defaults};
use crate::keycodes::keycode_to_char;
use crate::phonetic::suggestion::PhoneticSuggestion;
use crate::suggestion::Suggestion;
use crate::utility::split_string;

pub(crate) struct PhoneticMethod {
    buffer: String,
    suggestion: PhoneticSuggestion,
    // Candidate selections.
    selections: HashMap<String, String, RandomState>,
    // Previously selected candidate index of the current suggestion list.
    prev_selection: usize,
}

impl PhoneticMethod {
    /// Creates a new `PhoneticMethod` struct.
    pub(crate) fn new(config: &Config) -> Self {
        let selections =
            if let Ok(file) = read_to_string(config.get_user_phonetic_selection_data()) {
                serde_json::from_str(&file).unwrap()
            } else {
                HashMap::with_hasher(RandomState::new())
            };

        PhoneticMethod {
            buffer: String::with_capacity(20),
            suggestion: PhoneticSuggestion::new(config),
            selections,
            prev_selection: 0,
        }
    }

    /// Returns `Suggestion` struct with suggestions.
    fn create_suggestion(&mut self, config: &Config) -> Suggestion {
        if config.get_phonetic_suggestion() {
            let (suggestions, selection) =
                self.suggestion.suggest(&self.buffer, &mut self.selections, config);

            self.prev_selection = selection;

            Suggestion::new(self.buffer.clone(), suggestions, self.prev_selection)
        } else {
            let suggestion = self.suggestion.suggest_only_phonetic(&self.buffer);

            Suggestion::new_lonely(suggestion)
        }
    }
}

impl Method for PhoneticMethod {
    fn get_suggestion(&mut self, key: u16, _modifier: u8, config: &Config) -> Suggestion {
        self.buffer.push(keycode_to_char(key));
        self.create_suggestion(config)
    }

    fn candidate_committed(&mut self, index: usize, config: &Config) {
        // Check if user has selected a different suggestion
        if self.prev_selection != index && config.get_phonetic_suggestion() {
            let suggestion = split_string(&self.suggestion.suggestions[index], true)
                .1
                .to_string();
            self.selections
                .insert(split_string(&self.buffer, false).1.to_string(), suggestion);
            write(
                config.get_user_phonetic_selection_data(),
                serde_json::to_string(&self.selections).unwrap(),
            )
            .unwrap();
        }

        // Reset to defaults
        self.buffer.clear();
    }

    fn update_engine(&mut self, config: &Config) {
        self.suggestion.database.update(config);
    }

    fn ongoing_input_session(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.buffer.clear();
    }

    fn backspace_event(&mut self, config: &Config) -> Suggestion {
        if !self.buffer.is_empty() {
            // Remove the last character.
            self.buffer.pop();

            if self.buffer.is_empty() {
                // The buffer is now empty, so return empty suggestion.
                return Suggestion::empty();
            }

            self.create_suggestion(config)
        } else {
            Suggestion::empty()
        }
    }
}

// Implement Default trait on PhoneticMethod for testing convenience.
impl Default for PhoneticMethod {
    fn default() -> Self {
        let config = get_phonetic_method_defaults();

        PhoneticMethod {
            buffer: String::new(),
            suggestion: PhoneticSuggestion::new(&config),
            selections: HashMap::with_hasher(RandomState::new()),
            prev_selection: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneticMethod;
    use crate::context::Method;
    use crate::config::get_phonetic_method_defaults;

    #[test]
    fn test_backspace() {
        let mut method = PhoneticMethod {
            buffer: "ab".to_string(),
            ..Default::default()
        };
        let config = get_phonetic_method_defaults();

        assert!(!method.backspace_event(&config).is_empty()); // a
        assert!(method.backspace_event(&config).is_empty()); // Empty
    }
}
