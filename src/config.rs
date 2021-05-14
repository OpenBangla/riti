use std::{env::var, path::PathBuf};

/// Config struct for configuring RitiContext.
#[derive(Clone, Default)]
pub struct Config {
    layout: String,
    database_dir: PathBuf,
    user_dir: PathBuf,
    phonetic_suggestion: bool,
    phonetic_include_english: bool,
    fixed_suggestion: bool,
    fixed_include_english: bool,
    fixed_vowel: bool,
    fixed_chandra: bool,
    fixed_kar: bool,
    fixed_old_reph: bool,
    fixed_numpad: bool,
}

impl Config {
    pub fn new_config(
        layout: String,
        phonetic_suggestion: bool,
        phonetic_include_english: bool,
        database_dir: PathBuf,
        fixed_suggestion: bool,
        fixed_include_english: bool,
        fixed_vowel: bool,
        fixed_chandra: bool,
        fixed_kar: bool,
        fixed_old_reph: bool,
        fixed_numpad: bool,
    ) -> Config {
        Config {
            layout,
            user_dir: get_user_data_dir(),
            phonetic_suggestion,
            phonetic_include_english,
            database_dir,
            fixed_suggestion,
            fixed_include_english,
            fixed_vowel,
            fixed_chandra,
            fixed_kar,
            fixed_old_reph,
            fixed_numpad,
        }
    }

    pub(crate) fn set_layout_file_path(&mut self, layout: &str) {
        self.layout = layout.to_string();
    }

    pub(crate) fn get_layout_file_path(&self) -> &str {
        &self.layout
    }

    pub(crate) fn set_database_dir(&mut self, path: &str) {
        self.database_dir = path.into();
    }

    pub(crate) fn get_database_path(&self) -> PathBuf {
        self.database_dir.join("dictionary.json")
    }

    pub(crate) fn get_suffix_data_path(&self) -> PathBuf {
        self.database_dir.join("suffix.json")
    }

    pub(crate) fn get_autocorrect_data(&self) -> PathBuf {
        self.database_dir.join("autocorrect.json")
    }

    /// Get file path of user defined Auto Correct file.
    pub(crate) fn get_user_phonetic_autocorrect(&self) -> PathBuf {
        self.user_dir.join("autocorrect.json")
    }

    /// Get file path of user defined phonetic candidate selection file.
    pub(crate) fn get_user_phonetic_selection_data(&self) -> PathBuf {
        self.user_dir.join("phonetic-candidate-selection.json")
    }

    pub(crate) fn get_phonetic_suggestion(&self) -> bool {
        self.phonetic_suggestion
    }

    /// Set the config's phonetic database.
    pub(crate) fn set_phonetic_suggestion(&mut self, phonetic_suggestion: bool) {
        self.phonetic_suggestion = phonetic_suggestion;
    }

    pub(crate) fn get_phonetic_include_english(&self) -> bool {
        self.phonetic_include_english
    }

    pub(crate) fn set_phonetic_include_english(&mut self, include: bool) {
        self.phonetic_include_english = include;
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

    /// Get the config's fixed include english.
    pub fn get_fixed_include_english(&self) -> bool {
        self.fixed_include_english
    }

    /// Set the config's fixed include english.
    pub fn set_fixed_include_english(&mut self, fixed_include_english: bool) {
        self.fixed_include_english = fixed_include_english;
    }
}

pub(crate) fn get_user_data_dir() -> PathBuf {
    var("XDG_DATA_HOME")
        .ok()
        .or_else(|| var("HOME").ok().map(|path| path + "/.local/share"))
        .map(|path| path + "/openbangla-keyboard")
        .or_else(|| {
            // Windows
            var("localappdata")
                .ok()
                .map(|path| path + "/OpenBangla Keyboard")
        })
        .unwrap()
        .into()
}

pub(crate) fn get_phonetic_method_defaults() -> Config {
    Config {
        layout: format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/data/avrophonetic.json"
        ),
        user_dir: get_user_data_dir(),
        database_dir: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data").into(),
        phonetic_suggestion: true,
        phonetic_include_english: false,
        fixed_suggestion: false,
        fixed_include_english: false,
        fixed_vowel: false,
        fixed_chandra: false,
        fixed_kar: false,
        fixed_numpad: false,
        fixed_old_reph: false,
    }
}

pub(crate) fn get_fixed_method_defaults() -> Config {
    Config {
        layout: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data/Probhat.json"),
        database_dir: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data").into(),
        user_dir: get_user_data_dir(),
        fixed_suggestion: true,
        fixed_include_english: false,
        fixed_vowel: true,
        fixed_chandra: true,
        fixed_kar: true,
        fixed_numpad: true,
        fixed_old_reph: true,
        phonetic_suggestion: false,
        phonetic_include_english: false,
    }
}
