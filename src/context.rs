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
    pub fn getSuggestionForKey(&self, key: u16, shift: bool, ctrl: bool, alt: bool) -> Suggestion {
        self.method.borrow().getSuggestion(key, shift, ctrl, alt)
    }

    /// Returns bit masked integer instructing how the IM should handle the special key(eg. BackSpace,
    /// Enter, Space etc.)
    /// 
    /// See the [`Constants`](index.html#constants) which are bit masked for more information.
    pub fn handleSpecialKey(&self, key: u16) -> u8 {
        self.method.borrow().handleSpecialKey(key)
    }
}

pub(crate) trait Method {
    fn setLayout(&self, path: &str);
    fn getSuggestion(&self, key: u16, shift: bool, ctrl: bool, alt: bool) -> Suggestion;
    fn handleSpecialKey(&self, key: u16) -> u8;
}

/// IM needs to accept the key.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handleSpecialKey) function.
pub const IM_KEY_ACCEPTED: u8 = 1 << 0;
/// IM needs to commit the current suggestion.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handleSpecialKey) function.
pub const IM_COMMIT: u8 = 1 << 1;
/// IM needs to update suggestions.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handleSpecialKey) function,
/// usually when the key is BackSpace.
pub const IM_NEED_UPDATE: u8 = 1 << 2;
/// IM needs to reset.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handleSpecialKey) function,
/// usually when the key is BackSpace.
pub const IM_NEED_RESET: u8 = 1 << 3;