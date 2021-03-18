use std::path::PathBuf;

#[derive(Clone, Default)]
pub struct Config {
    layout: String,
    phonetic_database: bool,
    phonetic_include_english: bool,
    database_dir: PathBuf,
    fixed_database: bool,
    fixed_vowel: bool,
    fixed_chandra: bool,
    fixed_kar: bool,
    fixed_old_reph: bool,
    fixed_numpad: bool,
}

impl Config {
    pub fn new_config(
        layout: String,
        phonetic_database: bool,
        phonetic_include_english: bool,
        database_dir: PathBuf,
        fixed_database: bool,
        fixed_vowel: bool,
        fixed_chandra: bool,
        fixed_kar: bool,
        fixed_old_reph: bool,
        fixed_numpad: bool,
    ) -> Config {
        Config {
            layout,
            phonetic_database,
            phonetic_include_english,
            database_dir,
            fixed_database,
            fixed_vowel,
            fixed_chandra,
            fixed_kar,
            fixed_old_reph,
            fixed_numpad,
        }
    }

    pub(crate) fn get_layout_file_path(&self) -> &str {
        &self.layout
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

    pub(crate) fn get_phonetic_database(&self) -> bool {
        self.phonetic_database
    }

    pub(crate) fn get_phonetic_include_english(&self) -> bool {
        self.phonetic_include_english
    }

    /// Get the config's fixed database.
    pub fn get_fixed_database(&self) -> bool {
        self.fixed_database
    }

    /// Set the config's fixed database.
    pub fn set_fixed_database(&mut self, fixed_database: bool) {
        self.fixed_database = fixed_database;
    }

    /// Get the config's fixed vowel.
    pub fn get_fixed_vowel(&self) -> bool {
        self.fixed_vowel
    }

    /// Set the config's fixed vowel.
    pub fn set_fixed_vowel(&mut self, fixed_vowel: bool) {
        self.fixed_vowel = fixed_vowel;
    }

    /// Get the config's fixed chandra.
    pub fn get_fixed_chandra(&self) -> bool {
        self.fixed_chandra
    }

    /// Set the config's fixed chandra.
    pub fn set_fixed_chandra(&mut self, fixed_chandra: bool) {
        self.fixed_chandra = fixed_chandra;
    }

    /// Get the config's fixed kar.
    pub fn get_fixed_kar(&self) -> bool {
        self.fixed_kar
    }

    /// Set the config's fixed kar.
    pub fn set_fixed_kar(&mut self, fixed_kar: bool) {
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
}

pub(crate) fn phonetic_method_defaults() -> Config {
    Config {
        layout: format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/data/avrophonetic.json"
        ),
        database_dir: format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/data").into(),
        phonetic_database: true,
        phonetic_include_english: true,
        fixed_database: false,
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
        fixed_database: true,
        fixed_vowel: true,
        fixed_chandra: true,
        fixed_kar: true,
        fixed_numpad: true,
        fixed_old_reph: true,
        phonetic_database: false,
        phonetic_include_english: false,
    }
}
