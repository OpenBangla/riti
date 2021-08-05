// Fixed keyboard layout parser.

use std::collections::HashMap;
use std::fmt;

use crate::config::Config;
use crate::keycodes::*;
use crate::utility::Modifiers;
use serde_json::Value;
use LayoutModifiers::*;

#[derive(Debug, PartialEq)]
pub(crate) enum LayoutModifiers {
    Normal,
    AltGr,
}

pub(crate) struct LayoutParser {
    layout: HashMap<String, String>,
}

impl LayoutParser {
    pub(crate) fn new(layout: Value) -> Self {
        let layout = serde_json::from_value(layout).unwrap();
        LayoutParser { layout }
    }

    fn layout_get_value(&self, key: &str, modifier: LayoutModifiers) -> Option<String> {
        self.layout
            .get(&format!("Key_{}_{}", key, modifier))
            .filter(|s| !s.is_empty())
            .cloned()
    }

    fn layout_get_value_numpad(&self, key: &str, config: &Config) -> Option<String> {
        self.layout
            .get(key)
            .filter(|s| config.get_fixed_numpad() && !s.is_empty())
            .cloned()
    }

    pub(crate) fn get_char_for_key(&self, key: u16, modifier: LayoutModifiers, config: &Config) -> Option<String> {
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
            (VC_A, modifier) => self.layout_get_value("a", modifier),
            (VC_B, modifier) => self.layout_get_value("b", modifier),
            (VC_C, modifier) => self.layout_get_value("c", modifier),
            (VC_D, modifier) => self.layout_get_value("d", modifier),
            (VC_E, modifier) => self.layout_get_value("e", modifier),
            (VC_F, modifier) => self.layout_get_value("f", modifier),
            (VC_G, modifier) => self.layout_get_value("g", modifier),
            (VC_H, modifier) => self.layout_get_value("h", modifier),
            (VC_I, modifier) => self.layout_get_value("i", modifier),
            (VC_J, modifier) => self.layout_get_value("j", modifier),
            (VC_K, modifier) => self.layout_get_value("k", modifier),
            (VC_L, modifier) => self.layout_get_value("l", modifier),
            (VC_M, modifier) => self.layout_get_value("m", modifier),
            (VC_N, modifier) => self.layout_get_value("n", modifier),
            (VC_O, modifier) => self.layout_get_value("o", modifier),
            (VC_P, modifier) => self.layout_get_value("p", modifier),
            (VC_Q, modifier) => self.layout_get_value("q", modifier),
            (VC_R, modifier) => self.layout_get_value("r", modifier),
            (VC_S, modifier) => self.layout_get_value("s", modifier),
            (VC_T, modifier) => self.layout_get_value("t", modifier),
            (VC_U, modifier) => self.layout_get_value("u", modifier),
            (VC_V, modifier) => self.layout_get_value("v", modifier),
            (VC_W, modifier) => self.layout_get_value("w", modifier),
            (VC_X, modifier) => self.layout_get_value("x", modifier),
            (VC_Y, modifier) => self.layout_get_value("y", modifier),
            (VC_Z, modifier) => self.layout_get_value("z", modifier),

            (VC_A_SHIFT, modifier) => self.layout_get_value("A", modifier),
            (VC_B_SHIFT, modifier) => self.layout_get_value("B", modifier),
            (VC_C_SHIFT, modifier) => self.layout_get_value("C", modifier),
            (VC_D_SHIFT, modifier) => self.layout_get_value("D", modifier),
            (VC_E_SHIFT, modifier) => self.layout_get_value("E", modifier),
            (VC_F_SHIFT, modifier) => self.layout_get_value("F", modifier),
            (VC_G_SHIFT, modifier) => self.layout_get_value("G", modifier),
            (VC_H_SHIFT, modifier) => self.layout_get_value("H", modifier),
            (VC_I_SHIFT, modifier) => self.layout_get_value("I", modifier),
            (VC_J_SHIFT, modifier) => self.layout_get_value("J", modifier),
            (VC_K_SHIFT, modifier) => self.layout_get_value("K", modifier),
            (VC_L_SHIFT, modifier) => self.layout_get_value("L", modifier),
            (VC_M_SHIFT, modifier) => self.layout_get_value("M", modifier),
            (VC_N_SHIFT, modifier) => self.layout_get_value("N", modifier),
            (VC_O_SHIFT, modifier) => self.layout_get_value("O", modifier),
            (VC_P_SHIFT, modifier) => self.layout_get_value("P", modifier),
            (VC_Q_SHIFT, modifier) => self.layout_get_value("Q", modifier),
            (VC_R_SHIFT, modifier) => self.layout_get_value("R", modifier),
            (VC_S_SHIFT, modifier) => self.layout_get_value("S", modifier),
            (VC_T_SHIFT, modifier) => self.layout_get_value("T", modifier),
            (VC_U_SHIFT, modifier) => self.layout_get_value("U", modifier),
            (VC_V_SHIFT, modifier) => self.layout_get_value("V", modifier),
            (VC_W_SHIFT, modifier) => self.layout_get_value("W", modifier),
            (VC_X_SHIFT, modifier) => self.layout_get_value("X", modifier),
            (VC_Y_SHIFT, modifier) => self.layout_get_value("Y", modifier),
            (VC_Z_SHIFT, modifier) => self.layout_get_value("Z", modifier),

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
            (VC_KP_0, _) => self.layout_get_value_numpad("Num0", config),
            (VC_KP_1, _) => self.layout_get_value_numpad("Num1", config),
            (VC_KP_2, _) => self.layout_get_value_numpad("Num2", config),
            (VC_KP_3, _) => self.layout_get_value_numpad("Num3", config),
            (VC_KP_4, _) => self.layout_get_value_numpad("Num4", config),
            (VC_KP_5, _) => self.layout_get_value_numpad("Num5", config),
            (VC_KP_6, _) => self.layout_get_value_numpad("Num6", config),
            (VC_KP_7, _) => self.layout_get_value_numpad("Num7", config),
            (VC_KP_8, _) => self.layout_get_value_numpad("Num8", config),
            (VC_KP_9, _) => self.layout_get_value_numpad("Num9", config),
            (VC_KP_DIVIDE, _) => self.layout_get_value_numpad("NumDivide", config),
            (VC_KP_MULTIPLY, _) => self.layout_get_value_numpad("NumMultiply", config),
            (VC_KP_SUBTRACT, _) => self.layout_get_value_numpad("NumSubtract", config),
            (VC_KP_ADD, _) => self.layout_get_value_numpad("NumAdd", config),
            (VC_KP_DECIMAL, _) => self.layout_get_value_numpad("NumDecimal", config),

            _ => None,
        }
    }
}

