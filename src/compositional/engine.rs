// Compositional engine trait: the extensibility seam for compositional layouts.
// Khipro is driven by the `.mim` interpreter (`MimEngine`); future layouts can
// implement this trait differently and need not be `.mim` based.

/// A compositional engine turns a roman "tape" into composed Bengali text.
///
/// Implementations must be deterministic and stateless with respect to the
/// `tape`: calling `convert` with the same input must always yield the same
/// output. This lets [`super::method::CompositionalMethod`] handle backspace by
/// simply dropping the last roman character and re-converting.
pub(crate) trait CompositionalEngine {
    /// Converts the full roman `tape` into composed Bengali text.
    fn convert(&self, tape: &str) -> String;
}
