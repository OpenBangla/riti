// Layout loader.

use serde_json::Value;
use std::fs::read_to_string;
use std::str::FromStr;

/// Layout Loader
/// 
/// This struct is used to load the specified layout file and
/// to give access to it's contents.
pub(crate) struct LayoutLoader {
    layout: Value,
}

/// Layout type.
pub(crate) enum LayoutType {
    Phonetic,
    Fixed,
}

impl LayoutLoader {
    /// Load the layout from `file`.
    pub(crate) fn new(file: &str) -> Self {
        let layout: Value = serde_json::from_str(&read_to_string(file).unwrap()).unwrap();

        LayoutLoader { layout }
    }

    /// Give layout's `layout` json object, which contains the layout data.
    pub(crate) fn layout(&self) -> &Value {
        &self.layout["layout"]
    }

    /// Return layout's type.
    pub(crate) fn layout_type(&self) -> LayoutType {
        self.layout["info"]["type"].as_str().unwrap().parse().unwrap()
    }
}

impl FromStr for LayoutType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "phonetic" => Ok(LayoutType::Phonetic),
            "fixed" => Ok(LayoutType::Fixed),
            _ => panic!("Unknown Layout type!"),
        }
    }
}
