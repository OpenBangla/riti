// Fixed keyboard layout parser.

use serde_json::{Map, Value};
use std::fmt;

use crate::keycodes::*;
use LayoutModifiers::*;

pub(crate) enum LayoutModifiers {
    Normal,
    Shift,
    AltGr,
    ShiftAltGr,
}

pub(crate) struct LayoutParser {
    layout: Map<String, Value>,
}

impl LayoutParser {
    pub(crate) fn new(layout: Map<String, Value>) -> Self {
        LayoutParser { layout }
    }

    fn layout_get_value(&self, key: &str, modifier: LayoutModifiers) -> Option<&str> {
        self.layout
            .get(&format!("Key_{}_{}", key, modifier))
            .unwrap()
            .as_str()
    }

    fn layout_get_value_numpad(&self, key: &str) -> Option<&str> {
        self.layout.get(key).unwrap().as_str()
    }

    pub(crate) fn get_char_for_key(&self, key: u16, modifier: LayoutModifiers) -> Option<&str> {
        match (key, modifier) {
            // Numerics
            (VC_0, _) => self.layout_get_value("0", Normal),
            (VC_PAREN_RIGHT, _) => self.layout_get_value("0", Shift),
            (VC_1, _) => self.layout_get_value("1", Normal),
            (VC_EXCLAIM, _) => self.layout_get_value("1", Shift),
            (VC_2, _) => self.layout_get_value("2", Normal),
            (VC_AT, _) => self.layout_get_value("2", Shift),
            (VC_3, _) => self.layout_get_value("3", Normal),
            (VC_HASH, _) => self.layout_get_value("3", Shift),
            (VC_4, _) => self.layout_get_value("4", Normal),
            (VC_DOLLAR, _) => self.layout_get_value("4", Shift),
            (VC_5, _) => self.layout_get_value("5", Normal),
            (VC_PERCENT, _) => self.layout_get_value("5", Shift),
            (VC_6, _) => self.layout_get_value("6", Normal),
            (VC_CIRCUM, _) => self.layout_get_value("6", Shift),
            (VC_7, _) => self.layout_get_value("7", Normal),
            (VC_AMPERSAND, _) => self.layout_get_value("7", Shift),
            (VC_8, _) => self.layout_get_value("8", Normal),
            (VC_ASTERISK, _) => self.layout_get_value("8", Shift),
            (VC_9, _) => self.layout_get_value("9", Normal),
            (VC_PAREN_LEFT, _) => self.layout_get_value("9", Shift),
            // Alphabets
            (VC_A, modifier) => self.layout_get_value("A", modifier),
            (VC_B, modifier) => self.layout_get_value("B", modifier),
            (VC_C, modifier) => self.layout_get_value("C", modifier),
            (VC_D, modifier) => self.layout_get_value("D", modifier),
            (VC_E, modifier) => self.layout_get_value("E", modifier),
            (VC_F, modifier) => self.layout_get_value("F", modifier),
            (VC_G, modifier) => self.layout_get_value("G", modifier),
            (VC_H, modifier) => self.layout_get_value("H", modifier),
            (VC_I, modifier) => self.layout_get_value("I", modifier),
            (VC_J, modifier) => self.layout_get_value("J", modifier),
            (VC_K, modifier) => self.layout_get_value("K", modifier),
            (VC_L, modifier) => self.layout_get_value("L", modifier),
            (VC_M, modifier) => self.layout_get_value("M", modifier),
            (VC_N, modifier) => self.layout_get_value("N", modifier),
            (VC_O, modifier) => self.layout_get_value("O", modifier),
            (VC_P, modifier) => self.layout_get_value("P", modifier),
            (VC_Q, modifier) => self.layout_get_value("Q", modifier),
            (VC_R, modifier) => self.layout_get_value("R", modifier),
            (VC_S, modifier) => self.layout_get_value("S", modifier),
            (VC_T, modifier) => self.layout_get_value("T", modifier),
            (VC_U, modifier) => self.layout_get_value("U", modifier),
            (VC_V, modifier) => self.layout_get_value("V", modifier),
            (VC_W, modifier) => self.layout_get_value("W", modifier),
            (VC_X, modifier) => self.layout_get_value("X", modifier),
            (VC_Y, modifier) => self.layout_get_value("Y", modifier),
            (VC_Z, modifier) => self.layout_get_value("Z", modifier),
            // Other characters
            (VC_GRAVE, _) => self.layout_get_value("BackQuote", Normal),
            (VC_TILDE, _) => self.layout_get_value("BackQuote", Shift),
            (VC_MINUS, _) => self.layout_get_value("Minus", Normal),
            (VC_UNDERSCORE, _) => self.layout_get_value("Minus", Shift),
            (VC_EQUALS, _) => self.layout_get_value("Equals", Normal),
            (VC_PLUS, _) => self.layout_get_value("Equals", Shift),
            (VC_BRACKET_LEFT, _) => self.layout_get_value("OpenBracket", Normal),
            (VC_BRACE_LEFT, _) => self.layout_get_value("OpenBracket", Shift),
            (VC_BRACKET_RIGHT, _) => self.layout_get_value("CloseBracket", Normal),
            (VC_BRACE_RIGHT, _) => self.layout_get_value("CloseBracket", Shift),
            (VC_BACK_SLASH, _) => self.layout_get_value("BackSlash", Normal),
            (VC_BAR, _) => self.layout_get_value("BackSlash", Shift),
            (VC_SEMICOLON, _) => self.layout_get_value("Semicolon", Normal),
            (VC_COLON, _) => self.layout_get_value("Semicolon", Shift),
            (VC_APOSTROPHE, _) => self.layout_get_value("Quote", Normal),
            (VC_QUOTE, _) => self.layout_get_value("Quote", Shift),
            (VC_COMMA, _) => self.layout_get_value("Comma", Normal),
            (VC_LESS, _) => self.layout_get_value("Comma", Shift),
            (VC_PERIOD, _) => self.layout_get_value("Period", Normal),
            (VC_GREATER, _) => self.layout_get_value("Period", Shift),
            (VC_SLASH, _) => self.layout_get_value("Slash", Normal),
            (VC_QUESTION, _) => self.layout_get_value("Slash", Shift),
            // NumPad
            (VC_KP_0, _) => self.layout_get_value_numpad("Num0"),
            (VC_KP_1, _) => self.layout_get_value_numpad("Num1"),
            (VC_KP_2, _) => self.layout_get_value_numpad("Num2"),
            (VC_KP_3, _) => self.layout_get_value_numpad("Num3"),
            (VC_KP_4, _) => self.layout_get_value_numpad("Num4"),
            (VC_KP_5, _) => self.layout_get_value_numpad("Num5"),
            (VC_KP_6, _) => self.layout_get_value_numpad("Num6"),
            (VC_KP_7, _) => self.layout_get_value_numpad("Num7"),
            (VC_KP_8, _) => self.layout_get_value_numpad("Num8"),
            (VC_KP_9, _) => self.layout_get_value_numpad("Num9"),
            (VC_KP_DIVIDE, _) => self.layout_get_value_numpad("NumDivide"),
            (VC_KP_MULTIPLY, _) => self.layout_get_value_numpad("NumMultiply"),
            (VC_KP_SUBTRACT, _) => self.layout_get_value_numpad("NumSubtract"),
            (VC_KP_ADD, _) => self.layout_get_value_numpad("NumAdd"),
            (VC_KP_DECIMAL, _) => self.layout_get_value_numpad("NumDecimal"),

            _ => None,
        }
    }
}

