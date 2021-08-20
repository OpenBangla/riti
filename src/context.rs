use std::cell::RefCell;

use crate::{config::Config, data::Data, fixed::method::FixedMethod};
use crate::phonetic::method::PhoneticMethod;
use crate::suggestion::Suggestion;

/// Context handle used for libRiti IM APIs
pub struct RitiContext {
    method: RefCell<Box<dyn Method>>,
    config: Config,
    data: Data,
}

impl RitiContext {
    /// A new `RitiContext` instance.
    pub fn new_with_config(config: &Config) -> Self {
        let config = config.to_owned();
        let data = Data::new(&config);
        let method = RefCell::new(<dyn Method>::new(&config));
        RitiContext { method, config, data }
    }

    /// Get suggestion for key.
    pub fn get_suggestion_for_key(&self, key: u16, modifier: u8) -> Suggestion {
        self.method.borrow_mut().get_suggestion(key, modifier, &self.data, &self.config)
    }

    /// A candidate of the suggestion list was committed.
    ///
    /// `index`: index of the candidate.
    ///
    /// This function will end the ongoing input session.
    pub fn candidate_committed(&self, index: usize) {
        self.method.borrow_mut().candidate_committed(index, &self.config)
    }

    /// Update the suggestion making engine. This would also look for changes
    /// in layout selection and AutoCorrect database.
    pub fn update_engine(&mut self, config: &Config) {
        // If the layout file has been changed.
        if self.config.layout_changed(config) {
            self.method.replace(<dyn Method>::new(config));
        } else {
            self.method.borrow_mut().update_engine(config);
        }

        // Update the config
        self.config = config.to_owned();
    }

    /// Checks if there is an onging input session.         
    pub fn ongoing_input_session(&self) -> bool {
        self.method.borrow().ongoing_input_session()
    }

    /// Finish the ongoing input session if any.
    pub fn finish_input_session(&self) {
        self.method.borrow_mut().finish_input_session();
    }

    /// A BackSpace event.
    ///
    /// Returns a new `suggestion` after applying the BackSpace event.
    ///
    /// If the internal buffer becomes empty, this function will
    /// end the ongoing input session.
    pub fn backspace_event(&self) -> Suggestion {
        self.method.borrow_mut().backspace_event(&self.data, &self.config)
    }
}

pub(crate) trait Method {
    fn get_suggestion(&mut self, key: u16, modifier: u8, data: &Data, config: &Config) -> Suggestion;
    fn candidate_committed(&mut self, index: usize, config: &Config);
    fn update_engine(&mut self, config: &Config);
    fn ongoing_input_session(&self) -> bool;
    fn finish_input_session(&mut self);
    fn backspace_event(&mut self, data: &Data, config: &Config) -> Suggestion;
}

impl dyn Method {
    fn new(config: &Config) -> Box<dyn Method> {
        if config.is_phonetic() {
            Box::new(PhoneticMethod::new(config))
        } else {
            Box::new(FixedMethod::new(config))
        }
    }
}

/// Shift modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_SHIFT: u8 = 1 << 0;
/// AltGr modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_ALT_GR: u8 = 1 << 1;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::{get_phonetic_method_defaults, get_fixed_method_defaults}, keycodes::{VC_E, VC_H, VC_L, VC_P}};

    #[test]
    fn test_layout_change() {
        // Load the context with a Phonetic layout.
        let config = get_phonetic_method_defaults();
        let mut context = RitiContext::new_with_config(&config);

        context.get_suggestion_for_key(VC_H, 0);
        context.get_suggestion_for_key(VC_E, 0);
        context.get_suggestion_for_key(VC_L, 0);
        let suggestion = context.get_suggestion_for_key(VC_P, 0);
        context.finish_input_session();
        assert_eq!(suggestion.get_suggestions().collect::<Vec<_>>(), ["হেল্প", "🆘"]);

        // Change the layout to Fixed layout
        let config = get_fixed_method_defaults();
        context.update_engine(&config);

        context.get_suggestion_for_key(VC_H, 0);
        context.get_suggestion_for_key(VC_E, 0);
        context.get_suggestion_for_key(VC_L, 0);
        let suggestion = context.get_suggestion_for_key(VC_P, 0);
        context.finish_input_session();
        assert_eq!(suggestion.get_suggestions().collect::<Vec<_>>(), ["হীলপ"]);
    }
}