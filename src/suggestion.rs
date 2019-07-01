/// Suggestions which are intend to be shown by the IM's candidate window.
pub struct Suggestion {
    // Auxiliary text
    auxiliary: String,
    // This is Some() when the `Suggestion` is a *lonely* one. 
    suggestion: Option<String>,
    suggestions: Vec<String>,
}

impl Suggestion {
    /// Creates a new `Suggestion` struct with given arguments.
    /// 
    /// `auxiliary`: The auxiliary text.
    /// 
    /// `suggestions`: Vector of suggestions.
    pub fn new(auxiliary: String, suggestions: Vec<String>) -> Self {
        Suggestion { auxiliary, suggestion: None, suggestions }
    }

    /// Creates a new `Suggestion` struct with only one suggestion.
    /// 
    /// *A lonely suggestion.* 😁
    /// 
    /// `suggestion`: The suggestion.
    pub fn new_lonely(suggestion: String) -> Self {
        Suggestion { auxiliary: String::new(), suggestion: Some(suggestion), suggestions: Vec::new() }
    }

    /// Constructs an empty `Suggestion` struct.
    pub fn empty() -> Self {
        Suggestion { auxiliary: String::new(), suggestion: None, suggestions: Vec::new() }
    }

    /// Returns `true` when the `Suggestion` struct is a **lonely** one, otherwise returns `false`.
    /// 
    /// A *lonely* `Suggestion` struct means that the struct has only one suggestion.
    pub fn is_lonely(&self) -> bool {
        self.suggestion.is_some()
    }

    /// Returns `true` if the `Suggestion` struct is empty.
    pub fn is_empty(&self) -> bool {
        if self.suggestion.is_some() {
            self.suggestion.as_ref().unwrap().is_empty()
        } else {
            self.suggestions.is_empty()
        }
    }

    /// Get the suggestions as a slice.
    pub fn get_suggestions(&self) -> &[String] {        
        self.suggestions.as_slice()
    }

    /// Get the only suggestion of the *lonely* `Suggestion`.
    pub fn get_lonely_suggestion(&self) -> &str {        
        self.suggestion.as_ref().unwrap()
    }

    /// Get the auxiliary text.
    pub fn get_auxiliary_text(&self) -> &str {
        &self.auxiliary
    }

    /// Get the length of the suggestions contained.
    pub fn len(&self) -> usize {
        self.suggestions.len()
    }
}
