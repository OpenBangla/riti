// Suggestion making module.

use crate::phonetic::database::Database;
use regex::Regex;

// A regex for splitting the string into three parts.
lazy_static! {
    static ref SPLIT_STRING: Regex = Regex::new(r#"((^(?::`|\.`|[-\]\\~!@#&*()_=+\[{}'";<>/?|.,])*?(?=(?:,{2,}))|^(?::`|\.`|[-\]\\~!@#&*()_=+\[{}'";<>/?|.,])*)(.*?(?:,,)*)((?::`|\.`|[-\]\\~!@#&*()_=+\[{}'";<>/?|.,])*$))"#).unwrap();
}

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
    pub(crate) fn make_suggestion(&self, word: &str) -> Vec<String> {
        let splitted_string = {
            let caps = SPLIT_STRING.captures(word).unwrap();
            // We are splitting the string into three parts.
            // We take out preceding and trailing meta characters,
            // those are captured in the first and third match.
            // These two match could fail so we use a default "" value.
            (
                caps.get(1).map_or("", |m| m.as_str()),
                caps.get(2).unwrap().as_str(),
                caps.get(3).map_or("", |m| m.as_str()),
            )
        };
        Vec::new()
    }
}
