use std::{borrow::Cow, fs::File, io::Read, ops::Deref};

use crate::context::{MODIFIER_ALT_GR, MODIFIER_SHIFT};

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
        "\u{09BE}\u{09BF}\u{09C0}\u{09C1}\u{09C2}\u{09C3}\u{09C7}\u{09C8}\u{09CB}\u{09CC}\u{09C4}"
            .contains(*self)
    }

    /// Checks the char for a pure consonant character.
    fn is_pure_consonant(&self) -> bool {
        "\u{0995}\u{0996}\u{0997}\u{0998}\u{0999}\u{099A}\u{099B}\u{099C}\u{099D}\u{099E}\u{099F}\u{09A0}\u{09A1}\u{09A2}\u{09A3}\u{09A4}\u{09A5}\u{09A6}\u{09A7}\u{09A8}\u{09AA}\u{09AB}\u{09AC}\u{09AD}\u{09AE}\u{09AF}\u{09B0}\u{09B2}\u{09B6}\u{09B7}\u{09B8}\u{09B9}\u{09CE}\u{09DC}\u{09DD}\u{09DF}".contains(*self)
    }
}

/// Checks if the `vec` already has the `value` before inserting.
/// If it does, then the `value` is not inserted.
pub(crate) fn push_checked<T: PartialEq>(vec: &mut Vec<T>, value: T) {
    if !vec.contains(&value) {
        vec.push(value);
    }
}

/// Tuple of modifier keys.
///
/// First  is Shift, second is AltGr.
pub(crate) type Modifiers = (bool, bool);

/// Returns boolean tuples of the modifiers from the bit masked integer `modifier`.
/// First  is Shift, second is Ctrl and third is Alt.
pub(crate) fn get_modifiers(modifier: u8) -> Modifiers {
    let shift = (modifier & MODIFIER_SHIFT) == MODIFIER_SHIFT;
    let alt_gr = (modifier & MODIFIER_ALT_GR) == MODIFIER_ALT_GR;

    (shift, alt_gr)
}

/// Split the string into three parts.
/// This function splits preceding and trailing meta characters.
///
/// `include_colon` argument controls the inclusion of colon as a trailing meta character.
pub(crate) fn split_string(input: &str, include_colon: bool) -> (&str, &str, &str) {
    const META: &str = "-]~!@#%&*()_=+[{}'\";<>/?|.,।";

    let first_index = match input.find(|c| !META.contains(c)) {
        Some(i) => i,
        None => {
            // If no non-META/alphanumeric char is found,
            // the string has no middle or last part
            return (input, "", "");
        }
    };
    let (first_part, rest) = input.split_at(first_index);

    let mut escape = false;
    let mut last_index = rest.len();
    for (i, c) in rest.char_indices().rev() {
        if !escape && c == '`' {
            // escape
            escape = true; // not updating the last index for escape
        } else if ((include_colon || escape) && c == ':') || META.contains(c) {
            // meta
            escape = false;
            last_index = i;
        } else {
            // alphanumeric
            break;
        }
    }
    let (middle_part, last_part) = rest.split_at(last_index);

    (first_part, middle_part, last_part)
}

/// Read the entire contents of a file into a bytes vector.
///
/// Optimized to allocate the required amount of capacity beforehand.
pub(crate) fn read(file: &mut File) -> Vec<u8> {
    let len = file.metadata().map(|m| m.len() + 1).unwrap();
    let mut buf = Vec::with_capacity(len as usize);
    file.read_to_end(&mut buf).unwrap();
    buf
}

#[derive(Debug)]
pub(crate) struct SplittedString<'a> {
    preceding: Cow<'a, str>,
    middle: &'a str,
    trailing: Cow<'a, str>,
}

