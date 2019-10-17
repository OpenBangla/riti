use either::Either;

/// Suggestions which are intend to be shown by the IM's candidate window.
#[derive(Debug)]
pub struct Suggestion {
    // Auxiliary text
    auxiliary: String,
    // Suggestion is of two variants, the 'normal' one includes a list of suggestion and
    // the 'lonely' one is just a String.
    suggestion: Either<Vec<String>, String>,
    // Index of the previously selected suggestion.
    selection: usize,
}

impl Suggestion {
    /// Creates a new `Suggestion` struct with given arguments.
    /// 
    /// `auxiliary`: The auxiliary text.
    /// 
    /// `suggestions`: Vector of suggestions.
    /// 
    /// `selection`: Index of the previously selected suggestion.
    pub fn new(auxiliary: String, suggestions: Vec<String>, selection: usize) -> Self {
        Suggestion { auxiliary, suggestion: Either::Left(suggestions), selection }
    }

    /// Creates a new `Suggestion` struct with only one suggestion.
    /// 
    /// *A lonely suggestion.* 😁
    /// 
    /// `suggestion`: The suggestion.
    pub fn new_lonely(suggestion: String) -> Self {
        Suggestion { auxiliary: String::new(), suggestion: Either::Right(suggestion), selection: 0 }
    }

    /// Constructs an empty `Suggestion` struct.
    pub fn empty() -> Self {
        Suggestion { auxiliary: String::new(), suggestion: Either::Right(String::new()), selection: 0 }
    }

    /// Returns `true` when the `Suggestion` struct is a **lonely** one, otherwise returns `false`.
    /// 
    /// A *lonely* `Suggestion` struct means that the struct has only one suggestion.
    pub fn is_lonely(&self) -> bool {
        self.suggestion.is_right()
    }

    /// Returns `true` if the `Suggestion` struct is empty.
    pub fn is_empty(&self) -> bool {
        match &self.suggestion {
            Either::Left(list) => list.is_empty(),
            Either::Right(suggestion) => suggestion.is_empty()
        }
    }

    /// Get the suggestions as a slice.
    pub fn get_suggestions(&self) -> &[String] {        
        self.suggestion.as_ref().left().unwrap()
    }

    /// Get the only suggestion of the *lonely* `Suggestion`.
    pub fn get_lonely_suggestion(&self) -> &str {        
        self.suggestion.as_ref().right().unwrap()
    }

    /// Get the auxiliary text.
    pub fn get_auxiliary_text(&self) -> &str {
        &self.auxiliary
    }

    /// Returns index of the suggestion, which was previously selected.
    pub fn previously_selected_index(&self) -> usize {
        self.selection
    }

    /// Get the length of the suggestions contained.
    pub fn len(&self) -> usize {
        self.suggestion.as_ref().left().unwrap().len()
    }
}
