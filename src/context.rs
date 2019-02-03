use std::cell::RefCell;
use crate::settings::Settings;
use crate::suggestion::Suggestion;

pub struct RitiContext {
    method: RefCell<Box<dyn Method>>,
    settings: Settings,
}

pub(crate) trait Method {
    fn setLayout(&self, path: &str);
    fn getSuggestion(&self, key: u16, shift: bool, ctrl: bool, alt: bool) -> Suggestion;
    fn handleSpecialKey(&self, key: u16) -> u8;
}