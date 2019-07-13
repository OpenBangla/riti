#![allow(unused_assignments)]
use std::cmp::Ordering;
use serde_json::{self, Value};
use stringplus::StringPlus;

pub(crate) struct PhoneticRegex {
    patterns: Vec<Value>,
    vowel: String,
    consonant: String,
    ignore: String,
    max_pattern_len: usize,
}

impl PhoneticRegex {
    /// Creates a new `PhoneticRegex` instance.
    pub(crate) fn new() -> PhoneticRegex {
        let file: Value = serde_json::from_str(include_str!("regex.json")).unwrap();
        PhoneticRegex {
            patterns: file["patterns"].as_array().unwrap().clone(),
            vowel: file["vowel"].as_str().unwrap().to_string(),
            consonant: file["consonant"].as_str().unwrap().to_string(),
            ignore: file["ignore"].as_str().unwrap().to_string(),
            max_pattern_len: file["patterns"][0]["find"].as_str().unwrap().len(),
        }
    }

    /// Parse `input` string containing phonetic text and return a regex string.
    pub(crate) fn parse(&self, input: &str) -> String {
        let fixed = self.clean_string(input);
        let mut output = String::new();

        let len = fixed.len();
        let mut cur = 0;
        while cur < len {
            let start = cur as i32;
            let mut end: i32 = 0;
            let mut matched = false;

            for chunk_len in (1..=self.max_pattern_len).rev() {
                end = start + chunk_len as i32;
                if end <= len as i32 {
                    let chunk = fixed.substring(start as usize, chunk_len as usize);

                    // Binary Search
                    let mut left: i32 = 0;
                    let mut right = self.patterns.len() as i32 - 1;
                    let mut mid: i32 = 0;
                    while right >= left {
                        mid = (right + left) / 2;
                        let pattern = &self.patterns[mid as usize];
                        let find = pattern["find"].as_str().unwrap();
                        if find == chunk {
                            let rules = pattern["rules"].as_array().unwrap();
                            if !rules.is_empty() {
                                for rule in rules {
                                    let mut replace = true;
                                    let mut chk = 0;
                                    let matches = rule["matches"].as_array().unwrap();
                                    for _match in matches {
                                        let value = _match["value"].as_str().unwrap_or_default();
                                        let _type = _match["type"].as_str().unwrap();
                                        let scope = _match["scope"].as_str().unwrap();
                                        let is_negative = _match["negative"].as_bool().unwrap();

                                        if _type == "suffix" {
                                            chk = end;
                                        } else {
                                            chk = start - 1;
                                        }

                                        // Beginning
                                        match scope {
                                            "punctuation" => if ((chk < 0 && (_type == "prefix"))
                                                || (chk >= len as i32 && (_type == "suffix"))
                                                || self.is_punctuation(fixed.at(chk as usize)))
                                                == is_negative
                                            {
                                                replace = false;
                                                break;
                                            },
                                            "vowel" => if (((chk >= 0 && (_type == "prefix"))
                                                || (chk < len as i32 && (_type == "suffix")))
                                                && self.is_vowel(fixed.at(chk as usize)))
                                                == is_negative
                                            {
                                                replace = false;
                                                break;
                                            },

                                            "consonant" => if (((chk >= 0 && (_type == "prefix"))
                                                || (chk < len as i32 && (_type == "suffix")))
                                                && self.is_consonant(fixed.at(chk as usize)))
                                                == is_negative
                                            {
                                                replace = false;
                                                break;
                                            },

                                            "exact" => {
                                                let mut s: i32 = 0;
                                                let mut e: i32 = 0;
                                                if _type == "suffix" {
                                                    s = end;
                                                    e = end + value.len() as i32;
                                                } else {
                                                    // Prefix
                                                    s = start - value.len() as i32;
                                                    e = start;
                                                }
                                                if !self.is_exact(value, &fixed, s, e, is_negative)
                                                {
                                                    replace = false;
                                                    break;
                                                }
                                            }
                                            _ => panic!("Unknown scope"),
                                        };
                                    }

                                    if replace {
                                        output += rule["replace"].as_str().unwrap();
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
                            output += pattern["replace"].as_str().unwrap();
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

        format!("^{}$", output)
    }

    fn clean_string(&self, string: &str) -> String {
        string
            .to_ascii_lowercase()
            .chars()
            .filter(|&character| !self.is_ignore(character))
            .collect()
    }

    fn is_vowel(&self, string: &str) -> bool {
        self.vowel.contains(&string.to_ascii_lowercase())
    }

    fn is_consonant(&self, string: &str) -> bool {
        self.consonant.contains(&string.to_ascii_lowercase())
    }

    fn is_ignore(&self, character: char) -> bool {
        self.ignore.contains(character.to_ascii_lowercase())
    }

    fn is_exact(&self, needle: &str, heystack: &str, start: i32, end: i32, not: bool) -> bool {
        let len = end - start;
        (start >= 0 && end < heystack.len() as i32
            && (heystack.substring(start as usize, len as usize) == needle)) != not
    }

    fn is_punctuation(&self, character: &str) -> bool {
        !(self.is_vowel(character) || self.is_consonant(character))
    }
}

#[cfg(test)]
mod tests {
    use super::PhoneticRegex;
    #[test]
    fn regex_test() {
        let regex = PhoneticRegex::new();
        assert_eq!(regex.parse("osthir"), "^([ওোঅ]|(অ্য)|(য়ো?))(্[যবম])?(্?)([ঃঁ]?)([সশষ])(্[যবম])?(্?)([ঃঁ]?)(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্\u{200C}?))))(্[যবম])?(্?)([ঃঁ]?)([ইঈিী]|(য়[িী]))(্[যবম])?(্?)([ঃঁ]?)([রড়ঢ়]|(হ্র))(্[যবম])?(্?)([ঃঁ]?)$");
        assert_eq!(regex.parse("OSTHIR"), "^([ওোঅ]|(অ্য)|(য়ো?))(্[যবম])?(্?)([ঃঁ]?)([সশষ])(্[যবম])?(্?)([ঃঁ]?)(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্\u{200C}?))))(্[যবম])?(্?)([ঃঁ]?)([ইঈিী]|(য়[িী]))(্[যবম])?(্?)([ঃঁ]?)([রড়ঢ়]|(হ্র))(্[যবম])?(্?)([ঃঁ]?)$");
    }
}
