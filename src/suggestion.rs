use edit_distance::edit_distance;
use std::cmp::Ordering;

/// Suggestions which are intended to be shown by the IM's candidate window.
/// Suggestion is of two variants, the 'Full' one includes a list of suggestion and
/// the 'Single' one is just a String.
#[derive(Debug)]
pub enum Suggestion {
    Full {
        auxiliary: String,
        suggestions: Vec<Rank>,
        // Index of the last selected suggestion.
        selection: usize,
    },
    Single {
        suggestion: String,
    },
}

impl Suggestion {
    /// Creates a new `Suggestion` struct with given arguments.
    ///
    /// `auxiliary`: The auxiliary text.
    ///
    /// `suggestions`: Vector of suggestions.
    ///
    /// `selection`: Index of the last selected suggestion.
    pub fn new(auxiliary: String, suggestions: Vec<Rank>, selection: usize) -> Self {
        Self::Full {
            auxiliary,
            suggestions,
            selection,
        }
    }

    /// Creates a new `Suggestion` struct with only one suggestion.
    ///
    /// *A lonely suggestion.* 😁
    ///
    /// `suggestion`: The suggestion.
    pub fn new_lonely(suggestion: String) -> Self {
        Self::Single { suggestion }
    }

    /// Constructs an empty `Suggestion` struct.
    pub fn empty() -> Self {
        Self::Single {
            suggestion: String::new(),
        }
    }

    /// Returns `true` when the `Suggestion` struct is a **lonely** one, otherwise returns `false`.
    ///
    /// A *lonely* `Suggestion` struct means that the struct has only one suggestion.
    pub fn is_lonely(&self) -> bool {
        match &self {
            Self::Single { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` if the `Suggestion` struct is empty.
    pub fn is_empty(&self) -> bool {
        match &self {
            Self::Full { suggestions, .. } => suggestions.is_empty(),
            Self::Single { suggestion } => suggestion.is_empty(),
        }
    }

    /// Get the suggestions as an iterator.
    pub fn get_suggestions(&self) -> impl Iterator<Item = &str> {
        match &self {
            Self::Full { suggestions, .. } => suggestions.iter().map(Rank::to_string),
            _ => panic!(),
        }
    }

    /// Get the only suggestion of the *lonely* `Suggestion`.
    pub fn get_lonely_suggestion(&self) -> &str {
        match &self {
            Self::Single { suggestion } => suggestion,
            _ => panic!(),
        }
    }

    /// Get the auxiliary text.
    pub fn get_auxiliary_text(&self) -> &str {
        match &self {
            Self::Full { auxiliary, .. } => auxiliary,
            _ => panic!(),
        }
    }

    /// Returns index of the suggestion, which was previously selected.
    pub fn previously_selected_index(&self) -> usize {
        match &self {
            Self::Full { selection, .. } => *selection,
            _ => panic!(),
        }
    }

    /// Get the length of the suggestions contained.
    pub fn len(&self) -> usize {
        match &self {
            Self::Full { suggestions, .. } => suggestions.len(),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Rank {
    First(String),
    Emoji(String, u8),
    Other(String, u8),
    Last(String, u8)
}

impl Rank {
    /// Returns the suggestion item.
    pub(crate) fn to_string(&self) -> &str {
        match self {
            Rank::First(s) => s,
            Rank::Emoji(s, _) => s,
            Rank::Other(s, _) => s,
            Rank::Last(s, _) => s,
        }
    }

    /// A first ranked suggestion.
    pub(crate) fn first_ranked(item: String) -> Self {
        Rank::First(item)
    }

    /// A suggestion with a ranking calculated according to the `base` word.
    ///
    /// Uses edit distance to rank the `item`. 
    pub(crate) fn new_suggestion(item: String, base: &str) -> Self {
        let distance = edit_distance(base, &item) * 10;
        Rank::Other(item, distance as u8)
    }

    /// An Emoji suggestion.
    pub(crate) fn emoji(item: String) -> Self {
        Rank::Emoji(item, 1)
    }

    /// An Emoji suggestion with custom ranking.
    pub(crate) fn emoji_ranked(item: String, rank: u8) -> Self {
        Rank::Emoji(item, rank)
    }

    /// A suggestion with a low `rank` ranking. 
    pub(crate) fn last_ranked(item: String, rank: u8) -> Self {
        Rank::Last(item, rank)
    }

    /// Gives a mutable reference of the Rank's item.
    pub(crate) fn change_item(&mut self) -> &mut String {
        match self {
            Rank::First(s) => s,
            Rank::Emoji(s, _) => s,
            Rank::Other(s, _) => s,
            Rank::Last(s, _) => s,
        }
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
        suggestion.push(Rank::emoji("🔥".to_owned()));
        suggestion.sort_unstable();
        assert_eq!(suggestion, ["ফিরে", "🔥", "ফীরে", "ফইড়ে"]);

        suggestion = ["অ্যা", "অ্যাঁ", "আ", "আঃ", "া", "এ"].iter().map(|&s| Rank::new_suggestion(s.to_owned(), "আ")).collect();
        suggestion.push(Rank::emoji("🅰️".to_owned()));
        suggestion.sort_unstable();
        assert_eq!(suggestion, ["আ", "🅰️", "আঃ", "া", "এ", "অ্যা", "অ্যাঁ"]);
    }
}
