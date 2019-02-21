use regex::Regex;

lazy_static! {
    static ref VOWEL: Regex = Regex::new(r#"^[\u0985\u0986\u0987\u0988\u0989\u098A\u098B\u098F\u0990\u0993\u0994\u098C\u09E1\u09BE\u09BF\u09C0\u09C1\u09C2\u09C3\u09C7\u09C8\u09CB\u09CC]$"#).unwrap();
    static ref KAR: Regex = Regex::new(r#"^[\u09BE\u09BF\u09C0\u09C1\u09C2\u09C3\u09C7\u09C8\u09CB\u09CC\u09C4]$"#).unwrap();
    static ref CONSONANT: Regex = Regex::new(r#"^[\u0995\u0996\u0997\u0998\u0999\u099A\u099B\u099C\u099D\u099E\u099F\u09A0\u09A1\u09A2\u09A3\u09A4\u09A5\u09A6\u09A7\u09A8\u09AA\u09AB\u09AC\u09AD\u09AE\u09AF\u09B0\u09B2\u09B6\u09B7\u09B8\u09B9\u09CE\u09DC\u09DD\u09DF]$"#).unwrap();
}

/// Some utility functions which we implement on the `str` type.
pub(crate) trait Utility {
    /// Checks the str for a vowel character.
    fn is_vowel(&self) -> bool;
    /// Checks the str for a kar character.
    fn is_kar(&self) -> bool;
    /// Checks the str for a pure consonant character.
    fn is_pure_consonant(&self) -> bool;
    /// Split the string into three parts.
    /// This function splits preceding and trailing meta characters.
    fn split_string(&self) -> (String, String, String);
}

impl Utility for str {
    /// Checks the str for a vowel character.
    fn is_vowel(&self) -> bool {
        VOWEL.is_match(self)
    }

    /// Checks the str for a kar character.
    fn is_kar(&self) -> bool {
        KAR.is_match(self)
    }

    /// Checks the str for a pure consonant character.
    fn is_pure_consonant(&self) -> bool {
        CONSONANT.is_match(self)
    }

    fn split_string(&self) -> (String, String, String) {
        let meta = "-]~!@#%&*()_=+[{}'\";<>/?|.,";
        let mut first_index = 0;
        let mut last_index = 0;
        let mut encountered_alpha = false;

        for (index, c) in self.chars().enumerate() {
            if !meta.contains(c) {
                first_index = index;
                encountered_alpha = true;
                break;
            }
        }

        // Corner case: If we haven't yet encountered an alpha or
        // a numeric character, then the string has no middle part
        // or last part we need. So return "" for them ;)
        if !encountered_alpha {
            return (self[..].to_owned(), "".to_owned(), "".to_owned());
        }

        for (index, c) in self.chars().rev().enumerate() {
            if !meta.contains(c) {
                last_index = self.len() - index - 1;
                break;
            }
        }

        let first_part = self[0..first_index].to_owned();
        let middle_part = self[first_index..=last_index].to_owned();
        let last_part = self[last_index + 1..].to_owned();

        (first_part, middle_part, last_part)
    }
}

#[cfg(test)]
mod test {
    use super::Utility;
    #[test]
    fn test_utilities() {
        assert!("আ".is_vowel());
        assert!(!"ক".is_vowel());
        assert!("া".is_kar());
        assert!(!"আ".is_kar());
        assert!("ক".is_pure_consonant());
    }

    #[test]
    fn test_split_string() {
        assert_eq!(
            "[][][][]".split_string(),
            ("[][][][]".to_owned(), "".to_owned(), "".to_owned())
        );
        assert_eq!(
            "t*".split_string(),
            ("".to_owned(), "t".to_owned(), "*".to_owned())
        );
        assert_eq!(
            "1".split_string(),
            ("".to_owned(), "1".to_owned(), "".to_owned())
        );
        assert_eq!(
            "#\"percent%sign\"#".split_string(),
            (
                "#\"".to_owned(),
                "percent%sign".to_owned(),
                "\"#".to_owned()
            )
        );
        assert_eq!(
            "text".split_string(),
            ("".to_owned(), "text".to_owned(), "".to_owned())
        );
        assert_eq!(
            ":)".split_string(),
            ("".to_owned(), ":".to_owned(), ")".to_owned())
        );
    }
}
