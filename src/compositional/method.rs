// Compositional Method
//
// Keeps a roman "tape" and feeds it to a `CompositionalEngine` to compose
// Bengali text. The composed text is always the first candidate; dictionary and
// emoji candidates follow when compositional suggestion is enabled.

use upodesh::bangla::suggest;

use super::engine::CompositionalEngine;
use super::engine_for_layout;
use crate::config::Config;
use crate::context::Method;
use crate::data::Data;
use crate::keycodes::keycode_to_char;
use crate::suggestion::{Rank, Suggestion};
use crate::utility::{clean_string, smart_quoter, SplittedString};

pub(crate) struct CompositionalMethod {
    /// The roman characters typed so far (the input "tape").
    tape: String,
    /// The composed Bengali output for the current `tape`.
    buffer: String,
    suggestions: Vec<Rank>,
    engine: Box<dyn CompositionalEngine>,
}

impl Method for CompositionalMethod {
    fn get_suggestion(
        &mut self,
        key: u16,
        _modifier: u8,
        _selection: u8,
        data: &Data,
        config: &Config,
    ) -> Suggestion {
        let character = keycode_to_char(key);
        self.tape.push(character);
        self.buffer = self.engine.convert(&self.tape);

        self.create_suggestion(data, config)
    }

    fn candidate_committed(&mut self, _index: usize, _config: &Config) {
        self.tape.clear();
        self.buffer.clear();
    }

    fn update_engine(&mut self, _config: &Config) {
        // The spec is static; a layout switch is handled by the context
        // recreating the method.
    }

    fn ongoing_input_session(&self) -> bool {
        !self.tape.is_empty()
    }

    fn finish_input_session(&mut self) {
        self.tape.clear();
        self.buffer.clear();
    }

    fn backspace_event(&mut self, ctrl: bool, data: &Data, config: &Config) -> Suggestion {
        if self.tape.is_empty() {
            return Suggestion::empty();
        }

        if ctrl {
            // Whole word deletion: Ctrl + Backspace combination.
            self.tape.clear();
            self.buffer.clear();
            return Suggestion::empty();
        }

        // Drop the last roman character and re-convert.
        self.tape.pop();
        if self.tape.is_empty() {
            self.buffer.clear();
            return Suggestion::empty();
        }

        self.buffer = self.engine.convert(&self.tape);
        self.create_suggestion(data, config)
    }
}

impl CompositionalMethod {
    /// Creates a new `CompositionalMethod` using the layout named in `config`.
    pub(crate) fn new(config: &Config) -> Self {
        CompositionalMethod {
            tape: String::with_capacity(20),
            buffer: String::with_capacity(20 * 3),
            suggestions: Vec::with_capacity(10),
            engine: engine_for_layout(config.get_layout_file_path()),
        }
    }

    fn create_suggestion(&mut self, data: &Data, config: &Config) -> Suggestion {
        if config.get_compositional_suggestion() {
            self.create_dictionary_suggestion(data, config)
        } else {
            Suggestion::new_lonely(self.buffer.clone(), config.get_ansi_encoding())
        }
    }

