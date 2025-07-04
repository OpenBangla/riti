use regex::Regex;

use super::chars::is_ligature_making_kar;
use crate::{data::Data, fixed::chars::ZWNJ, suggestion::Rank};

/// Find words from the dictionary `data` with given word, rank them according to the `base` word and append them in the `suggestions`.
pub(crate) fn search_dictionary(
    word: &str,
    base: &str,
    suggestions: &mut Vec<Rank>,
    traditional_kar: bool,
    data: &Data,
) {
    let table = match word.chars().next().unwrap_or_default() {
        // Kars
        'া' => "aa",
        'ি' => "i",
        'ী' => "ii",
        'ু' => "u",
        'ূ' => "uu",
        'ৃ' => "rri",
        'ে' => "e",
        'ৈ' => "oi",
        'ো' => "o",
        'ৌ' => "ou",
        // Vowels
        'অ' => "a",
        'আ' => "aa",
        'ই' => "i",
        'ঈ' => "ii",
        'উ' => "u",
        'ঊ' => "uu",
        'ঋ' => "rri",
        'এ' => "e",
        'ঐ' => "oi",
        'ও' => "o",
        'ঔ' => "ou",
        // Consonants
        'ক' => "k",
        'খ' => "kh",
        'গ' => "g",
        'ঘ' => "gh",
        'ঙ' => "nga",
        'চ' => "c",
        'ছ' => "ch",
        'জ' => "j",
        'ঝ' => "jh",
        'ঞ' => "nya",
        'ট' => "tt",
        'ঠ' => "tth",
        'ড' => "dd",
        'ঢ' => "ddh",
        'ণ' => "nn",
        'ত' => "t",
        'থ' => "th",
        'দ' => "d",
        'ধ' => "dh",
        'ন' => "n",
        'প' => "p",
        'ফ' => "ph",
        'ব' => "b",
        'ভ' => "bh",
        'ম' => "m",
        'য' => "z",
        'র' => "r",
        'ল' => "l",
        'শ' => "sh",
        'ষ' => "ss",
        'স' => "s",
        'হ' => "h",
        'ড়' => "rr",
        'ঢ়' => "rrh",
        'য়' => "y",
        'ৎ' => "khandatta",
        // Otherwise we don't have any suggestions to search from, so return from the function.
        _ => return,
    };

    let word = clean_string(word);

    let need_chars_upto = match word.chars().count() {
        1 => 0,
        2..=3 => 1,
        _ => 5,
    };

    let regex = format!(
        "^{word}[অআইঈউঊঋএঐওঔঌৡািীুূৃেৈোৌকখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহৎড়ঢ়য়ংঃঁ\u{09CD}]{{0,{need_chars_upto}}}$"
    );
    let rgx = Regex::new(&regex).unwrap();

    let words = data.get_words_for(table).filter(|i| rgx.is_match(i));

    if traditional_kar {
        suggestions.extend(words.map(|w| {
            // Check if the word has any of the ligature making Kars.
            let word = if w.chars().any(is_ligature_making_kar) {
                let mut temp = String::with_capacity(w.capacity());
                for ch in w.chars() {
                    if is_ligature_making_kar(ch) {
                        temp.push(ZWNJ);
                    }
                    temp.push(ch);
                }
                temp
            } else {
                w.clone()
            };

            Rank::new_suggestion(word, base)
        }));
    } else {
        suggestions.extend(words.map(|s| Rank::new_suggestion(s.clone(), base)));
    }
}

fn clean_string(string: &str) -> String {
    string
        .chars()
        .filter(|&c| !"|()[]{}^$*+?.~!@#%&-_='\";<>/\\,:`।\u{200C}".contains(c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{clean_string, search_dictionary};
    use crate::{config::get_fixed_method_defaults, data::Data, suggestion::Rank};

    #[test]
    fn test_database() {
        let config = get_fixed_method_defaults();
        let data = Data::new(&config);
        let mut suggestion = Vec::new();

        search_dictionary("ই", "", &mut suggestion, false, &data);
        assert_eq!(suggestion, ["ই"]);
        suggestion.clear();

        search_dictionary("আমা", "", &mut suggestion, false, &data);
        assert_eq!(suggestion, ["আমা", "আমান", "আমার", "আমায়"]);
        suggestion.clear();

        search_dictionary("খ(১", "", &mut suggestion, false, &data);
        assert_eq!(suggestion, Vec::<Rank>::new());
        suggestion.clear();

        search_dictionary("1", "", &mut suggestion, false, &data);
        assert_eq!(suggestion, Vec::<Rank>::new());
        suggestion.clear();
    }

    #[test]
    fn test_ignore_meta_chars() {
        assert_eq!(clean_string("Me|t(a)"), "Meta");
    }
}

#[cfg(feature = "bench")]
mod benches {
    extern crate test;

    use super::search_dictionary;
    use crate::config::get_fixed_method_defaults;
    use crate::data::Data;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_fixed_database_ama(b: &mut Bencher) {
        let config = get_fixed_method_defaults();
        let data = Data::new(&config);

        b.iter(|| {
            let mut suggestions = Vec::new();
            search_dictionary("আমা", "", &mut suggestions, false, &data);
            black_box(suggestions);
        })
    }

    #[bench]
    fn bench_fixed_database_compiu(b: &mut Bencher) {
        let config = get_fixed_method_defaults();
        let data = Data::new(&config);

        b.iter(|| {
            let mut suggestions = Vec::new();
            search_dictionary("কম্পি", "", &mut suggestions, false, &data);
            black_box(suggestions);
        })
    }

    #[bench]
    fn bench_fixed_database_ains(b: &mut Bencher) {
        let config = get_fixed_method_defaults();
        let data = Data::new(&config);

        b.iter(|| {
            let mut suggestions = Vec::new();
            search_dictionary("আইনস্", "", &mut suggestions, false, &data);
            black_box(suggestions);
        })
    }
}
