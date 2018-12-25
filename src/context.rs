struct RitiContext {
    data: String,
    method: Box<Method>,
}

trait Method {
    fn setLayout(&self, path: &str);
}