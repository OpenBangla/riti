// Fixed keyboard layout parser.

use serde_json::{Map, Value};

pub(crate) struct LayoutParser {
    layout: Map<String, Value>,
}

impl LayoutParser {
    pub(crate) fn new(layout: Map<String, Value>) -> Self {
        LayoutParser { layout }
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutParser;
    use serde_json::{self, Value};

    #[test]
    fn test_loading() {
        let parser = LayoutParser::new(
            serde_json::from_str::<Value>(include_str!("../../data/Probhat.json"))
                .unwrap()
                .get("layout")
                .unwrap()
                .as_object()
                .unwrap()
                .clone()
        );
    }
}
