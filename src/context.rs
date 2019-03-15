use std::cell::RefCell;
use crate::settings::Settings;
use crate::suggestion::Suggestion;

/// Context handle used for libRiti IM APIs
pub struct RitiContext {
    method: RefCell<Box<dyn Method>>,
    settings: Settings,
}

impl RitiContext {
    /// Get suggestion for key.
    pub fn get_suggestion_for_key(&self, key: u16, modifier: u8) -> Suggestion {
        self.method.borrow_mut().get_suggestion(key, modifier)
    }

    /// Returns bit masked integer instructing how the IM should handle the special key(eg. BackSpace,
    /// Enter, Space etc.)
    /// 
    /// See the [`Constants`](index.html#constants) which are bit masked for more information.
    pub fn handle_special_key(&self, key: u16) -> u8 {
        self.method.borrow_mut().handle_special_key(key)
    }

    /// Was the key handled?
    pub fn key_handled(&self) -> bool {
        self.method.borrow().key_handled()
    }
}

pub(crate) trait Method {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion;
    fn handle_special_key(&mut self, key: u16) -> u8;
    fn key_handled(&self) -> bool;
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

/// IM needs to do nothing.
/// 
/// Returned by the [`handle_special_key()`](struct.RitiContext.html#method.handle_special_key) function.
pub const IM_DEFAULT: u8 = 0;
/// IM needs to accept the key.
/// 
/// Returned by the [`handle_special_key()`](struct.RitiContext.html#method.handle_special_key) function.
pub const IM_KEY_ACCEPTED: u8 = 1 << 0;
/// IM needs to commit the current suggestion.
/// 
/// Returned by the [`handle_special_key()`](struct.RitiContext.html#method.handle_special_key) function.
pub const IM_COMMIT: u8 = 1 << 1;
/// IM needs to update suggestions.
/// 
/// Returned by the [`handle_special_key()`](struct.RitiContext.html#method.handle_special_key) function,
/// usually when the key is BackSpace.
pub const IM_NEED_UPDATE: u8 = 1 << 2;
/// IM needs to reset.
/// 
/// Returned by the [`handle_special_key()`](struct.RitiContext.html#method.handle_special_key) function,
/// usually when the key is BackSpace.
pub const IM_NEED_RESET: u8 = 1 << 3;