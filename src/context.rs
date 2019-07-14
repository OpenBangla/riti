use crate::suggestion::Suggestion;
use crate::phonetic::method::PhoneticMethod;
use crate::fixed::method::FixedMethod;
use crate::loader::{LayoutLoader, LayoutType};

/// Context handle used for libRiti IM APIs
pub struct RitiContext {
    method: Box<dyn Method>,
    loader: LayoutLoader,
}

impl RitiContext {
    /// A new `RitiContext` instance.
    pub fn new() -> Self {
        let loader = LayoutLoader::load_from_settings();

        match loader.layout_type() {
            LayoutType::Phonetic => {
                let method = Box::new(PhoneticMethod::new(loader.layout()));
                RitiContext { method, loader }
            }
            LayoutType::Fixed => {
                let method = Box::new(FixedMethod::new(loader.layout()));
                RitiContext { method, loader }
            }
        }
    }

    /// Get suggestion for key.
    pub fn get_suggestion_for_key(&mut self, key: u16, modifier: u8) -> Suggestion {
        self.method.get_suggestion(key, modifier)
    }

    /// A candidate of the suggestion list was committed.
    /// 
    /// `index`: index of the candidate.
    pub fn candidate_committed(&mut self, index: usize) {
        self.method.candidate_committed(index)
    }

    /// Returns `true` if the key was handled, `false` otherwise.
    pub fn key_handled(&self) -> bool {
        self.method.key_handled()
    }

    /// Update the suggestion making engine. This would also look for changes
    /// in layout selection and AutoCorrect database.
    pub fn update_engine(&mut self) {
        if self.loader.changed() {
            self.loader = LayoutLoader::load_from_settings();

            match self.loader.layout_type() {
                LayoutType::Phonetic => self.method = Box::new(PhoneticMethod::new(self.loader.layout())),
                LayoutType::Fixed => self.method = Box::new(FixedMethod::new(self.loader.layout()))
            };
        } else {
            self.method.update_engine();
        }
    }
}

pub(crate) trait Method {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion;
    fn candidate_committed(&mut self, index: usize);
    fn key_handled(&self) -> bool;
    fn update_engine(&mut self);
}

/// Shift modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_SHIFT: u8 = 1 << 0;
/// Ctrl modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_CTRL: u8 = 1 << 1;
/// Alt modifier key.
///
/// Used by the [`get_suggestion_for_key()`](struct.RitiContext.html#method.get_suggestion_for_key) function.
pub const MODIFIER_ALT: u8 = 1 << 2;
