// Khipro Method
//
// Khipro is a modern, shift-free phonetic transliteration scheme for Bangla.
// Unlike the Avro Phonetic method, Khipro is *deterministic*: a given sequence
// of roman keystrokes maps to exactly one Bangla output. Therefore there is no
// dictionary based candidate list — we emit a single "lonely" suggestion built
// directly by the `okkhor` Khipro transliterator.
use okkhor::khipro::KhiproPhonetic;

use crate::config::Config;
use crate::context::Method;
use crate::data::Data;
use crate::keycodes::keycode_to_char;
use crate::suggestion::Suggestion;

pub(crate) struct KhiproMethod {
    buffer: String,
    parser: KhiproPhonetic,
}

impl KhiproMethod {
    /// Creates a new `KhiproMethod` struct.
    pub(crate) fn new(_config: &Config) -> Self {
        KhiproMethod {
            buffer: String::with_capacity(20),
            parser: KhiproPhonetic::new(),
        }
    }

    /// Transliterate the current buffer into a single (lonely) `Suggestion`.
    fn create_suggestion(&self, config: &Config) -> Suggestion {
        let converted = self.parser.convert(&self.buffer);
        Suggestion::new_lonely(converted, config.get_ansi_encoding())
    }
}

impl Method for KhiproMethod {
    fn get_suggestion(
        &mut self,
        key: u16,
        _modifier: u8,
        _selection: u8,
        _data: &Data,
        config: &Config,
    ) -> Suggestion {
        let character = keycode_to_char(key);
        self.buffer.push(character);
        self.create_suggestion(config)
    }

    fn candidate_committed(&mut self, _index: usize, _config: &Config) {
        // Deterministic method: no candidate selection to remember.
        self.buffer.clear();
    }

    fn update_engine(&mut self, _config: &Config) {
        // Khipro is deterministic and stateless; nothing to update.
    }

    fn ongoing_input_session(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.buffer.clear();
    }

    fn backspace_event(&mut self, ctrl: bool, _data: &Data, config: &Config) -> Suggestion {
        if !self.buffer.is_empty() {
            // Whole word deletion: Ctrl + Backspace combination.
            if ctrl {
                self.buffer.clear();
                return Suggestion::empty();
            }

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

// Implement Default trait on KhiproMethod for testing convenience.
impl Default for KhiproMethod {
    fn default() -> Self {
        KhiproMethod {
            buffer: String::new(),
            parser: KhiproPhonetic::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KhiproMethod;
    use crate::config::get_khipro_method_defaults;
    use crate::context::Method;
    use crate::data::Data;
    use crate::keycodes::{VC_A, VC_M, VC_R};

    #[test]
    fn test_basic_conversion() {
        let config = get_khipro_method_defaults();
        let data = Data::new();
        let mut method = KhiproMethod::default();

        // Type "amar" -> "আমার".
        method.get_suggestion(VC_A, 0, 0, &data, &config);
        method.get_suggestion(VC_M, 0, 0, &data, &config);
        method.get_suggestion(VC_A, 0, 0, &data, &config);
        let suggestion = method.get_suggestion(VC_R, 0, 0, &data, &config);

        assert!(suggestion.is_lonely());
        assert_eq!(suggestion.get_lonely_suggestion(), "আমার");

        method.finish_input_session();
        assert!(!method.ongoing_input_session());
    }

    #[test]
    fn test_backspace() {
        let config = get_khipro_method_defaults();
        let data = Data::new();
        let mut method = KhiproMethod {
            buffer: "am".to_string(),
            ..Default::default()
        };

        // One BackSpace leaves "a", another empties the buffer.
        assert!(!method.backspace_event(false, &data, &config).is_empty());
        assert!(method.backspace_event(false, &data, &config).is_empty());

        // Ctrl + Backspace clears the whole buffer at once.
        method = KhiproMethod {
            buffer: "amar".to_string(),
            ..Default::default()
        };
        assert!(method.backspace_event(true, &data, &config).is_empty());
        assert!(!method.ongoing_input_session());
    }
}
