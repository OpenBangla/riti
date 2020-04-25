// Fixed keyboard layout parser.

#![allow(non_upper_case_globals)]

use serde_json::{Map, Value};
use std::fmt;

use crate::keycodes::*;
use crate::settings::get_settings_fixed_numberpad;
use crate::utility::Modifiers;
use LayoutModifiers::*;

#[derive(Debug, PartialEq)]
pub(crate) enum LayoutModifiers {
    Normal,
    AltGr,
}

pub(crate) struct LayoutParser {
    layout: Map<String, Value>,
}

impl LayoutParser {
    pub(crate) fn new(layout: &Value) -> Self {
        let layout = layout.as_object().unwrap().clone();
        LayoutParser { layout }
    }

    fn layout_get_value(&self, key: &str, modifier: LayoutModifiers) -> Option<String> {
        self.layout
            .get(&format!("Key_{}_{}", key, modifier))
            .unwrap()
            .as_str()
            .map(|s| s.to_string())
    }

    fn layout_get_value_numpad(&self, key: &str) -> Option<String> {
        if get_settings_fixed_numberpad() {
            self.layout
                .get(key)
                .unwrap()
                .as_str()
                .map(|s| s.to_string())
        } else {
            None
        }
    }

    pub(crate) fn get_char_for_key(&self, key: u16, modifier: LayoutModifiers) -> Option<String> {
        match (key, modifier) {
            // Numerics
            (VC_0, modifier) => self.layout_get_value("0", modifier),
            (VC_PAREN_RIGHT, modifier) => self.layout_get_value("ParenRight", modifier),

            (VC_1, modifier) => self.layout_get_value("1", modifier),
            (VC_EXCLAIM, modifier) => self.layout_get_value("Exclaim", modifier),

            (VC_2, modifier) => self.layout_get_value("2", modifier),
            (VC_AT, modifier) => self.layout_get_value("At", modifier),

            (VC_3, modifier) => self.layout_get_value("3", modifier),
            (VC_HASH, modifier) => self.layout_get_value("Hash", modifier),

            (VC_4, modifier) => self.layout_get_value("4", modifier),
            (VC_DOLLAR, modifier) => self.layout_get_value("Dollar", modifier),

            (VC_5, modifier) => self.layout_get_value("5", modifier),
            (VC_PERCENT, modifier) => self.layout_get_value("Percent", modifier),

            (VC_6, modifier) => self.layout_get_value("6", modifier),
            (VC_CIRCUM, modifier) => self.layout_get_value("Circum", modifier),

            (VC_7, modifier) => self.layout_get_value("7", modifier),
            (VC_AMPERSAND, modifier) => self.layout_get_value("Ampersand", modifier),

            (VC_8, modifier) => self.layout_get_value("8", modifier),
            (VC_ASTERISK, modifier) => self.layout_get_value("Asterisk", modifier),

            (VC_9, modifier) => self.layout_get_value("9", modifier),
            (VC_PAREN_LEFT, modifier) => self.layout_get_value("ParenLeft", modifier),

            // Alphabets
            (VC_a, modifier) => self.layout_get_value("a", modifier),
            (VC_b, modifier) => self.layout_get_value("b", modifier),
            (VC_c, modifier) => self.layout_get_value("c", modifier),
            (VC_d, modifier) => self.layout_get_value("d", modifier),
            (VC_e, modifier) => self.layout_get_value("e", modifier),
            (VC_f, modifier) => self.layout_get_value("f", modifier),
            (VC_g, modifier) => self.layout_get_value("g", modifier),
            (VC_h, modifier) => self.layout_get_value("h", modifier),
            (VC_i, modifier) => self.layout_get_value("i", modifier),
            (VC_j, modifier) => self.layout_get_value("j", modifier),
            (VC_k, modifier) => self.layout_get_value("k", modifier),
            (VC_l, modifier) => self.layout_get_value("l", modifier),
            (VC_m, modifier) => self.layout_get_value("m", modifier),
            (VC_n, modifier) => self.layout_get_value("n", modifier),
            (VC_o, modifier) => self.layout_get_value("o", modifier),
            (VC_p, modifier) => self.layout_get_value("p", modifier),
            (VC_q, modifier) => self.layout_get_value("q", modifier),
            (VC_r, modifier) => self.layout_get_value("r", modifier),
            (VC_s, modifier) => self.layout_get_value("s", modifier),
            (VC_t, modifier) => self.layout_get_value("t", modifier),
            (VC_u, modifier) => self.layout_get_value("u", modifier),
            (VC_v, modifier) => self.layout_get_value("v", modifier),
            (VC_w, modifier) => self.layout_get_value("w", modifier),
            (VC_x, modifier) => self.layout_get_value("x", modifier),
            (VC_y, modifier) => self.layout_get_value("y", modifier),
            (VC_z, modifier) => self.layout_get_value("z", modifier),

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
            (VC_GRAVE, modifier) => self.layout_get_value("Grave", modifier),
            (VC_TILDE, modifier) => self.layout_get_value("Tilde", modifier),

            (VC_MINUS, modifier) => self.layout_get_value("Minus", modifier),
            (VC_UNDERSCORE, modifier) => self.layout_get_value("UnderScore", modifier),

            (VC_EQUALS, modifier) => self.layout_get_value("Equals", modifier),
            (VC_PLUS, modifier) => self.layout_get_value("Plus", modifier),

            (VC_BRACKET_LEFT, modifier) => self.layout_get_value("BracketLeft", modifier),
            (VC_BRACE_LEFT, modifier) => self.layout_get_value("BraceLeft", modifier),

            (VC_BRACKET_RIGHT, modifier) => self.layout_get_value("BracketRight", modifier),
            (VC_BRACE_RIGHT, modifier) => self.layout_get_value("BraceRight", modifier),

            (VC_BACK_SLASH, modifier) => self.layout_get_value("BackSlash", modifier),
            (VC_BAR, modifier) => self.layout_get_value("Bar", modifier),

            (VC_SEMICOLON, modifier) => self.layout_get_value("Semicolon", modifier),
            (VC_COLON, modifier) => self.layout_get_value("Colon", modifier),

            (VC_APOSTROPHE, modifier) => self.layout_get_value("Apostrophe", modifier),
            (VC_QUOTE, modifier) => self.layout_get_value("Quote", modifier),

            (VC_COMMA, modifier) => self.layout_get_value("Comma", modifier),
            (VC_LESS, modifier) => self.layout_get_value("Less", modifier),

            (VC_PERIOD, modifier) => self.layout_get_value("Period", modifier),
            (VC_GREATER, modifier) => self.layout_get_value("Greater", modifier),

            (VC_SLASH, modifier) => self.layout_get_value("Slash", modifier),
            (VC_QUESTION, modifier) => self.layout_get_value("Question", modifier),

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

impl From<Modifiers> for LayoutModifiers {
    fn from(modifiers: Modifiers) -> Self {
        match modifiers {
            (_, false, false) => Normal,
            (_, true, true) => AltGr,
            _ => panic!("Unknown modifier combination"),
        }
    }
}

impl fmt::Display for LayoutModifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Normal => write!(f, "Normal"),
            AltGr => write!(f, "AltGr"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LayoutModifiers, LayoutParser};
    use crate::keycodes::*;
    use serde_json::{self, Value};

    #[test]
    fn test_key_bindings() {
        // Load the layout
        let layout =
            serde_json::from_str::<Value>(include_str!("../../data/Probhat.json")).unwrap();
        let layout = layout.get("layout").unwrap();
        let parser = LayoutParser::new(layout);

        assert_eq!(
            parser.get_char_for_key(VC_a, LayoutModifiers::Normal),
            Some("া".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_A, LayoutModifiers::Normal),
            Some("অ".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_a, LayoutModifiers::AltGr),
            Some("ঌ".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_A, LayoutModifiers::AltGr),
            Some("ৠ".to_string())
        );

        assert_eq!(
            parser.get_char_for_key(VC_4, LayoutModifiers::Normal),
            Some("৪".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_4, LayoutModifiers::AltGr),
            Some("৷".to_string())
        );

        assert_eq!(
            parser.get_char_for_key(VC_DOLLAR, LayoutModifiers::Normal),
            Some("৳".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_DOLLAR, LayoutModifiers::AltGr),
            Some("৲".to_string())
        );

        assert_eq!(
            parser.get_char_for_key(VC_BACK_SLASH, LayoutModifiers::Normal),
            Some("‌".to_string())
        ); // ZWNJ
        assert_eq!(
            parser.get_char_for_key(VC_BAR, LayoutModifiers::Normal),
            Some("॥".to_string())
        );
    }

    #[test]
    fn test_modifiers() {
        assert_eq!(
            LayoutModifiers::from((false, false, false)),
            LayoutModifiers::Normal
        );
        assert_eq!(
            LayoutModifiers::from((true, false, false)),
            LayoutModifiers::Normal
        );
        assert_eq!(
            LayoutModifiers::from((false, true, true)),
            LayoutModifiers::AltGr
        );
        assert_eq!(
            LayoutModifiers::from((true, true, true)),
            LayoutModifiers::AltGr
        );
    }

    #[test]
    fn test_all_keys() {
        // Load the layout
        let layout =
            serde_json::from_str::<Value>(include_str!("../../data/Probhat.json")).unwrap();
        let layout = layout.get("layout").unwrap();
        let parser = LayoutParser::new(layout);

        assert!(parser
            .get_char_for_key(VC_GRAVE, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_GRAVE, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_TILDE, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_TILDE, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_1, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_1, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_2, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_2, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_3, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_3, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_4, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_4, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_5, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_5, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_6, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_6, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_7, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_7, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_8, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_8, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_9, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_9, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_0, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_0, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EXCLAIM, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EXCLAIM, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_HASH, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_HASH, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_DOLLAR, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_DOLLAR, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERCENT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERCENT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_CIRCUM, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_CIRCUM, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AMPERSAND, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AMPERSAND, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_ASTERISK, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_ASTERISK, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_LEFT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_LEFT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_RIGHT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_RIGHT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_UNDERSCORE, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_UNDERSCORE, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PLUS, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PLUS, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_MINUS, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_MINUS, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EQUALS, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EQUALS, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_a, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_a, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_b, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_b, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_c, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_c, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_d, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_d, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_e, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_e, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_f, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_f, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_g, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_g, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_h, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_h, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_i, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_i, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_j, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_j, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_k, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_k, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_l, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_l, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_m, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_m, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_n, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_n, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_o, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_o, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_p, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_p, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_q, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_q, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_r, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_r, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_s, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_s, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_t, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_t, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_u, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_u, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_v, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_v, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_w, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_w, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_x, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_x, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_y, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_y, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_z, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_z, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_A, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_A, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_B, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_B, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_C, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_C, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_D, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_D, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_E, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_E, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_F, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_F, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_G, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_G, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_H, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_H, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_I, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_I, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_J, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_J, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_K, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_K, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_L, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_L, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_M, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_M, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_N, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_N, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_O, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_O, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_P, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_P, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Q, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Q, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_R, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_R, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_S, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_S, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_T, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_T, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_U, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_U, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_V, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_V, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_W, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_W, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_X, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_X, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Y, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Y, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Z, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Z, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_LEFT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_LEFT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_RIGHT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_RIGHT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BACK_SLASH, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BACK_SLASH, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_LEFT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_LEFT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_RIGHT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_RIGHT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BAR, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BAR, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SEMICOLON, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SEMICOLON, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_APOSTROPHE, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_APOSTROPHE, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COMMA, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COMMA, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERIOD, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERIOD, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SLASH, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SLASH, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COLON, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COLON, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUOTE, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUOTE, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_LESS, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_LESS, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_GREATER, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_GREATER, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUESTION, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUESTION, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DIVIDE, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DIVIDE, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_MULTIPLY, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_MULTIPLY, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_SUBTRACT, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_SUBTRACT, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_ADD, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_ADD, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DECIMAL, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DECIMAL, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_1, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_1, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_2, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_2, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_3, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_3, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_4, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_4, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_5, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_5, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_6, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_6, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_7, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_7, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_8, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_8, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_9, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_9, LayoutModifiers::AltGr)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_0, LayoutModifiers::Normal)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_0, LayoutModifiers::AltGr)
            .is_some());
    }
}
