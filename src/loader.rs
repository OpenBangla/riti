// Layout loader.

use serde_json::Value;
use std::fs::read_to_string;
use std::str::FromStr;

use crate::settings::get_settings_layout_file;

/// Layout Loader
///
/// This struct is used to load the specified layout file and
/// to give access to it's contents.
pub(crate) struct LayoutLoader {
    // Layout file path.
    path: String,
    layout: Value,
}

/// Layout type.
pub(crate) enum LayoutType {
    Phonetic,
    Fixed,
}

impl LayoutLoader {
    /// Load the layout which is specified in settings.
    pub(crate) fn load_from_settings() -> Self {
        let path = get_settings_layout_file();
        let layout: Value = serde_json::from_str(&read_to_string(&path).unwrap()).unwrap();

        LayoutLoader { path, layout }
    }

    /// Give layout's `layout` json object, which contains the layout data.
    pub(crate) fn layout(&self) -> &Value {
        &self.layout["layout"]
    }

    /// Return layout's type.
    pub(crate) fn layout_type(&self) -> LayoutType {
        self.layout["info"]["type"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap()
    }

    /// Checks if the layout path had changed.
    pub(crate) fn changed(&self) -> bool {
        self.path != get_settings_layout_file()
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
