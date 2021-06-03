// Layout loader.

use serde_json::Value;
use std::fs::read_to_string;

use crate::config::Config;

/// Layout Loader
///
/// This struct is used to load the specified layout file and
/// to give access to it's contents.
pub(crate) struct LayoutLoader {
    // Layout file path.
    path: Option<String>,
    typ: LayoutType,
    layout: Option<Value>,
}

/// Layout type.
#[derive(Copy, Clone)]
pub(crate) enum LayoutType {
    Phonetic,
    Fixed,
}

impl LayoutLoader {
    /// Load the layout which is specified in config.
    pub(crate) fn load_from_config(config: &Config) -> Self {
        let path = config.get_layout_file_path().to_string();

        let (path, typ, layout) = if path == "avro_phonetic" {
            (None, LayoutType::Phonetic, None)
        } else {
            let layout: Value = serde_json::from_str(&read_to_string(&path).unwrap()).unwrap();
            (Some(path), LayoutType::Fixed, Some(layout))
        };

        LayoutLoader { path, typ, layout }
    }

    /// Give layout's `layout` json object, which contains the layout data.
    pub(crate) fn layout(&self) -> &Value {
        &self.layout.as_ref().unwrap()["layout"]
    }

    /// Return layout's type.
    pub(crate) fn layout_type(&self) -> LayoutType {
        self.typ
    }
}
