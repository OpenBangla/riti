use std::cmp::Ordering;
use edit_distance::edit_distance;
use either::Either;

/// Suggestions which are intended to be shown by the IM's candidate window.
#[derive(Debug)]
pub struct Suggestion {
    // Auxiliary text
    auxiliary: String,
    // Suggestion is of two variants, the 'normal' one includes a list of suggestion and
    // the 'lonely' one is just a String.
    suggestion: Either<Vec<String>, String>,
    // Index of the previously selected suggestion.
    selection: usize,
}

impl Suggestion {
    /// Creates a new `Suggestion` struct with given arguments.
    ///
    /// `auxiliary`: The auxiliary text.
    ///
    /// `suggestions`: Vector of suggestions.
    ///
    /// `selection`: Index of the previously selected suggestion.
    pub fn new(auxiliary: String, suggestions: Vec<String>, selection: usize) -> Self {
        Suggestion {
            auxiliary,
            suggestion: Either::Left(suggestions),
            selection,
        }
    }

    /// Creates a new `Suggestion` struct with only one suggestion.
    ///
    /// *A lonely suggestion.* 😁
    ///
    /// `suggestion`: The suggestion.
    pub fn new_lonely(suggestion: String) -> Self {
        Suggestion {
            auxiliary: String::new(),
            suggestion: Either::Right(suggestion),
            selection: 0,
        }
    }

    /// Constructs an empty `Suggestion` struct.
    pub fn empty() -> Self {
        Suggestion {
            auxiliary: String::new(),
            suggestion: Either::Right(String::new()),
            selection: 0,
        }
    }

    /// Returns `true` when the `Suggestion` struct is a **lonely** one, otherwise returns `false`.
    ///
    /// A *lonely* `Suggestion` struct means that the struct has only one suggestion.
    pub fn is_lonely(&self) -> bool {
        self.suggestion.is_right()
    }

    /// Returns `true` if the `Suggestion` struct is empty.
    pub fn is_empty(&self) -> bool {
        match &self.suggestion {
            Either::Left(list) => list.is_empty(),
            Either::Right(suggestion) => suggestion.is_empty(),
        }
    }

    /// Get the suggestions as a slice.
    pub fn get_suggestions(&self) -> &[String] {
        self.suggestion.as_ref().left().unwrap()
    }

    /// Get the only suggestion of the *lonely* `Suggestion`.
    pub fn get_lonely_suggestion(&self) -> &str {
        self.suggestion.as_ref().right().unwrap()
    }

    /// Get the auxiliary text.
    pub fn get_auxiliary_text(&self) -> &str {
        &self.auxiliary
    }

    /// Returns index of the suggestion, which was previously selected.
    pub fn previously_selected_index(&self) -> usize {
        self.selection
    }

    /// Get the length of the suggestions contained.
    pub fn len(&self) -> usize {
        self.suggestion.as_ref().left().unwrap().len()
    }
}

#[derive(Debug)]
enum Rank {
    First(String),
    Emoji(String, u8),
    Other(String, u8),
    Last(String, u8)
}

impl Rank {
    pub(crate) fn to_string(&self) -> &str {
        match self {
            Rank::First(s) => s,
            Rank::Emoji(s, _) => s,
            Rank::Other(s, _) => s,
            Rank::Last(s, _) => s,
        }
    }

    pub(crate) fn first_ranked(item: String) -> Self {
        Rank::First(item)
    }

    pub(crate) fn new_suggestion(item: String, base: &str) -> Self {
        let distance = edit_distance(base, &item) * 10;
        Rank::Other(item, distance as u8)
    }

    pub(crate) fn new_emoji(item: String) -> Self {
        Rank::Emoji(item, 1)
    }

    pub(crate) fn last_ranked(item: String, rank: u8) -> Self {
        Rank::Last(item, rank)
    }
}

impl PartialEq<&str> for Rank {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Rank::First(s) => s == other,
            Rank::Emoji(s, _) => s == other,
            Rank::Other(s, _) => s == other,
            Rank::Last(s, _) => s == other,
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Rank::First(_), Rank::First(_)) => Ordering::Equal,
            (Rank::First(_), Rank::Emoji(_, _)) => Ordering::Less,
            (Rank::Emoji(_, _), Rank::First(_)) => Ordering::Greater,
            (Rank::First(_), Rank::Other(_, _)) => Ordering::Less,
            (Rank::Other(_, _), Rank::First(_)) => Ordering::Greater,
            (Rank::First(_), Rank::Last(_, _)) => Ordering::Less,
            (Rank::Last(_, _), Rank::First(_)) => Ordering::Greater,

            (Rank::Emoji(_, _), Rank::Emoji(_, _)) => Ordering::Equal,
            (Rank::Emoji(_, e), Rank::Other(_, s)) => e.cmp(s),
            (Rank::Other(_, s), Rank::Emoji(_, e)) => s.cmp(e),
            (Rank::Emoji(_, _), Rank::Last(_, _)) => Ordering::Less,
            (Rank::Last(_, _), Rank::Emoji(_, _)) => Ordering::Greater,

            (Rank::Other(_, s1), Rank::Other(_, s2)) => s1.cmp(s2),
            (Rank::Other(_, _), Rank::Last(_, _)) => Ordering::Less,
            (Rank::Last(_, _), Rank::Other(_, _)) => Ordering::Greater,

            (Rank::Last(_, s1), Rank::Last(_, s2)) => s1.cmp(s2),
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Rank {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_trait_impl() {
        let r = Rank::Emoji("Happy".to_owned(), 1);
        assert_eq!(r, "Happy");

        let mut vr1 = vec![Rank::Last(":)".to_owned(), 2), Rank::Last("Thanks!".to_owned(), 1), Rank::Other("my".to_owned(), 10), Rank::Other("friend!".to_owned(), 20), Rank::First("Hello".to_owned()), Rank::Emoji("✋".to_owned(), 1)];
        vr1.sort_unstable();
        assert_eq!(vr1, vec![Rank::First("Hello".to_owned()), Rank::Emoji("✋".to_owned(), 1), Rank::Other("my".to_owned(), 10), Rank::Other("friend!".to_owned(), 20), Rank::Last("Thanks!".to_owned(), 1), Rank::Last(":)".to_owned(), 2)]);
        assert_eq!(vr1, ["Hello", "✋", "my", "friend!", "Thanks!", ":)"]);
    }

    #[test]
    fn test_ranked_sort() {
        let mut suggestion: Vec<Rank> = ["ফইড়ে", "ফীরে", "ফিরে"].iter().map(|&s| Rank::new_suggestion(s.to_owned(), "ফিরে")).collect();
        suggestion.push(Rank::new_emoji("🔥".to_owned()));
        suggestion.sort_unstable();
        assert_eq!(suggestion, ["ফিরে", "🔥", "ফীরে", "ফইড়ে"]);

        suggestion = ["অ্যা", "অ্যাঁ", "আ", "আঃ", "া", "এ"].iter().map(|&s| Rank::new_suggestion(s.to_owned(), "আ")).collect();
        suggestion.push(Rank::new_emoji("🅰️".to_owned()));
        suggestion.sort_unstable();
        assert_eq!(suggestion, ["আ", "🅰️", "আঃ", "া", "এ", "অ্যা", "অ্যাঁ"]);
    }
}
