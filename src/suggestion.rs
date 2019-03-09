/// Suggestions which are intend to be shown by the IM's candidate window.
pub struct Suggestion {
    suggestions: Vec<String>,
}

impl Suggestion {
    pub(crate) fn new() -> Self {
        Suggestion { suggestions: Vec::new() }
    }
}