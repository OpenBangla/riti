/// Suggestions which are intend to be shown by the IM's candidate window.
pub struct Suggestion {
    suggestions: Vec<String>,
}

impl Suggestion {
    /// Creates a new `Suggestion` struct with given suggestions.
    pub fn new(suggestions: Vec<String>) -> Self {
        Suggestion { suggestions }
    }

    /// Constructs an empty Suggestion struct.
    pub fn empty() -> Self {
        Suggestion { suggestions: Vec::new() }
    }
}