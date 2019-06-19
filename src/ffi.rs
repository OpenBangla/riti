use std::ffi::CString;
use std::os::raw::c_char;

use crate::context::RitiContext;
use crate::suggestion::Suggestion;

#[no_mangle]
pub extern fn riti_context_new() -> *const RitiContext {
    Box::into_raw(Box::new(RitiContext::new()))
}

#[no_mangle]
pub extern fn riti_get_suggestion_for_key(ptr: *const RitiContext, key: u16, modifier: u8) -> Suggestion {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    context.get_suggestion_for_key(key, modifier)
}

#[no_mangle]
pub extern fn riti_context_key_handled(ptr: *const RitiContext) -> bool {
    let context = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    context.key_handled()
}

#[no_mangle]
pub extern fn riti_suggestion_get_suggestions(ptr: *const Suggestion) -> *const *const c_char {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let slice = suggestion.get_suggestions();

    let mut res_vec: Vec<*const c_char> = Vec::new();

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
pub extern fn riti_suggestion_get_length(ptr: *const Suggestion) -> usize {
    let suggestion = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    suggestion.len()
}
