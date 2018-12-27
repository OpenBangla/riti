use std::cell::RefCell;
use crate::settings::Settings;

pub struct RitiContext {
    method: RefCell<Box<dyn Method>>,
    settings: Settings,
}

pub(crate) trait Method {
    fn setLayout(&self, path: &str);
}