impl From<Modifiers> for LayoutModifiers {
    fn from(modifiers: Modifiers) -> Self {
        match modifiers {
            (_, false) => Normal,
            (_, true) => AltGr,
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
    use crate::config::get_fixed_method_defaults;
    use crate::keycodes::*;
    use serde_json::{self, Value};

    #[test]
    fn test_key_bindings() {
        // Load the layout
        let layout =
            serde_json::from_str::<Value>(include_str!("../../data/Probhat.json")).unwrap();
        let layout = layout.get("layout").unwrap().to_owned();
        let parser = LayoutParser::new(layout);
        let config = get_fixed_method_defaults();

        assert_eq!(
            parser.get_char_for_key(VC_A, LayoutModifiers::Normal, &config),
            Some("া".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_A_SHIFT, LayoutModifiers::Normal, &config),
            Some("অ".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_A, LayoutModifiers::AltGr, &config),
            Some("ঌ".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_A_SHIFT, LayoutModifiers::AltGr, &config),
            Some("ৠ".to_string())
        );

        assert_eq!(
            parser.get_char_for_key(VC_4, LayoutModifiers::Normal, &config),
            Some("৪".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_4, LayoutModifiers::AltGr, &config),
            Some("৷".to_string())
        );

        assert_eq!(
            parser.get_char_for_key(VC_DOLLAR, LayoutModifiers::Normal, &config),
            Some("৳".to_string())
        );
        assert_eq!(
            parser.get_char_for_key(VC_DOLLAR, LayoutModifiers::AltGr, &config),
            Some("৲".to_string())
        );

        assert_eq!(
            parser.get_char_for_key(VC_BACK_SLASH, LayoutModifiers::Normal, &config),
            Some("‌".to_string())
        ); // ZWNJ
        assert_eq!(
            parser.get_char_for_key(VC_BAR, LayoutModifiers::Normal, &config),
            Some("॥".to_string())
        );
    }

    #[test]
    fn test_modifiers() {
        assert_eq!(
            LayoutModifiers::from((false, false)),
            LayoutModifiers::Normal
        );
        assert_eq!(
            LayoutModifiers::from((true, false)),
            LayoutModifiers::Normal
        );
        assert_eq!(LayoutModifiers::from((false, true)), LayoutModifiers::AltGr);
        assert_eq!(LayoutModifiers::from((true, true)), LayoutModifiers::AltGr);
    }

    #[test]
    fn test_all_keys() {
        // Load the layout
        let layout =
            serde_json::from_str::<Value>(include_str!("../../data/Probhat.json")).unwrap();
        let layout = layout.get("layout").unwrap().to_owned();
        let parser = LayoutParser::new(layout);
        let config = get_fixed_method_defaults();

        assert!(parser
            .get_char_for_key(VC_GRAVE, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_GRAVE, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_TILDE, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_TILDE, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_1, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_1, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_2, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_2, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_3, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_3, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_4, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_4, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_5, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_5, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_6, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_6, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_7, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_7, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_8, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_8, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_9, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_9, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_0, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_0, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EXCLAIM, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EXCLAIM, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_HASH, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_HASH, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_DOLLAR, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_DOLLAR, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERCENT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERCENT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_CIRCUM, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_CIRCUM, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AMPERSAND, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_AMPERSAND, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_ASTERISK, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_ASTERISK, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_LEFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_LEFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_RIGHT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PAREN_RIGHT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_UNDERSCORE, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_UNDERSCORE, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PLUS, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PLUS, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_MINUS, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_MINUS, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EQUALS, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_EQUALS, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_A, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_A, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_B, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_B, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_C, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_C, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_D, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_D, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_E, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_E, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_F, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_F, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_G, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_G, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_H, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_H, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_I, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_I, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_J, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_J, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_K, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_K, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_L, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_L, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_M, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_M, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_N, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_N, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_O, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_O, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_P, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_P, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Q, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Q, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_R, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_R, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_S, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_S, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_T, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_T, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_U, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_U, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_V, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_V, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_W, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_W, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_X, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_X, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Y, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Y, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Z, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Z, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_A_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_A_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_B_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_B_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_C_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_C_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_D_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_D_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_E_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_E_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_F_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_F_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_G_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_G_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_H_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_H_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_I_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_I_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_J_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_J_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_K_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_K_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_L_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_L_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_M_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_M_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_N_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_N_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_O_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_O_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_P_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_P_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Q_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Q_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_R_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_R_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_S_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_S_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_T_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_T_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_U_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_U_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_V_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_V_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_W_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_W_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_X_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_X_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Y_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Y_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Z_SHIFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_Z_SHIFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_LEFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_LEFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_RIGHT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACKET_RIGHT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BACK_SLASH, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BACK_SLASH, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_LEFT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_LEFT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_RIGHT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BRACE_RIGHT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BAR, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_BAR, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SEMICOLON, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SEMICOLON, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_APOSTROPHE, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_APOSTROPHE, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COMMA, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COMMA, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERIOD, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_PERIOD, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SLASH, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_SLASH, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COLON, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_COLON, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUOTE, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUOTE, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_LESS, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_LESS, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_GREATER, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_GREATER, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUESTION, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_QUESTION, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DIVIDE, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DIVIDE, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_MULTIPLY, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_MULTIPLY, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_SUBTRACT, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_SUBTRACT, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_ADD, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_ADD, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DECIMAL, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_DECIMAL, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_1, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_1, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_2, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_2, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_3, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_3, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_4, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_4, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_5, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_5, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_6, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_6, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_7, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_7, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_8, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_8, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_9, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_9, LayoutModifiers::AltGr, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_0, LayoutModifiers::Normal, &config)
            .is_some());
        assert!(parser
            .get_char_for_key(VC_KP_0, LayoutModifiers::AltGr, &config)
            .is_some());
    }
}
