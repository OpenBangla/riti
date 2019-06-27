// Auto Correct database loading.

use std::fs::read_to_string;
use rustc_hash::FxHashMap;

use crate::settings::*;

/// Auto Correct dictionary loading.
pub(crate) struct AutoCorrect {
    // Main Auto Correct database.
    database: FxHashMap<String, String>,
    // User defined Auto Correct database.
    user: FxHashMap<String, String>,
}

impl AutoCorrect {
    /// A new `AutoCorrect` instance.
    /// 
    /// Main AutoCorrect dictionary is loaded from the path defined in `RITI_PHONETIC_AUTOCORRECT` environmental variable.
    /// 
    /// User defined AutoCorrect dictionary is loaded from `$XDG_DATA_HOME/openbangla-keyboard/autocorrect.json` path.
    pub(crate) fn new() -> Self {
        //TODO
        //let database = serde_json::from_str(&read_to_string(get_settings_phonetic_autocorrect()).unwrap()).unwrap();
        let database = serde_json::from_str(include_str!("autocorrect.json")).unwrap();

        let user = if let Ok(file) = read_to_string(get_settings_user_phonetic_autocorrect()) {
            serde_json::from_str(&file).unwrap()
        } else {
            FxHashMap::default()
        };

        AutoCorrect { database, user }
    }

    /// Search for a `term` in AutoCorrect dictionary.
    /// We first search in the user defined 
    pub(crate) fn search(&self, term: &str) -> Option<String> {
        self.user.get(term).cloned().or_else(|| self.database.get(term).cloned())
    }

    /// Update the user defined AutoCorrect dictionary.
    pub(crate) fn update(&mut self) {
        self.user = if let Ok(file) = read_to_string(get_settings_user_phonetic_autocorrect()) {
            serde_json::from_str(&file).unwrap()
        } else {
            FxHashMap::default()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::AutoCorrect;

    #[test]
    fn test_autocorrect() {
        let db = AutoCorrect::new();

        assert_eq!(db.search("academy"), Some("oZakaDemi".to_string()));
        assert_eq!(db.search("\\nai\\"), None);
    }
}
