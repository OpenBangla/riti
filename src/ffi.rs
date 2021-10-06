use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::context::RitiContext;
use crate::suggestion::Suggestion;
use crate::config::Config;

// FFI functions for handling the `RitiContext` structure.

/// Creates a new instance of RitiContext with a Config which is properly
/// populated using `riti_config_set_*` set of functions.
#[no_mangle]
pub extern "C" fn riti_context_new_with_config(ptr: *const Config) -> *mut RitiContext {
    let config = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    Box::into_raw(Box::new(RitiContext::new_with_config(config)))
}

#[no_mangle]
pub extern "C" fn riti_context_free(ptr: *mut RitiContext) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn riti_get_suggestion_for_key(
    ptr: *mut RitiContext,
    key: u16,
    modifier: u8,
) -> *mut Suggestion {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let suggestion = context.get_suggestion_for_key(key, modifier);

    Box::into_raw(Box::new(suggestion))
}

/// A candidate of the suggestion list was committed.
///
/// `index`: index of the candidate.
///
/// This function will end the ongoing input session.
#[no_mangle]
pub extern "C" fn riti_context_candidate_committed(ptr: *mut RitiContext, index: usize) {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    context.candidate_committed(index)
}

/// Update the suggestion making engine. This would also look for changes
/// in layout selection and AutoCorrect database.
#[no_mangle]
pub extern "C" fn riti_context_update_engine(ptr: *mut RitiContext, config: *const Config) {
    let context = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let config = unsafe {
        assert!(!config.is_null());
        &*config
    };

    context.update_engine(config)
}

/// Checks if there is an ongoing input session.
#[no_mangle]
pub extern "C" fn riti_context_ongoing_input_session(ptr: *mut RitiContext) -> bool {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    context.ongoing_input_session()
}

/// Finish the ongoing input session if any.
#[no_mangle]
pub extern "C" fn riti_context_finish_input_session(ptr: *mut RitiContext) {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    context.finish_input_session()
}

/// A BackSpace event.
///
/// Returns a new `suggestion` after applying the BackSpace event.
///
/// If the internal buffer becomes empty, this function will
/// end the ongoing input session.
#[no_mangle]
pub extern "C" fn riti_context_backspace_event(ptr: *mut RitiContext) -> *mut Suggestion {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let suggestion = context.backspace_event();

    Box::into_raw(Box::new(suggestion))
}

// FFI functions for handling the `Suggestion` structure.

#[no_mangle]
pub extern "C" fn riti_suggestion_free(ptr: *mut Suggestion) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

/// Get the suggestion of the `index` from suggestions.
#[no_mangle]
pub extern "C" fn riti_suggestion_get_suggestion(ptr: *const Suggestion, index: usize) -> *mut c_char {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let string = suggestion.get_suggestions()[index].clone();

    unsafe {
        CString::from_vec_unchecked(string.into()).into_raw()
    }
}

/// Get the only suggestion of the *lonely* `Suggestion`.
#[no_mangle]
pub extern "C" fn riti_suggestion_get_lonely_suggestion(ptr: *const Suggestion) -> *mut c_char {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    unsafe { CString::from_vec_unchecked(suggestion.get_lonely_suggestion().into()).into_raw() }
}

#[no_mangle]
pub extern "C" fn riti_suggestion_get_auxiliary_text(ptr: *const Suggestion) -> *mut c_char {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    unsafe { CString::from_vec_unchecked(suggestion.get_auxiliary_text().into()).into_raw() }
}

/// Free the allocated string.
#[no_mangle]
pub extern "C" fn riti_string_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        CString::from_raw(ptr);
    }
}

/// Returns index of the suggestion, which was previously selected.
#[no_mangle]
pub extern "C" fn riti_suggestion_previously_selected_index(ptr: *const Suggestion) -> usize {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    suggestion.previously_selected_index()
}

#[no_mangle]
pub extern "C" fn riti_suggestion_get_length(ptr: *const Suggestion) -> usize {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    suggestion.len()
}

/// Returns `true` when the `Suggestion` struct is a **lonely** one, otherwise returns `false`.
///
/// A *lonely* `Suggestion` struct means that the struct has only one suggestion.
#[no_mangle]
pub extern "C" fn riti_suggestion_is_lonely(ptr: *const Suggestion) -> bool {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    suggestion.is_lonely()
}

#[no_mangle]
pub extern "C" fn riti_suggestion_is_empty(ptr: *const Suggestion) -> bool {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    suggestion.is_empty()
}

/// Creates a new instance of Config which is used to initialize
/// and to control the configuration of RitiContext.
/// 
/// This function creates an instance of Config in an initial
/// state which can't be used before populating the Config using
/// `riti_config_set_*` set of functions.
#[no_mangle]
pub extern "C" fn riti_config_new() -> *mut Config {
    Box::into_raw(Box::new(Config::default()))
}


/// Free the allocated Config struct.
#[no_mangle]
pub extern "C" fn riti_config_free(ptr: *mut Config) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn riti_config_set_layout_file(ptr: *mut Config, path: *const c_char) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    unsafe {
        let layout = CStr::from_ptr(path).to_str().unwrap();
        config.set_layout_file_path(layout);
    }
}

#[no_mangle]
pub extern "C" fn riti_config_set_database_dir(ptr: *mut Config, path: *const c_char) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    unsafe {
        let path = CStr::from_ptr(path).to_str().unwrap();
        config.set_database_dir(path);
    }
}

#[no_mangle]
pub extern "C" fn riti_config_set_suggestion_include_english(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_suggestion_include_english(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_phonetic_suggestion(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_phonetic_suggestion(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_fixed_suggestion(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_fixed_suggestion(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_fixed_auto_vowel(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_fixed_automatic_vowel(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_fixed_auto_chandra(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_fixed_automatic_chandra(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_fixed_traditional_kar(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_fixed_traditional_kar(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_fixed_old_reph(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_fixed_old_reph(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_fixed_numpad(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_fixed_numpad(option);
}

#[no_mangle]
pub extern "C" fn riti_config_set_fixed_old_kar_order(ptr: *mut Config, option: bool) {
    let config = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    config.set_fixed_old_kar_order(option);
}