impl fmt::Display for LayoutModifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Normal => write!(f, "Normal"),
            Shift => write!(f, "Shift"),
            AltGr => write!(f, "AltGr"),
            ShiftAltGr => write!(f, "ShiftAltGr"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LayoutParser, LayoutModifiers};
    use crate::keycodes::*;
    use serde_json::{self, Value};

    #[test]
    fn test_loading() {
        let _parser = LayoutParser::new(
            serde_json::from_str::<Value>(include_str!("../../data/Probhat.json"))
                .unwrap()
                .get("layout")
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        );
    }

    #[test]
    fn test_key_bindings() {
        // Load the layout
        let parser = LayoutParser::new(
            serde_json::from_str::<Value>(include_str!("../../data/Probhat.json"))
                .unwrap()
                .get("layout")
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        );

        assert_eq!(parser.get_char_for_key(VC_A, LayoutModifiers::Normal), Some("া"));
        assert_eq!(parser.get_char_for_key(VC_A, LayoutModifiers::Shift), Some("অ"));
        assert_eq!(parser.get_char_for_key(VC_A, LayoutModifiers::AltGr), Some("ঌ"));
        assert_eq!(parser.get_char_for_key(VC_A, LayoutModifiers::ShiftAltGr), Some("ৠ"));

        assert_eq!(parser.get_char_for_key(VC_1, LayoutModifiers::Normal), Some("১"));
        assert_eq!(parser.get_char_for_key(VC_EXCLAIM, LayoutModifiers::Normal), Some("!"));

        assert_eq!(parser.get_char_for_key(VC_BACK_SLASH, LayoutModifiers::Normal), Some("‌")); // ZWNJ
        assert_eq!(parser.get_char_for_key(VC_BAR, LayoutModifiers::Normal), Some("॥"));
        assert_eq!(parser.get_char_for_key(VC_BAR, LayoutModifiers::Shift), Some("॥"));
    }
}
