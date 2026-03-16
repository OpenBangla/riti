use okkhor::parser::Parser;

use crate::config::Config;
use crate::context::Method;
use crate::keycodes::keycode_to_char;
use crate::predictor::Predictor;
use crate::suggestion::Suggestion;

pub(crate) struct PredictivePhoneticMethod {
    buffer: String,
    parser: Parser,
    predictor: Option<Predictor>,
    last_was_space: bool,
}

impl PredictivePhoneticMethod {
    pub(crate) fn new(config: &Config) -> Self {
        Self {
            buffer: String::with_capacity(64),
            parser: Parser::new_phonetic(),
            predictor: Predictor::new(config).ok(),
            last_was_space: false,
        }
    }

    fn create_suggestion(&mut self, config: &Config) -> Suggestion {
        // Split buffer at last space into context (completed words) and current word
        let (context_phonetic, current_word_phonetic) =
            if let Some(pos) = self.buffer.rfind(' ') {
                (&self.buffer[..pos], &self.buffer[pos + 1..])
            } else {
                ("", self.buffer.as_str())
            };

        // Convert completed words to Bengali
        let bengali_context: String = if context_phonetic.is_empty() {
            String::new()
        } else {
            context_phonetic
                .split(' ')
                .map(|w| self.parser.convert(w))
                .collect::<Vec<_>>()
                .join(" ")
        };

        // Convert current word being typed to Bengali
        let bengali_prefix = if current_word_phonetic.is_empty() {
            String::new()
        } else {
            self.parser.convert(current_word_phonetic)
        };

        // Build auxiliary text: full Bengali rendering
        let auxiliary = if bengali_context.is_empty() {
            bengali_prefix.clone()
        } else if bengali_prefix.is_empty() {
            format!("{} ", bengali_context)
        } else {
            format!("{} {}", bengali_context, bengali_prefix)
        };

        // Generate predictions
        let suggestions = if let Some(ref mut predictor) = self.predictor {
            let predictions = predictor.predict(&bengali_context, &bengali_prefix, 10);
            if !predictions.is_empty() {
                predictions.into_iter().map(|(s, _)| s).collect()
            } else if !bengali_prefix.is_empty() {
                vec![bengali_prefix]
            } else if !bengali_context.is_empty() {
                // Context only (e.g. after space): offer context as committable suggestion
                vec![bengali_context.clone()]
            } else {
                vec![]
            }
        } else if !bengali_prefix.is_empty() {
            vec![bengali_prefix]
        } else if !bengali_context.is_empty() {
            vec![bengali_context.clone()]
        } else {
            vec![]
        };

        Suggestion::Full {
            auxiliary,
            suggestions,
            selection: 0,
            ansi: config.get_ansi_encoding(),
        }
    }
}

impl Method for PredictivePhoneticMethod {
    fn get_suggestion(
        &mut self,
        key: u16,
        _modifier: u8,
        _selection: u8,
        _data: &crate::data::Data,
        config: &Config,
    ) -> (Suggestion, bool) {
        let character = keycode_to_char(key);

        // Don't start a session with a space
        if self.buffer.is_empty() && character == ' ' {
            return (Suggestion::empty(), false);
        }

        // Double-space detection: end session
        if character == ' ' && self.last_was_space {
            // Remove the trailing space that was added on the first space press
            self.buffer.pop();
            let suggestion = self.create_suggestion(config);
            self.finish_input_session();
            return (suggestion, false);
        }

        self.last_was_space = character == ' ';
        self.buffer.push(character);
        (self.create_suggestion(config), true)
    }

    fn candidate_committed(&mut self, _index: usize, _config: &Config) {
        self.buffer.clear();
        self.last_was_space = false;
    }

    fn update_engine(&mut self, config: &Config) {
        self.predictor = Predictor::new(config).ok();
    }

    fn ongoing_input_session(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.buffer.clear();
        self.last_was_space = false;
    }

    fn backspace_event(
        &mut self,
        ctrl: bool,
        _data: &crate::data::Data,
        config: &Config,
    ) -> Suggestion {
        if self.buffer.is_empty() {
            return Suggestion::empty();
        }

        if ctrl {
            // Ctrl+Backspace: delete last word
            if let Some(pos) = self.buffer.rfind(' ') {
                self.buffer.truncate(pos + 1);
            } else {
                self.buffer.clear();
            }
        } else {
            // Regular backspace: remove last character
            self.buffer.pop();
            self.last_was_space = self.buffer.ends_with(' ');
        }

        if self.buffer.is_empty() {
            return Suggestion::empty();
        }

        self.create_suggestion(config)
    }
}

#[cfg(test)]
impl PredictivePhoneticMethod {
    fn new_test() -> Self {
        let config = crate::config::get_predictive_method_defaults();
        Self::new(&config)
    }
}

#[cfg(test)]
mod tests {
    use super::PredictivePhoneticMethod;
    use crate::config::get_predictive_method_defaults;
    use crate::context::Method;
    use crate::data::Data;
    use crate::keycodes::*;
    use crate::suggestion::Suggestion;

