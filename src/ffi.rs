use std::ffi::CString;
use std::os::raw::c_char;

use crate::context::RitiContext;
use crate::suggestion::Suggestion;

// FFI functions for handling the `RitiContext` structure.

#[no_mangle]
pub extern "C" fn riti_context_new() -> *mut RitiContext {
    Box::into_raw(Box::new(RitiContext::new()))
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
pub extern "C" fn riti_context_update_engine(ptr: *mut RitiContext) {
    let context = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    context.update_engine()
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

#[no_mangle]
pub extern "C" fn riti_suggestion_get_suggestions(ptr: *const Suggestion) -> *mut *mut c_char {
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

    // Shrink capacity close to the length and ensure that it's equal in size.
    res_vec.shrink_to_fit();
    assert_eq!(res_vec.capacity(), res_vec.len());

    // Here we leak the memory for giving the ownership.
    let res = res_vec.as_mut_ptr();
    std::mem::forget(res_vec);
    res
}

/// Free the string array `ptr` of `len` length previously allocated by other function.
#[no_mangle]
pub extern "C" fn riti_string_array_free(ptr: *mut *mut c_char, len: usize) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        // Safe because we ensure that the capacity and the length of the vector 
        // is same while returning a pointer of that vector.
        let vec = Vec::from_raw_parts(ptr, len, len);
        
        // Now reconstitute the values to properly deallocate them.
        for item in vec {
            CString::from_raw(item);
        }
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
