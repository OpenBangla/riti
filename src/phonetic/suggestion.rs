// Suggestion making module.

use crate::phonetic::database::Database;
use crate::utility::Utility;

pub(crate) struct PhoneticSuggestion {
    suggestions: Vec<String>,
    database: Database,
}

impl PhoneticSuggestion {
    pub(crate) fn new() -> Self {
        PhoneticSuggestion {
            suggestions: Vec::new(),
            database: Database::new(),
        }
    }

    /// Make suggestion from the given `word`.
    pub(crate) fn suggest(&self, word: &str) -> Vec<String> {
        let splitted_string = word.split_string();
        Vec::new()
    }
}
