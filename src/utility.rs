/// Some utility functions which we implement on the `char` type.
pub(crate) trait Utility {
    /// Checks the char for a vowel character.
    fn is_vowel(&self) -> bool;
    /// Checks the char for a kar character.
    fn is_kar(&self) -> bool;
    /// Checks the char for a pure consonant character.
    fn is_pure_consonant(&self) -> bool;
}

impl Utility for char {
    /// Checks the char for a vowel character.
    fn is_vowel(&self) -> bool {
        "\u{0985}\u{0986}\u{0987}\u{0988}\u{0989}\u{098A}\u{098B}\u{098F}\u{0990}\u{0993}\u{0994}\u{098C}\u{09E1}\u{09BE}\u{09BF}\u{09C0}\u{09C1}\u{09C2}\u{09C3}\u{09C7}\u{09C8}\u{09CB}\u{09CC}".contains(*self)
    }

    /// Checks the char for a kar character.
    fn is_kar(&self) -> bool {
        "\u{09BE}\u{09BF}\u{09C0}\u{09C1}\u{09C2}\u{09C3}\u{09C7}\u{09C8}\u{09CB}\u{09CC}\u{09C4}".contains(*self)
    }

    /// Checks the char for a pure consonant character.
    fn is_pure_consonant(&self) -> bool {
        "\u{0995}\u{0996}\u{0997}\u{0998}\u{0999}\u{099A}\u{099B}\u{099C}\u{099D}\u{099E}\u{099F}\u{09A0}\u{09A1}\u{09A2}\u{09A3}\u{09A4}\u{09A5}\u{09A6}\u{09A7}\u{09A8}\u{09AA}\u{09AB}\u{09AC}\u{09AD}\u{09AE}\u{09AF}\u{09B0}\u{09B2}\u{09B6}\u{09B7}\u{09B8}\u{09B9}\u{09CE}\u{09DC}\u{09DD}\u{09DF}".contains(*self)
    }
}

#[cfg(test)]
mod test {
    use super::Utility;
    #[test]
    fn test_utilities() {
        assert!('আ'.is_vowel());
        assert!(!'ক'.is_vowel());
        assert!('া'.is_kar());
        assert!(!'আ'.is_kar());
        assert!('ক'.is_pure_consonant());
    }
}
