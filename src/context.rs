use std::cell::RefCell;

use crate::fixed::method::FixedMethod;
use crate::loader::{LayoutLoader, LayoutType};
use crate::phonetic::method::PhoneticMethod;
use crate::suggestion::Suggestion;

/// Context handle used for libRiti IM APIs
pub struct RitiContext {
    method: RefCell<Box<dyn Method>>,
    loader: LayoutLoader,
}

impl RitiContext {
    /// A new `RitiContext` instance.
    pub fn new() -> Self {
        let loader = LayoutLoader::load_from_settings();

        match loader.layout_type() {
            LayoutType::Phonetic => {
                let method = RefCell::new(Box::new(PhoneticMethod::new(loader.layout())));
                RitiContext { method, loader }
            }
            LayoutType::Fixed => {
                let method = RefCell::new(Box::new(FixedMethod::new(loader.layout())));
                RitiContext { method, loader }
            }
        }
    }

    /// Get suggestion for key.
    pub fn get_suggestion_for_key(&self, key: u16, modifier: u8) -> Suggestion {
        self.method.borrow_mut().get_suggestion(key, modifier)
    }

    /// A candidate of the suggestion list was committed.
    ///
    /// `index`: index of the candidate.
    ///
    /// This function will end the ongoing input session.
    pub fn candidate_committed(&self, index: usize) {
        self.method.borrow_mut().candidate_committed(index)
    }

    /// Returns `true` if the key was handled, `false` otherwise.
    pub fn key_handled(&self) -> bool {
        self.method.borrow().key_handled()
    }

    /// Update the suggestion making engine. This would also look for changes
    /// in layout selection and AutoCorrect database.
    pub fn update_engine(&mut self) {
        if self.loader.changed() {
            self.loader = LayoutLoader::load_from_settings();

            match self.loader.layout_type() {
                LayoutType::Phonetic => self
                    .method
                    .replace(Box::new(PhoneticMethod::new(self.loader.layout()))),
                LayoutType::Fixed => self
                    .method
                    .replace(Box::new(FixedMethod::new(self.loader.layout()))),
            };
        } else {
            self.method.borrow_mut().update_engine();
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
    /// Returns `true` if the input method has handled the event.
    ///
    /// If the internal buffer becomes empty, this function will
    /// end the ongoing input session.
    pub fn backspace_event(&self) -> bool {
        self.method.borrow_mut().backspace_event()
    }
}

pub(crate) trait Method {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion;
    fn candidate_committed(&mut self, index: usize);
    fn key_handled(&self) -> bool;
    fn update_engine(&mut self);
    fn ongoing_input_session(&self) -> bool;
    fn finish_input_session(&mut self);
    fn backspace_event(&mut self) -> bool;
}

/// Shift modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_SHIFT: u8 = 1 << 0;
/// AltGr modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_ALT_GR: u8 = 1 << 1;