    fn press_key(
        method: &mut PredictivePhoneticMethod,
        key: u16,
        data: &Data,
        config: &crate::config::Config,
    ) -> (Suggestion, bool) {
        method.get_suggestion(key, 0, 0, data, config)
    }

    #[test]
    fn test_basic_keypress_with_predictions() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        press_key(&mut method, VC_A, &data, &config);
        press_key(&mut method, VC_M, &data, &config);
        let (suggestion, handled) = press_key(&mut method, VC_I, &data, &config);

        assert!(handled);
        assert!(method.ongoing_input_session());

        let aux = suggestion.get_auxiliary_text();
        assert_eq!(aux, "আমি");

        // Model should return predictions
        let suggestions = suggestion.get_suggestions();
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_single_space_continues_session() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        press_key(&mut method, VC_A, &data, &config);
        press_key(&mut method, VC_M, &data, &config);
        press_key(&mut method, VC_I, &data, &config);
        let (suggestion, handled) = press_key(&mut method, VC_SPACE, &data, &config);

        assert!(handled);
        assert!(method.ongoing_input_session());
        // Auxiliary should contain the Bengali context with trailing space
        let aux = suggestion.get_auxiliary_text();
        assert!(aux.contains("আমি"));
    }

    #[test]
    fn test_double_space_ends_session() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        press_key(&mut method, VC_A, &data, &config);
        press_key(&mut method, VC_M, &data, &config);
        press_key(&mut method, VC_I, &data, &config);

        // First space continues session
        let (_, handled) = press_key(&mut method, VC_SPACE, &data, &config);
        assert!(handled);

        // Second space ends session
        let (_, handled) = press_key(&mut method, VC_SPACE, &data, &config);
        assert!(!handled);
        assert!(!method.ongoing_input_session());
    }

    #[test]
    fn test_space_on_empty_buffer_not_consumed() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        let (suggestion, handled) = press_key(&mut method, VC_SPACE, &data, &config);
        assert!(!handled);
        assert!(suggestion.is_empty());
        assert!(!method.ongoing_input_session());
    }

    #[test]
    fn test_regular_backspace() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        press_key(&mut method, VC_A, &data, &config);
        press_key(&mut method, VC_M, &data, &config);
        press_key(&mut method, VC_I, &data, &config);

        let suggestion = method.backspace_event(false, &data, &config);
        assert!(!suggestion.is_empty());
        // Buffer should be "am" now
        let aux = suggestion.get_auxiliary_text();
        assert_eq!(aux, "আম");
    }

    #[test]
    fn test_backspace_after_space_resets_double_space_tracking() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        press_key(&mut method, VC_A, &data, &config);
        press_key(&mut method, VC_M, &data, &config);
        press_key(&mut method, VC_I, &data, &config);
        press_key(&mut method, VC_SPACE, &data, &config);

        // Backspace removes the space
        method.backspace_event(false, &data, &config);

        // Now a single space should NOT end session (double-space tracking was reset)
        let (_, handled) = press_key(&mut method, VC_SPACE, &data, &config);
        assert!(handled);
        assert!(method.ongoing_input_session());
    }

    #[test]
    fn test_ctrl_backspace_deletes_last_word() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        // Type "ami tomar"
        for key in [VC_A, VC_M, VC_I, VC_SPACE, VC_T, VC_O, VC_M, VC_A, VC_R] {
            press_key(&mut method, key, &data, &config);
        }

        // Ctrl+Backspace should delete "tomar", leaving "ami "
        let suggestion = method.backspace_event(true, &data, &config);
        assert!(!suggestion.is_empty());
        let aux = suggestion.get_auxiliary_text();
        assert!(aux.contains("আমি"));
    }

    #[test]
    fn test_ctrl_backspace_single_word_clears() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        press_key(&mut method, VC_A, &data, &config);
        press_key(&mut method, VC_M, &data, &config);
        press_key(&mut method, VC_I, &data, &config);

        let suggestion = method.backspace_event(true, &data, &config);
        assert!(suggestion.is_empty());
        assert!(!method.ongoing_input_session());
    }

    #[test]
    fn test_multi_word_context_predictions() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        // Type "ami tomar"
        for key in [VC_A, VC_M, VC_I, VC_SPACE, VC_T, VC_O, VC_M, VC_A, VC_R] {
            press_key(&mut method, key, &data, &config);
        }

        let (suggestion, handled) = press_key(&mut method, VC_R, &data, &config);
        // Reread - we just pushed an extra 'r', let's check the auxiliary
        // Actually the buffer is "ami tomarr" now, but let's just verify multi-word context
        assert!(handled);
        // The auxiliary should contain Bengali text for context
        let aux = suggestion.get_auxiliary_text();
        assert!(aux.contains("আমি"));
    }

    #[test]
    fn test_backspace_on_empty_buffer() {
        let config = get_predictive_method_defaults();
        let data = Data::new();
        let mut method = PredictivePhoneticMethod::new_test();

        let suggestion = method.backspace_event(false, &data, &config);
        assert!(suggestion.is_empty());
    }
}
