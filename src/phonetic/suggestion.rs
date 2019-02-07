// Suggestion making module.

use crate::phonetic::database::Database;

pub(crate) struct PhoneticSuggestion {
    suggestions: Vec<String>,
    database: Database,
}