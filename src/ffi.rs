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
