// Compositional input methods.
//
// A compositional layout turns a roman key sequence into Bengali through a
// deterministic engine. This is a third input category in riti, alongside the
// phonetic and fixed methods. Khipro is the first such layout, driven by an
// embedded `.mim` spec.

pub(crate) mod engine;
pub(crate) mod method;
mod mim;

use engine::CompositionalEngine;
use mim::MimEngine;

/// Built-in layout name for the Khipro compositional layout.
pub(crate) const KHIPRO_LAYOUT: &str = "khipro";

/// The embedded Khipro `.mim` specification (desktop variant).
const KHIPRO_SPEC: &str = include_str!("../../data/khipro/bn-khipro.mim");

/// Returns `true` if `layout` names a supported compositional layout.
pub(crate) fn is_compositional_layout(layout: &str) -> bool {
    layout == KHIPRO_LAYOUT
}

/// Builds the compositional engine for the given built-in layout name.
///
/// Khipro is currently the only compositional layout, and the name is already
/// validated by [`is_compositional_layout`] before this is reached. When more
/// compositional layouts are added, turn this into a `match` on `layout`.
fn engine_for_layout(layout: &str) -> Box<dyn CompositionalEngine> {
    debug_assert_eq!(layout, KHIPRO_LAYOUT);
    Box::new(MimEngine::new(KHIPRO_SPEC))
}

#[cfg(test)]
mod tests {
    use super::mim::{parse_spec, run_conversion};
    use super::KHIPRO_SPEC;

    /// Splits a CSV line into fields, honouring double-quoted fields that may
    /// themselves contain commas.
    fn split_csv_line(line: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        for ch in line.chars() {
            match ch {
                '"' => in_quotes = !in_quotes,
                ',' if !in_quotes => {
                    fields.push(current.trim().to_string());
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        fields.push(current.trim().to_string());
        fields
    }

    #[test]
    fn test_khipro_conformance() {
        let spec = parse_spec(KHIPRO_SPEC);
        let csv = include_str!("../../data/khipro/khipro-testcases.csv");

        let mut failures = Vec::new();
        for (line_no, line) in csv.lines().enumerate() {
            let line = line.trim_end_matches('\r');
            if line.is_empty() {
                continue;
            }
            // Skip the header row.
            if line_no == 0 && line.starts_with("input") {
                continue;
            }

            let fields = split_csv_line(line);
            if fields.len() < 2 {
                continue;
            }
            let input = &fields[0];
            let expected = &fields[1];

            let got = run_conversion(&spec, input);
            if &got != expected {
                failures.push(format!("input={input:?} expected={expected:?} got={got:?}"));
            }
        }

        assert!(
            failures.is_empty(),
            "Khipro conformance failures ({}):\n{}",
            failures.len(),
            failures.join("\n")
        );
    }
}
