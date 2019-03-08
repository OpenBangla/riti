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
    pub fn get_suggestion_for_key(&self, key: u16, shift: bool, ctrl: bool, alt: bool) -> Suggestion {
        self.method.borrow().get_suggestion(key, shift, ctrl, alt)
    }

    /// Returns bit masked integer instructing how the IM should handle the special key(eg. BackSpace,
    /// Enter, Space etc.)
    /// 
    /// See the [`Constants`](index.html#constants) which are bit masked for more information.
    pub fn handle_special_key(&self, key: u16) -> u8 {
        self.method.borrow().handle_special_key(key)
    }
}

pub(crate) trait Method {
    fn set_layout(&self, path: &str);
    fn get_suggestion(&self, key: u16, shift: bool, ctrl: bool, alt: bool) -> Suggestion;
    fn handle_special_key(&self, key: u16) -> u8;
}

/// IM needs to accept the key.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handle_special_key) function.
pub const IM_KEY_ACCEPTED: u8 = 1 << 0;
/// IM needs to commit the current suggestion.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handle_special_key) function.
pub const IM_COMMIT: u8 = 1 << 1;
/// IM needs to update suggestions.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handle_special_key) function,
/// usually when the key is BackSpace.
pub const IM_NEED_UPDATE: u8 = 1 << 2;
/// IM needs to reset.
/// 
/// Returned by the [`handleSpecialKey()`](struct.RitiContext.html#method.handle_special_key) function,
/// usually when the key is BackSpace.
pub const IM_NEED_RESET: u8 = 1 << 3;