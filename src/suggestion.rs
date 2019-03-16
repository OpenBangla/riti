/// Suggestions which are intend to be shown by the IM's candidate window.
pub struct Suggestion {
    // Auxiliary text
    auxiliary: String,
    suggestions: Vec<String>,
}

impl Suggestion {
    /// Creates a new `Suggestion` struct with given arguments.
    /// 
    /// `auxiliary`: The auxiliary text.
    /// 
    /// `suggestions`: Vector of suggestions.
    pub fn new(auxiliary: String, suggestions: Vec<String>) -> Self {
        Suggestion { auxiliary, suggestions }
    }

    /// Constructs an empty `Suggestion` struct.
    pub fn empty() -> Self {
        Suggestion { auxiliary: String::new(), suggestions: Vec::new() }
    }

    /// Returns `true` if the `Suggestion` struct is empty.
    pub fn is_empty(&self) -> bool {
        self.suggestions.is_empty()
    }
}