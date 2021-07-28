use regex::Regex;

use crate::data::Data;

/// Find words from the dictionary `data` with given word and append them in the `suggestions`.
pub(crate) fn search_dictionary(word: &str, suggestions: &mut Vec<String>, data: &Data) {
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
        "^{}[অআইঈউঊঋএঐওঔঌৡািীুূৃেৈোৌকখগঘঙচছজঝঞটঠডঢণতথদধনপফবভমযরলশষসহৎড়ঢ়য়ংঃঁ\u{09CD}]{{0,{}}}$",
        word, need_chars_upto
    );
    let rgx = Regex::new(&regex).unwrap();

    suggestions.extend(
        data.get_words_for(table)
            .filter(|i| rgx.is_match(i))
            .cloned(),
    )
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
    use crate::{config::get_fixed_method_defaults, data::Data};

    #[test]
    fn test_database() {
        let config = get_fixed_method_defaults();
        let data = Data::new(&config);
        let mut suggestion = Vec::new();

        search_dictionary("ই", &mut suggestion, &data);
        assert_eq!(suggestion, ["ই"]);
        suggestion.clear();

        search_dictionary("আমা", &mut suggestion, &data);
        assert_eq!(suggestion, ["আমা", "আমান", "আমার", "আমায়"]);
        suggestion.clear();

        search_dictionary("খ(১", &mut suggestion, &data);
        assert_eq!(suggestion, Vec::<String>::new());
        suggestion.clear();

        search_dictionary("1", &mut suggestion, &data);
        assert_eq!(suggestion, Vec::<String>::new());
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
            search_dictionary("আমা", &mut suggestions, &data);
            black_box(suggestions);
        })
    }

    #[bench]
    fn bench_fixed_database_compiu(b: &mut Bencher) {
        let config = get_fixed_method_defaults();
        let data = Data::new(&config);

        b.iter(|| {
            let mut suggestions = Vec::new();
            search_dictionary("কম্পি", &mut suggestions, &data);
            black_box(suggestions);
        })
    }

    #[bench]
    fn bench_fixed_database_ains(b: &mut Bencher) {
        let config = get_fixed_method_defaults();
        let data = Data::new(&config);

        b.iter(|| {
            let mut suggestions = Vec::new();
            search_dictionary("আইনস্", &mut suggestions, &data);
            black_box(suggestions);
        })
    }
}