impl SplittedString<'_> {
    pub(crate) fn split(input: &str, include_colon: bool) -> SplittedString {
        const META: &str = "-]~!@#%&*()_=+[{}'\";<>/?|.,।";

        let first_index = match input.find(|c| !META.contains(c)) {
            Some(i) => i,
            None => {
                // If no non-META/alphanumeric char is found,
                // the string has no middle or last part
                return SplittedString {
                    preceding: input.into(),
                    middle: "",
                    trailing: "".into(),
                };
            }
        };
        let (preceding, rest) = input.split_at(first_index);

        let mut escape = false;
        let mut last_index = rest.len();
        for (i, c) in rest.char_indices().rev() {
            if !escape && c == '`' {
                // escape
                escape = true; // not updating the last index for escape
            } else if ((include_colon || escape) && c == ':') || META.contains(c) {
                // meta
                escape = false;
                last_index = i;
            } else {
                // alphanumeric
                break;
            }
        }
        let (middle, trailing) = rest.split_at(last_index);

        SplittedString {
            preceding: preceding.into(),
            middle,
            trailing: trailing.into(),
        }
    }

    pub(crate) fn map(&mut self, func: impl Fn(&str, &str) -> (String, String)) {
        let (p, t) = (func)(self.preceding.deref(), self.trailing.deref());
        self.preceding = Cow::Owned(p);
        self.trailing = Cow::Owned(t);
    }

    pub(crate) fn preceding(&self) -> &str {
        self.preceding.deref()
    }

    pub(crate) fn middle(&self) -> &str {
        self.middle
    }

    pub(crate) fn trailing(&self) -> &str {
        self.trailing.deref()
    }
}

impl PartialEq<(&str, &str, &str)> for SplittedString<'_> {
    fn eq(&self, other: &(&str, &str, &str)) -> bool {
        self.preceding == other.0 && self.middle == other.1 && self.trailing == other.2
    }
}

/// Convert preceding and trailing quotation marks(', ") into its curved form(‘, ’ “, ”) aka Smart Quote.
pub(crate) fn smart_quoter(
    mut splitted: SplittedString,
) -> SplittedString {
    // If the middle part is empty, there is no need to convert.
    if splitted.middle().is_empty() {
        return splitted;
    }

    // Convert preceding quotation mark(', ") into its curved form(‘, “).
    let mut preceding = String::with_capacity(splitted.preceding().len() + 3);
    for ch in splitted.preceding().chars() {
        match ch {
            '\'' => {
                preceding.push_str("‘");
            }
            '"' => {
                preceding.push_str("“");
            }
            _ => preceding.push(ch),
        }
    }

    // Convert trailing quotation mark(', ") into its curved form(’, ”).
    let mut trailing = String::with_capacity(splitted.trailing().len() + 3);
    for ch in splitted.trailing.chars() {
        match ch {
            '\'' => {
                trailing.push_str("’");
            }
            '"' => {
                trailing.push_str("”");
            }
            _ => trailing.push(ch),
        }
    }

    splitted.preceding = Cow::Owned(preceding);
    splitted.trailing = Cow::Owned(trailing);
    return splitted;
}

#[cfg(test)]
mod test {
    use super::{get_modifiers, smart_quoter, SplittedString, split_string, Utility};
    use crate::context::{MODIFIER_ALT_GR, MODIFIER_SHIFT};

    #[test]
    fn test_utilities() {
        assert!('আ'.is_vowel());
        assert!(!'ক'.is_vowel());
        assert!('া'.is_kar());
        assert!(!'আ'.is_kar());
        assert!('ক'.is_pure_consonant());
    }

    #[test]
    fn test_get_modifiers() {
        assert_eq!(get_modifiers(MODIFIER_SHIFT), (true, false));
        assert_eq!(get_modifiers(MODIFIER_ALT_GR), (false, true));
        assert_eq!(
            get_modifiers(MODIFIER_SHIFT | MODIFIER_ALT_GR),
            (true, true)
        );
    }

