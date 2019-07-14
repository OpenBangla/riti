/// Suggestions which are intend to be shown by the IM's candidate window.
pub struct Suggestion<'a> {
    // Auxiliary text
    auxiliary: &'a str,
    // This is Some() when the `Suggestion` is a *lonely* one. 
    suggestion: Option<&'a str>,
    suggestions: Option<&'a [String]>,
}

impl<'a> Suggestion<'a> {
    /// Creates a new `Suggestion` struct with given arguments.
    /// 
    /// `auxiliary`: The auxiliary text.
    /// 
    /// `suggestions`: Vector of suggestions.
    pub fn new(auxiliary: &'a str, suggestions: &'a [String]) -> Self {
        Suggestion { auxiliary, suggestion: None, suggestions: Some(suggestions) }
    }

    /// Creates a new `Suggestion` struct with only one suggestion.
    /// 
    /// *A lonely suggestion.* 😁
    /// 
    /// `suggestion`: The suggestion.
    pub fn new_lonely(suggestion: &'a str) -> Self {
        Suggestion { auxiliary: "", suggestion: Some(suggestion), suggestions: None }
    }

    /// Constructs an empty `Suggestion` struct.
    pub fn empty() -> Self {
        Suggestion { auxiliary: "", suggestion: None, suggestions: None }
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
            self.suggestion.unwrap().is_empty()
        } else {
            self.suggestions.map_or(true, |v| v.is_empty())
        }
    }

    /// Get the suggestions as a slice.
    pub fn get_suggestions(&self) -> &[String] {        
        self.suggestions.unwrap()
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
        self.suggestions.map_or(0, |v| v.len())
    }
}