    fn create_dictionary_suggestion(&mut self, data: &Data, config: &Config) -> Suggestion {
        let mut string = SplittedString::split(&self.buffer, true);

        // Smart Quoting feature.
        if config.get_smart_quote() {
            string = smart_quoter(string);
        }

        let (first_part, word, last_part) = string.as_tuple();

        self.suggestions.clear();

        // The exact composed word is always the first candidate.
        self.suggestions.push(Rank::first_ranked(word.to_string()));

        // Dictionary suggestions for the composed word.
        let mut words = suggest(&clean_string(word));
        words.sort_unstable();
        self.suggestions
            .extend(words.into_iter().map(|s| Rank::new_suggestion(s.clone(), word)));

        // Remove the duplicates if present.
        self.suggestions.dedup();

        // Add preceding and trailing meta characters.
        if !first_part.is_empty() || !last_part.is_empty() {
            for suggestion in self.suggestions.iter_mut() {
                *suggestion.change_item() =
                    format!("{}{}{}", first_part, suggestion.to_string(), last_part);
            }
        }

        if !config.get_ansi_encoding() {
            // Emoji addition with Emoticons.
            if let Some(emoji) = data.get_emoji_by_emoticon(&self.tape) {
                self.suggestions.push(Rank::emoji(emoji.to_owned()));
            } else if let Some(emojis) = data.get_emoji_by_bengali(word) {
                // Emoji addition with its Bengali name.
                let emojis = emojis
                    .zip(1..)
                    .map(|(s, r)| Rank::emoji_ranked(format!("{first_part}{s}{last_part}"), r));
                self.suggestions.extend(emojis);
            }
        }

        // Sort the suggestions.
        self.suggestions.sort_unstable();

        // Reduce the number of suggestions and add the typed english word at the
        // end. Also check that the typed text is not already included.
        if config.get_suggestion_include_english() && self.buffer != self.tape {
            self.suggestions.truncate(8);
            self.suggestions.push(Rank::last_ranked(self.tape.clone(), 1));
        } else {
            self.suggestions.truncate(9);
        }

        Suggestion::new(
            // The auxiliary text shows the roman keys the user actually pressed,
            // while the candidates carry the composed Bengali output.
            self.tape.clone(),
            &self.suggestions,
            0,
            config.get_ansi_encoding(),
        )
    }
}

#[cfg(test)]
impl Default for CompositionalMethod {
    fn default() -> Self {
        CompositionalMethod {
            tape: String::new(),
            buffer: String::new(),
            suggestions: Vec::new(),
            engine: engine_for_layout(super::KHIPRO_LAYOUT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CompositionalMethod;
    use crate::config::get_khipro_method_defaults;
    use crate::context::Method;
    use crate::data::Data;
    use crate::keycodes::{VC_H, VC_K, VC_N};

    #[test]
    fn test_basic_composition() {
        let mut method = CompositionalMethod::default();
        let data = Data::new();
        let config = get_khipro_method_defaults();

        // "kkh" -> "ক্ষ"
        method.get_suggestion(VC_K, 0, 0, &data, &config);
        method.get_suggestion(VC_K, 0, 0, &data, &config);
        let suggestion = method.get_suggestion(VC_H, 0, 0, &data, &config);
        assert_eq!(suggestion.get_suggestions()[0], "ক্ষ");
        method.finish_input_session();

        // "kkhn" -> "ক্ষ্ণ"
        method.get_suggestion(VC_K, 0, 0, &data, &config);
        method.get_suggestion(VC_K, 0, 0, &data, &config);
        method.get_suggestion(VC_H, 0, 0, &data, &config);
        let suggestion = method.get_suggestion(VC_N, 0, 0, &data, &config);
        assert_eq!(suggestion.get_suggestions()[0], "ক্ষ্ণ");
    }

    #[test]
    fn test_backspace_replays() {
        let mut method = CompositionalMethod::default();
        let data = Data::new();
        let config = get_khipro_method_defaults();

        method.get_suggestion(VC_K, 0, 0, &data, &config);
        method.get_suggestion(VC_K, 0, 0, &data, &config);
        let suggestion = method.get_suggestion(VC_H, 0, 0, &data, &config);
        assert_eq!(suggestion.get_suggestions()[0], "ক্ষ");

        // Removing the last roman key "h" should fall back to "kk" -> "ক্ক".
        let suggestion = method.backspace_event(false, &data, &config);
        assert_eq!(suggestion.get_suggestions()[0], "ক্ক");

        // Ctrl backspace clears everything.
        let suggestion = method.backspace_event(true, &data, &config);
        assert!(suggestion.is_empty());
        assert!(!method.ongoing_input_session());
    }
}
