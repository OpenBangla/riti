use std::cell::RefCell;

use crate::{config::Config, fixed::method::FixedMethod};
use crate::phonetic::method::PhoneticMethod;
use crate::suggestion::Suggestion;

/// Context handle used for libRiti IM APIs
pub struct RitiContext {
    method: RefCell<Box<dyn Method>>,
    config: Config,
}

impl RitiContext {
    /// A new `RitiContext` instance.
    pub fn new_with_config(config: &Config) -> Self {
        let config = config.to_owned();

        let method: RefCell<Box<dyn Method>> = if config.is_phonetic() {
            RefCell::new(Box::new(PhoneticMethod::new(&config)))
        } else {
            RefCell::new(Box::new(FixedMethod::new(&config)))
        };

        RitiContext { method, config }
    }

    /// Get suggestion for key.
    pub fn get_suggestion_for_key(&self, key: u16, modifier: u8) -> Suggestion {
        self.method.borrow_mut().get_suggestion(key, modifier, &self.config)
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
        // Update the config
        self.config = config.to_owned();

        // If the layout file has been changed.
        if self.config.layout_changed(config) {
            if config.is_phonetic() {
                self.method.replace(Box::new(PhoneticMethod::new(config)))
            } else {
                self.method.replace(Box::new(FixedMethod::new(config)))
            };
        } else {
            self.method.borrow_mut().update_engine(config);
        }
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
        self.method.borrow_mut().backspace_event(&self.config)
    }
}

pub(crate) trait Method {
    fn get_suggestion(&mut self, key: u16, modifier: u8, config: &Config) -> Suggestion;
    fn candidate_committed(&mut self, index: usize, config: &Config);
    fn update_engine(&mut self, config: &Config);
    fn ongoing_input_session(&self) -> bool;
    fn finish_input_session(&mut self);
    fn backspace_event(&mut self, config: &Config) -> Suggestion;
}

/// Shift modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_SHIFT: u8 = 1 << 0;
/// AltGr modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_ALT_GR: u8 = 1 << 1;
