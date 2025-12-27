use std::{
    env::var,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde_json::Value;

/// Config struct for configuring RitiContext.
#[derive(Clone, Debug)]
pub struct Config {
    layout: String,
    database_dir: PathBuf,
    user_dir: PathBuf,
    include_english: bool,
    phonetic_suggestion: bool,
    fixed_suggestion: bool,
    fixed_vowel: bool,
    fixed_chandra: bool,
    fixed_kar: bool,
    fixed_old_reph: bool,
    fixed_numpad: bool,
    fixed_kar_order: bool,
    // Output in ANSI encoding
    ansi: bool,
    smart_quote: bool,
}

impl Config {
    /// Sets the layout file path.
    /// For Avro Phonetic, it accepts the name `avro_phonetic`.
    ///
    /// Returns `true` if the layout file path or name is valid.
    pub fn set_layout_file_path(&mut self, layout: &str) -> bool {
        if layout == "avro_phonetic" || Path::new(layout).exists() {
            self.layout = layout.into();
            true
        } else {
            false
        }
    }

    pub fn get_layout_file_path(&self) -> &str {
        &self.layout
    }

    /// Sets the database directory path.
    ///
    /// Returns `true` if the path exists.
    pub fn set_database_dir(&mut self, path: &str) -> bool {
        if Path::new(path).exists() {
            self.database_dir = path.into();
            true
        } else {
            false
        }
    }

    /// Sets the user specific writable directory path.
    ///
    /// Returns `true` if the path exists.
    pub fn set_user_dir(&mut self, path: &str) -> bool {
        if Path::new(path).exists() {
            self.user_dir = path.into();
            true
        } else {
            false
        }
    }

    pub fn get_database_dir(&self) -> &PathBuf {
        &self.database_dir
    }

    pub fn get_suffix_data_path(&self) -> PathBuf {
        self.database_dir.join("suffix.json")
    }

    pub fn get_autocorrect_data(&self) -> PathBuf {
        self.database_dir.join("autocorrect.json")
    }

    /// Get file path of user defined Auto Correct file.
    pub fn get_user_phonetic_autocorrect(&self) -> PathBuf {
        self.user_dir.join("autocorrect.json")
    }

    /// Get file path of user defined phonetic candidate selection file.
    pub fn get_user_phonetic_selection_data(&self) -> PathBuf {
        self.user_dir.join("phonetic-candidate-selection.json")
    }

    pub fn get_suggestion_include_english(&self) -> bool {
        // Mutually exclusive
        self.include_english && !self.ansi
    }

    pub fn set_suggestion_include_english(&mut self, include: bool) {
        self.include_english = include;
    }

    pub fn get_phonetic_suggestion(&self) -> bool {
        self.phonetic_suggestion
    }

    /// Set the config's phonetic database.
    pub fn set_phonetic_suggestion(&mut self, phonetic_suggestion: bool) {
        self.phonetic_suggestion = phonetic_suggestion;
    }

    /// Get the config's fixed database.
    pub fn get_fixed_suggestion(&self) -> bool {
        self.fixed_suggestion
    }

    /// Set the config's fixed database.
    pub fn set_fixed_suggestion(&mut self, fixed_suggestion: bool) {
        self.fixed_suggestion = fixed_suggestion;
    }

    /// Get the config's fixed vowel.
    pub fn get_fixed_automatic_vowel(&self) -> bool {
        self.fixed_vowel
    }

    /// Set the config's fixed vowel.
    pub fn set_fixed_automatic_vowel(&mut self, fixed_vowel: bool) {
        self.fixed_vowel = fixed_vowel;
    }

    /// Get the config's fixed chandra.
    pub fn get_fixed_automatic_chandra(&self) -> bool {
        self.fixed_chandra
    }

    /// Set the config's fixed chandra.
    pub fn set_fixed_automatic_chandra(&mut self, fixed_chandra: bool) {
        self.fixed_chandra = fixed_chandra;
    }

    /// Get the config's fixed kar.
    pub fn get_fixed_traditional_kar(&self) -> bool {
        self.fixed_kar
    }

    /// Set the config's fixed kar.
    pub fn set_fixed_traditional_kar(&mut self, fixed_kar: bool) {
        self.fixed_kar = fixed_kar;
    }

    /// Get the config's fixed old reph.
    pub fn get_fixed_old_reph(&self) -> bool {
        self.fixed_old_reph
    }

    /// Set the config's fixed old reph.
    pub fn set_fixed_old_reph(&mut self, fixed_old_reph: bool) {
        self.fixed_old_reph = fixed_old_reph;
    }

    /// Get the config's fixed numpad.
    pub fn get_fixed_numpad(&self) -> bool {
        self.fixed_numpad
    }

    /// Set the config's fixed numpad.
    pub fn set_fixed_numpad(&mut self, fixed_numpad: bool) {
        self.fixed_numpad = fixed_numpad;
    }

    /// Get the config's fixed kar order.
    pub fn get_fixed_old_kar_order(&self) -> bool {
        self.fixed_kar_order
    }

    /// Set the config's fixed kar order.
    pub fn set_fixed_old_kar_order(&mut self, fixed_kar_order: bool) {
        self.fixed_kar_order = fixed_kar_order;
    }

    /// Checks if the layout path had changed.
    pub fn layout_changed(&self, new_config: &Self) -> bool {
        self.layout != new_config.layout
    }

    /// Checks if the layout is phonetic
    pub fn is_phonetic(&self) -> bool {
        self.get_layout_file_path() == "avro_phonetic"
    }

    /// Give layout's `layout` json object, which contains the layout data.
    pub fn get_layout(&self) -> Option<Value> {
        if self.is_phonetic() {
            None
        } else {
            read_to_string(self.get_layout_file_path())
                .ok()
                .and_then(|s| serde_json::from_str::<Value>(&s).ok())
                .map(|v| v["layout"].to_owned())
        }
    }

    /// Checks if ANSI encoding is enabled.
    pub fn get_ansi_encoding(&self) -> bool {
        self.ansi
    }

    /// Set the ANSI encoding configuration.
    pub fn set_ansi_encoding(&mut self, ansi: bool) {
        self.ansi = ansi;
    }

    /// Get the config's smart quote configuration.
    #[must_use]
    pub fn get_smart_quote(&self) -> bool {
        self.smart_quote
    }

    /// Set the config's smart quote.
    pub fn set_smart_quote(&mut self, smart_quote: bool) {
        self.smart_quote = smart_quote;
    }
}

pub fn get_user_data_dir() -> PathBuf {
    var("XDG_DATA_HOME")
        .ok()
        .or_else(|| var("HOME").ok().map(|path| path + "/.local/share"))
        .map(|path| path + "/openbangla-keyboard")
        .or_else(|| {
            // Windows
            var("localappdata")
                .ok()
                .map(|path| path + "\\OpenBangla")
        })
        .unwrap()
        .into()
}

#[cfg(test)]
pub fn get_phonetic_method_defaults() -> Config {
    Config {
        layout: "avro_phonetic".to_owned(),
        database_dir: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data").into(),
        phonetic_suggestion: true,
        ..Default::default()
    }
}

#[cfg(test)]
pub fn get_fixed_method_defaults() -> Config {
    Config {
        layout: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/Probhat.json"),
        database_dir: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data").into(),
        fixed_suggestion: true,
        fixed_vowel: true,
        fixed_chandra: true,
        fixed_kar: true,
        fixed_numpad: true,
        fixed_old_reph: true,
        fixed_kar_order: false,
        ..Default::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            layout: Default::default(),
            database_dir: Default::default(),
            user_dir: get_user_data_dir(),
            include_english: false,
            fixed_suggestion: false,
            fixed_vowel: false,
            fixed_chandra: false,
            fixed_kar: false,
            fixed_numpad: false,
            fixed_old_reph: false,
            fixed_kar_order: false,
            phonetic_suggestion: false,
            ansi: false,
            smart_quote: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_data_dir_linux() {
        assert_eq!(
            get_user_data_dir(),
            PathBuf::from(var("HOME").unwrap() + "/.local/share/openbangla-keyboard")
        );
        std::env::set_var("XDG_DATA_HOME", "/non/existent");
        assert_eq!(
            get_user_data_dir(),
            PathBuf::from("/non/existent/openbangla-keyboard")
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_data_dir_windows() {
        assert_eq!(
            get_user_data_dir(),
            PathBuf::from(var("localappdata").unwrap() + "\\OpenBangla")
        )
    }

    #[test]
    fn test_mutually_exclusive() {
        let mut config = Config::default();

        config.set_suggestion_include_english(true);
        config.set_ansi_encoding(false);
        assert!(config.get_suggestion_include_english());

        config.set_ansi_encoding(true);
        assert!(!config.get_suggestion_include_english());
    }

    #[test]
    fn test_path_validation() {
        let mut config = Config::default();

        assert!(!config.set_layout_file_path("non_existent"));
        assert!(config.set_layout_file_path("avro_phonetic"));

        assert!(!config.set_layout_file_path("/non_existent/Probhat.json"));
        assert!(config.set_layout_file_path(&format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/data/Probhat.json"
        )));

        assert!(!config.set_database_dir("/non_existent"));
        assert!(config.set_database_dir(&format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data")));
    }
}
