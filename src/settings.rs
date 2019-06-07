use std::env::var;

pub(crate) const ENV_LAYOUT_FILE: &str = "RITI_LAYOUT_FILE";
pub(crate) const ENV_LAYOUT_FIXED_VOWEL: &str = "RITI_LAYOUT_FIXED_VOWEL";
pub(crate) const ENV_LAYOUT_FIXED_CHANDRA: &str = "RITI_LAYOUT_FIXED_CHANDRA";
pub(crate) const ENV_LAYOUT_FIXED_KAR: &str = "RITI_LAYOUT_FIXED_KAR";

/// Get file path of the selected layout.
pub(crate) fn get_settings_layout_file() -> String {
    var(ENV_LAYOUT_FILE).unwrap()
}

/// Check if the Automatic Vowel Forming of 
/// Fixed Keyboard layout method feature is enabled. 
pub(crate) fn get_settings_fixed_automatic_vowel() -> bool {
    var(ENV_LAYOUT_FIXED_VOWEL).unwrap().parse().unwrap()
}

/// Check if the Automatic Chandra position of 
/// Fixed Keyboard layout method feature is enabled. 
pub(crate) fn get_settings_fixed_automatic_chandra() -> bool {
    var(ENV_LAYOUT_FIXED_CHANDRA).unwrap().parse().unwrap()
}

/// Check if the Traditional Kar Joining of 
/// Fixed Keyboard layout method feature is enabled. 
pub(crate) fn get_settings_fixed_traditional_kar() -> bool {
    var(ENV_LAYOUT_FIXED_KAR).unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use std::env::set_var;
    use super::*;

    #[test]
    fn test_get_bools() {
        set_var(ENV_LAYOUT_FIXED_VOWEL, "true");
        set_var(ENV_LAYOUT_FIXED_CHANDRA, "false");
        set_var(ENV_LAYOUT_FIXED_KAR, "true");

        assert!(get_settings_fixed_automatic_vowel());
        assert!(!get_settings_fixed_automatic_chandra());
        assert!(get_settings_fixed_traditional_kar());
    }
}