    #[test]
    fn test_splitted_string() {
        assert_eq!(SplittedString::split("[][][][]", false), ("[][][][]", "", ""));
        assert_eq!(SplittedString::split("t*", false), ("", "t", "*"));
        assert_eq!(SplittedString::split("1", false), ("", "1", ""));
        assert_eq!(
            SplittedString::split("#\"percent%sign\"#", false),
            ("#\"", "percent%sign", "\"#")
        );
        assert_eq!(SplittedString::split("*[মেটা]*", false), ("*[", "মেটা", "]*"));
        assert_eq!(SplittedString::split("text", false), ("", "text", ""));
        assert_eq!(SplittedString::split("kt:", false), ("", "kt:", ""));
        assert_eq!(SplittedString::split("kt:", true), ("", "kt", ":"));
        assert_eq!(SplittedString::split("kt:`", false), ("", "kt", ":`"));
        assert_eq!(SplittedString::split("kt:`", true), ("", "kt", ":`"));
        assert_eq!(SplittedString::split("kt::`", false), ("", "kt:", ":`"));
        assert_eq!(SplittedString::split("kt::`", true), ("", "kt", "::`"));
        assert_eq!(SplittedString::split("kt``", false), ("", "kt``", ""));
        assert_eq!(SplittedString::split("kt:``", false), ("", "kt:``", ""));
        assert_eq!(SplittedString::split("।ঃমেঃ।টাঃ।", false), ("।", "ঃমেঃ।টাঃ", "।"));
    }

    #[test]
    fn test_split_string() {
        assert_eq!(split_string("[][][][]", false), ("[][][][]", "", ""));
        assert_eq!(split_string("t*", false), ("", "t", "*"));
        assert_eq!(split_string("1", false), ("", "1", ""));
        assert_eq!(
            split_string("#\"percent%sign\"#", false),
            ("#\"", "percent%sign", "\"#")
        );
        assert_eq!(split_string("*[মেটা]*", false), ("*[", "মেটা", "]*"));
        assert_eq!(split_string("text", false), ("", "text", ""));
        assert_eq!(split_string("kt:", false), ("", "kt:", ""));
        assert_eq!(split_string("kt:", true), ("", "kt", ":"));
        assert_eq!(split_string("kt:`", false), ("", "kt", ":`"));
        assert_eq!(split_string("kt:`", true), ("", "kt", ":`"));
        assert_eq!(split_string("kt::`", false), ("", "kt:", ":`"));
        assert_eq!(split_string("kt::`", true), ("", "kt", "::`"));
        assert_eq!(split_string("kt``", false), ("", "kt``", ""));
        assert_eq!(split_string("kt:``", false), ("", "kt:``", ""));
        assert_eq!(split_string("।ঃমেঃ।টাঃ।", false), ("।", "ঃমেঃ।টাঃ", "।"));
    }

    #[test]
    fn test_smart_quoting() {
        assert_eq!(smart_quoter(SplittedString::split("\"", true)), ("\"".into(), "", "".into()));

        assert_eq!(smart_quoter(SplittedString::split("'Till", true)), ("‘".into(), "Till", "".into()));
        assert_eq!(smart_quoter(SplittedString::split("\"Hey", true)), ("“".into(), "Hey", "".into()));
        assert_eq!(smart_quoter(SplittedString::split("'\"Hey", true)), ("‘“".into(), "Hey", "".into()));

        assert_eq!(smart_quoter(SplittedString::split("finished'", true)), ("".into(), "finished", "’".into()));
        assert_eq!(smart_quoter(SplittedString::split("Hey\"", true)), ("".into(), "Hey", "”".into()));
        assert_eq!(smart_quoter(SplittedString::split("Hey'\"", true)), ("".into(), "Hey", "’”".into()));

        assert_eq!(smart_quoter(SplittedString::split("'Awkward'", true)), ("‘".into(), "Awkward", "’".into()));
        assert_eq!(smart_quoter(SplittedString::split("\"Nevertheless\"", true)), ("“".into(), "Nevertheless", "”".into()));

        assert_eq!(smart_quoter(SplittedString::split("\"'Quotation'\"", true)), ("“‘".into(), "Quotation", "’”".into()));
    }
}
