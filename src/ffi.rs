use std::ffi::CString;
use std::os::raw::c_char;

use crate::context::RitiContext;
use crate::suggestion::Suggestion;

#[no_mangle]
pub extern fn riti_context_new() -> *mut RitiContext {
    Box::into_raw(Box::new(RitiContext::new()))
}

#[no_mangle]
pub extern fn riti_context_free(ptr: *mut RitiContext) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn riti_get_suggestion_for_key(ptr: *mut RitiContext, key: u16, modifier: u8) -> *mut Suggestion {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let suggestion = context.get_suggestion_for_key(key, modifier);

    Box::into_raw(Box::new(suggestion))
}

#[no_mangle]
pub extern fn riti_suggestion_free(ptr: *mut Suggestion) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn riti_context_key_handled(ptr: *mut RitiContext) -> bool {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    context.key_handled()
}

#[no_mangle]
pub extern fn riti_suggestion_get_suggestions(ptr: *const Suggestion) -> *const *mut c_char {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let slice = suggestion.get_suggestions();

    let mut res_vec = Vec::with_capacity(slice.len());

    for string in slice.into_iter() {
        unsafe {
            res_vec.push(CString::from_vec_unchecked(string.as_bytes().into()).into_raw());
        }
    }

    // Here we leak the memory for giving the ownership.
    let res = res_vec.as_ptr();
    std::mem::forget(res_vec);
    res
}

#[no_mangle]
pub extern fn riti_suggestion_get_auxiliary_text(ptr: *const Suggestion) -> *mut c_char {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    unsafe {
        CString::from_vec_unchecked(suggestion.get_auxiliary_text().into()).into_raw()
    }
}

#[no_mangle]
pub extern fn riti_suggestion_get_length(ptr: *const Suggestion) -> usize {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    suggestion.len()
}

#[no_mangle]
pub extern fn riti_suggestion_is_empty(ptr: *const Suggestion) -> bool {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    suggestion.is_empty()
}
