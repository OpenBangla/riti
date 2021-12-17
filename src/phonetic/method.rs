// Phonetic Method
use ahash::RandomState;
use std::collections::HashMap;
use std::fs::{write, File};
use std::time::SystemTime;

use crate::config::Config;
use crate::context::Method;
use crate::data::Data;
use crate::keycodes::keycode_to_char;
use crate::phonetic::suggestion::PhoneticSuggestion;
use crate::suggestion::Suggestion;
use crate::utility::{read, split_string};

pub(crate) struct PhoneticMethod {
    buffer: String,
    suggestion: PhoneticSuggestion,
    // Candidate selections.
    selections: HashMap<String, String, RandomState>,
    // Last modification of the user's auto correct file.
    modified: SystemTime,
    // Previously selected candidate index of the current suggestion list.
    prev_selection: usize,
}

impl PhoneticMethod {
    /// Creates a new `PhoneticMethod` struct.
    pub(crate) fn new(config: &Config) -> Self {
        // Load candidate selections file.
        let selections = if let Ok(file) = std::fs::read(config.get_user_phonetic_selection_data())
        {
            serde_json::from_slice(&file).unwrap()
        } else {
            HashMap::with_hasher(RandomState::new())
        };

        // Load user's auto correct file.
        let (modified, autocorrect) = {
            if let Ok(mut file) = File::open(config.get_user_phonetic_autocorrect()) {
                let modified = file.metadata().unwrap().modified().unwrap();
                let autocorrect = serde_json::from_slice(&read(&mut file)).unwrap();
                (modified, autocorrect)
            } else {
                (
                    SystemTime::UNIX_EPOCH,
                    HashMap::with_hasher(RandomState::new()),
                )
            }
        };

        PhoneticMethod {
            buffer: String::with_capacity(20),
            suggestion: PhoneticSuggestion::new(autocorrect),
            selections,
            modified,
            prev_selection: 0,
        }
    }

    /// Returns `Suggestion` struct with suggestions.
    fn create_suggestion(&mut self, data: &Data, config: &Config) -> Suggestion {
        if config.get_phonetic_suggestion() {
            let (suggestions, selection) =
                self.suggestion
                    .suggest(&self.buffer, data, &mut self.selections, config);

            self.prev_selection = selection;

            Suggestion::new(self.buffer.clone(), &suggestions, self.prev_selection, config.get_ansi())
        } else {
            let suggestion = self.suggestion.suggest_only_phonetic(&self.buffer);

            Suggestion::new_lonely(suggestion, config.get_ansi())
        }
    }
}

impl Method for PhoneticMethod {
    fn get_suggestion(
        &mut self,
        key: u16,
        _modifier: u8,
        data: &Data,
        config: &Config,
    ) -> Suggestion {
        self.buffer.push(keycode_to_char(key));
        self.create_suggestion(data, config)
    }

    fn candidate_committed(&mut self, index: usize, config: &Config) {
        // Check if user has selected a different suggestion
        if self.prev_selection != index && config.get_phonetic_suggestion() {
            let suggestion = split_string(self.suggestion.suggestions[index].to_string(), true)
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
        if let Ok(mut file) = File::open(config.get_user_phonetic_autocorrect()) {
            let modified = file.metadata().unwrap().modified().unwrap();
            // Update the auto correct entries if only the file was modified in the meantime.
            if modified > self.modified {
                self.suggestion.user_autocorrect =
                    serde_json::from_slice(&read(&mut file)).unwrap();
                self.modified = modified;
            }
        }
    }

    fn ongoing_input_session(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.buffer.clear();
    }

    fn backspace_event(&mut self, data: &Data, config: &Config) -> Suggestion {
        if !self.buffer.is_empty() {
            // Remove the last character.
            self.buffer.pop();

            if self.buffer.is_empty() {
                // The buffer is now empty, so return empty suggestion.
                return Suggestion::empty();
            }

            self.create_suggestion(data, config)
        } else {
            Suggestion::empty()
        }
    }
}

// Implement Default trait on PhoneticMethod for testing convenience.
impl Default for PhoneticMethod {
    fn default() -> Self {
        PhoneticMethod {
            buffer: String::new(),
            suggestion: PhoneticSuggestion::new(HashMap::with_hasher(RandomState::new())),
            selections: HashMap::with_hasher(RandomState::new()),
            modified: SystemTime::UNIX_EPOCH,
            prev_selection: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneticMethod;
    use crate::config::get_phonetic_method_defaults;
    use crate::context::Method;
    use crate::data::Data;

    #[test]
    fn test_backspace() {
        let config = get_phonetic_method_defaults();
        let data = Data::new(&config);
        let mut method = PhoneticMethod {
            buffer: "ab".to_string(),
            ..Default::default()
        };

        assert!(!method.backspace_event(&data, &config).is_empty()); // a
        assert!(method.backspace_event(&data, &config).is_empty()); // Empty
    }
}
