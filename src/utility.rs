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
    let meta = "-]~!@#%&*()_=+[{}'\";<>/?|.,।";
    let mut first_index = 0;
    let mut last_index = input.len();
    let mut encountered_alpha = false;

    for c in input.chars() {
        if !meta.contains(c) {
            encountered_alpha = true;
            break;
        }
        
        first_index += c.len_utf8();
    }

    // Corner case: If we haven't yet encountered an alpha or
    // a numeric character, then the string has no middle part
    // or last part we need. So return "" for them ;)
    if !encountered_alpha {
        return (&input[..], "", "");
    }

    let mut skip_next = false; // Skip the next iteration.

    for (index, c) in input.chars().rev().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        // Check is there a double ` accent character.
        // Accent character can be used to escape tha colon character.
        if c == '`' {
            let next = input.chars().rev().nth(index + 1).unwrap_or_default();
            if next == '`' {
                break;
            } else if next == ':' {
                last_index -= 2;
                skip_next = true;
                continue;
            } else {
                last_index -= 1;
                continue;
            }
        }
        // Include colon as a meta character if `include_colon` is true.
        if include_colon && c == ':' {
            last_index -= 1;
        }

        if !meta.contains(c) {
            break;
        }

        last_index -= c.len_utf8();
    }

    let first_part = &input[0..first_index];
    let middle_part = &input[first_index..last_index];
    let last_part = &input[last_index..];

    (first_part, middle_part, last_part)
}

#[cfg(test)]
mod test {
    use super::{get_modifiers, split_string, Utility};
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
        assert_eq!(split_string("kt``", false), ("", "kt``", ""));
        assert_eq!(split_string("।ঃমেঃ।টাঃ।", false), ("।", "ঃমেঃ।টাঃ", "।"));
    }
}
