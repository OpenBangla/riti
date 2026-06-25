// A `CompositionalEngine` backed by a parsed m17n `.mim` specification.

use super::interp::run_conversion;
use super::spec::{parse_spec, Spec};
use crate::compositional::engine::CompositionalEngine;

/// A compositional engine driven by an m17n `.mim` spec.
pub(crate) struct MimEngine {
    spec: Spec,
}

impl MimEngine {
    /// Build an engine by parsing the given `.mim` specification text.
    pub(crate) fn new(spec_text: &str) -> Self {
        MimEngine {
            spec: parse_spec(spec_text),
        }
    }
}

impl CompositionalEngine for MimEngine {
    fn convert(&self, tape: &str) -> String {
        run_conversion(&self.spec, tape)
    }
}
