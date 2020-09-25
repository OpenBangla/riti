#![allow(unused_assignments)]
use std::cmp::Ordering;
use stringplus::StringPlus;

use super::patterns::{Scope, Type, CONSONANT, IGNORE, MAX_PATTERN_LEN, PATTERNS, VOWELS};

/// Parse `input` string containing phonetic text and return a regex string.
pub(crate) fn parse(input: &str) -> String {
    let fixed = clean_string(input);
    let len = fixed.len();

    let mut output = String::with_capacity(len * 60);
    output.push('^'); // Regex beginning mark.

    let mut cur = 0;
    while cur < len {
        let start = cur as i32;
        let mut end: i32 = 0;
        let mut matched = false;

        for chunk_len in (1..=MAX_PATTERN_LEN).rev() {
            end = start + chunk_len as i32;
            if end <= len as i32 {
                let chunk = &fixed[start as usize..end as usize];

                // Binary Search
                let mut left: i32 = 0;
                let mut right = PATTERNS.len() as i32 - 1;
                let mut mid: i32 = 0;
                while right >= left {
                    mid = (right + left) / 2;
                    let pattern = &PATTERNS[mid as usize];
                    let find = pattern.find;
                    if find == chunk {
                        let rules = pattern.rules;
                        if !rules.is_empty() {
                            for rule in rules {
                                let mut replace = true;
                                let mut chk = 0;
                                let matches = rule.matches;
                                for _match in matches {
                                    let value = _match.value;
                                    let _type = _match.type_;
                                    let scope = _match.scope;
                                    let is_negative = _match.negative;

                                    if _type == Type::Suffix {
                                        chk = end;
                                    } else {
                                        chk = start - 1;
                                    }

                                    // Beginning
                                    match scope {
                                        Scope::Punctuation => {
                                            if ((chk < 0 && (_type == Type::Prefix))
                                                || (chk >= len as i32 && (_type == Type::Suffix))
                                                || is_punctuation(fixed.at(chk as usize)))
                                                == is_negative
                                            {
                                                replace = false;
                                                break;
                                            }
                                        }
                                        Scope::Vowel => {
                                            if (((chk >= 0 && (_type == Type::Prefix))
                                                || (chk < len as i32 && (_type == Type::Suffix)))
                                                && is_vowel(fixed.at(chk as usize)))
                                                == is_negative
                                            {
                                                replace = false;
                                                break;
                                            }
                                        }

                                        Scope::Consonant => {
                                            if (((chk >= 0 && (_type == Type::Prefix))
                                                || (chk < len as i32 && (_type == Type::Suffix)))
                                                && is_consonant(fixed.at(chk as usize)))
                                                == is_negative
                                            {
                                                replace = false;
                                                break;
                                            }
                                        }

                                        Scope::Exact => {
                                            let mut s: i32 = 0;
                                            let mut e: i32 = 0;
                                            if _type == Type::Suffix {
                                                s = end;
                                                e = end + value.len() as i32;
                                            } else {
                                                // Prefix
                                                s = start - value.len() as i32;
                                                e = start;
                                            }
                                            if !is_exact(value, &fixed, s, e, is_negative) {
                                                replace = false;
                                                break;
                                            }
                                        }
                                    };
                                }

                                if replace {
                                    output += rule.replace;
                                    output += "(্[যবম])?(্?)([ঃঁ]?)";
                                    cur = (end - 1) as usize;
                                    matched = true;
                                    break;
                                }
                            }
                        }

                        if matched {
                            break;
                        }

                        // Default
                        output += pattern.replace;
                        output += "(্[যবম])?(্?)([ঃঁ]?)";
                        cur = (end - 1) as usize;
                        matched = true;
                        break;
                    } else if find.len() > chunk.len()
                        || (find.len() == chunk.len() && find.cmp(&chunk) == Ordering::Less)
                    {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
                if matched {
                    break;
                }
            }
        }

        if !matched {
            output += &fixed[cur..cur + 1];
        }
        cur += 1;
    }
    output.push('$'); // Regex end mark.

    output
}

fn clean_string(string: &str) -> String {
    string
        .to_ascii_lowercase()
        .chars()
        .filter(|&character| !is_ignore(character))
        .collect()
}

fn is_vowel(string: &str) -> bool {
    VOWELS.contains(&string.to_ascii_lowercase())
}

fn is_consonant(string: &str) -> bool {
    CONSONANT.contains(&string.to_ascii_lowercase())
}

fn is_ignore(character: char) -> bool {
    IGNORE.contains(character.to_ascii_lowercase())
}

fn is_exact(needle: &str, heystack: &str, start: i32, end: i32, not: bool) -> bool {
    (start >= 0
        && end < heystack.len() as i32
        && (&heystack[start as usize..end as usize] == needle))
        != not
}

fn is_punctuation(character: &str) -> bool {
    !(is_vowel(character) || is_consonant(character))
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn regex_test() {
        assert_eq!(parse("l"), "^ল(্[যবম])?(্?)([ঃঁ]?)$");
        assert_eq!(parse("osthir"), "^([ওোঅ]|(অ্য)|(য়ো?))(্[যবম])?(্?)([ঃঁ]?)([সশষ])(্[যবম])?(্?)([ঃঁ]?)(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্\u{200C}?))))(্[যবম])?(্?)([ঃঁ]?)([ইঈিী]|(য়[িী]))(্[যবম])?(্?)([ঃঁ]?)([রড়ঢ়]|(হ্র))(্[যবম])?(্?)([ঃঁ]?)$");
        assert_eq!(parse("OSTHIR"), "^([ওোঅ]|(অ্য)|(য়ো?))(্[যবম])?(্?)([ঃঁ]?)([সশষ])(্[যবম])?(্?)([ঃঁ]?)(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্\u{200C}?))))(্[যবম])?(্?)([ঃঁ]?)([ইঈিী]|(য়[িী]))(্[যবম])?(্?)([ঃঁ]?)([রড়ঢ়]|(হ্র))(্[যবম])?(্?)([ঃঁ]?)$");
    }
}
