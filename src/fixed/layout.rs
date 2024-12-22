// Fixed keyboard layout parser.

use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

use crate::keycodes::*;
use crate::utility::Modifiers;
use LayoutModifiers::*;

#[derive(Debug, PartialEq)]
pub(crate) enum LayoutModifiers {
    Normal,
    AltGr,
}

/// A Fixed Layout
///
/// Provides the character for a specific input key combination.
/// Initiated by parsing a fixed method layout file(JSON formatted)
pub(crate) struct Layout {
    map: HashMap<String, String>,
}

impl Layout {
    pub(crate) fn parse(json_key_map: Value) -> Option<Self> {
        serde_json::from_value(json_key_map)
            .map(|map| Layout { map })
            .ok()
    }

    fn layout_get_value(&self, key: &str, modifier: LayoutModifiers) -> Option<String> {
        self.map
            .get(&format!("Key_{}_{}", key, modifier))
            .filter(|s| !s.is_empty())
            .cloned()
    }

    fn layout_get_value_numpad(&self, key: &str, fixed_numpad: bool) -> Option<String> {
        self.map
            .get(key)
            .filter(|s| fixed_numpad && !s.is_empty())
            .cloned()
    }

    pub(crate) fn get_char_for_key(
        &self,
        key: u16,
        modifier: LayoutModifiers,
        fixed_numpad: bool,
    ) -> Option<String> {
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
            (VC_KP_0, _) => self.layout_get_value_numpad("Num0", fixed_numpad),
            (VC_KP_1, _) => self.layout_get_value_numpad("Num1", fixed_numpad),
            (VC_KP_2, _) => self.layout_get_value_numpad("Num2", fixed_numpad),
            (VC_KP_3, _) => self.layout_get_value_numpad("Num3", fixed_numpad),
            (VC_KP_4, _) => self.layout_get_value_numpad("Num4", fixed_numpad),
            (VC_KP_5, _) => self.layout_get_value_numpad("Num5", fixed_numpad),
            (VC_KP_6, _) => self.layout_get_value_numpad("Num6", fixed_numpad),
            (VC_KP_7, _) => self.layout_get_value_numpad("Num7", fixed_numpad),
            (VC_KP_8, _) => self.layout_get_value_numpad("Num8", fixed_numpad),
            (VC_KP_9, _) => self.layout_get_value_numpad("Num9", fixed_numpad),
            (VC_KP_DIVIDE, _) => self.layout_get_value_numpad("NumDivide", fixed_numpad),
            (VC_KP_MULTIPLY, _) => self.layout_get_value_numpad("NumMultiply", fixed_numpad),
            (VC_KP_SUBTRACT, _) => self.layout_get_value_numpad("NumSubtract", fixed_numpad),
            (VC_KP_ADD, _) => self.layout_get_value_numpad("NumAdd", fixed_numpad),
            (VC_KP_DECIMAL, _) => self.layout_get_value_numpad("NumDecimal", fixed_numpad),

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
    use super::{Layout, LayoutModifiers};
    use crate::keycodes::*;
    use serde_json::Value;

    #[test]
    fn test_key_bindings() {
        // Load the layout
        let layout = serde_json::from_str::<Value>(include_str!("../../data/Probhat.json"))
            .ok()
            .and_then(|v| v.get("layout").cloned())
            .and_then(Layout::parse)
            .unwrap();
        let fixed_numpad = true;

        assert_eq!(
            layout.get_char_for_key(VC_A, LayoutModifiers::Normal, fixed_numpad),
            Some("া".to_string())
        );
        assert_eq!(
            layout.get_char_for_key(VC_A_SHIFT, LayoutModifiers::Normal, fixed_numpad),
            Some("অ".to_string())
        );
        assert_eq!(
            layout.get_char_for_key(VC_A, LayoutModifiers::AltGr, fixed_numpad),
            Some("ঌ".to_string())
        );
        assert_eq!(
            layout.get_char_for_key(VC_A_SHIFT, LayoutModifiers::AltGr, fixed_numpad),
            Some("ৠ".to_string())
        );

        assert_eq!(
            layout.get_char_for_key(VC_4, LayoutModifiers::Normal, fixed_numpad),
            Some("৪".to_string())
        );
        assert_eq!(
            layout.get_char_for_key(VC_4, LayoutModifiers::AltGr, fixed_numpad),
            Some("৷".to_string())
        );

        assert_eq!(
            layout.get_char_for_key(VC_DOLLAR, LayoutModifiers::Normal, fixed_numpad),
            Some("৳".to_string())
        );
        assert_eq!(
            layout.get_char_for_key(VC_DOLLAR, LayoutModifiers::AltGr, fixed_numpad),
            Some("৲".to_string())
        );

        assert_eq!(
            layout.get_char_for_key(VC_BACK_SLASH, LayoutModifiers::Normal, fixed_numpad),
            Some("‌".to_string())
        ); // ZWNJ
        assert_eq!(
            layout.get_char_for_key(VC_BAR, LayoutModifiers::Normal, fixed_numpad),
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
        let layout = serde_json::from_str::<Value>(include_str!("../../data/Probhat.json"))
            .ok()
            .and_then(|v| v.get("layout").cloned())
            .and_then(Layout::parse)
            .unwrap();
        let fixed_numpad = true;

        assert!(layout
            .get_char_for_key(VC_GRAVE, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_GRAVE, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_TILDE, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_TILDE, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_1, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_1, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_2, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_2, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_3, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_3, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_4, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_4, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_5, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_5, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_6, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_6, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_7, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_7, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_8, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_8, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_9, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_9, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_0, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_0, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_EXCLAIM, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_EXCLAIM, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_AT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_AT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_HASH, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_HASH, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_DOLLAR, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_DOLLAR, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PERCENT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PERCENT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_CIRCUM, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_CIRCUM, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_AMPERSAND, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_AMPERSAND, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_ASTERISK, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_ASTERISK, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PAREN_LEFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PAREN_LEFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PAREN_RIGHT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PAREN_RIGHT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_UNDERSCORE, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_UNDERSCORE, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PLUS, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PLUS, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_MINUS, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_MINUS, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_EQUALS, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_EQUALS, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_A, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_A, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_B, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_B, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_C, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_C, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_D, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_D, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_E, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_E, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_F, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_F, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_G, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_G, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_H, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_H, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_I, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_I, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_J, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_J, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_K, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_K, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_L, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_L, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_M, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_M, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_N, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_N, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_O, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_O, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_P, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_P, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Q, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Q, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_R, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_R, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_S, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_S, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_T, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_T, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_U, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_U, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_V, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_V, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_W, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_W, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_X, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_X, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Y, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Y, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Z, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Z, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_A_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_A_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_B_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_B_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_C_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_C_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_D_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_D_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_E_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_E_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_F_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_F_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_G_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_G_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_H_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_H_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_I_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_I_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_J_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_J_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_K_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_K_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_L_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_L_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_M_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_M_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_N_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_N_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_O_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_O_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_P_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_P_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Q_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Q_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_R_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_R_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_S_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_S_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_T_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_T_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_U_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_U_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_V_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_V_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_W_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_W_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_X_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_X_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Y_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Y_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Z_SHIFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_Z_SHIFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACKET_LEFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACKET_LEFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACKET_RIGHT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACKET_RIGHT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BACK_SLASH, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BACK_SLASH, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACE_LEFT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACE_LEFT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACE_RIGHT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BRACE_RIGHT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BAR, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_BAR, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_SEMICOLON, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_SEMICOLON, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_APOSTROPHE, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_APOSTROPHE, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_COMMA, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_COMMA, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PERIOD, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_PERIOD, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_SLASH, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_SLASH, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_COLON, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_COLON, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_QUOTE, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_QUOTE, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_LESS, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_LESS, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_GREATER, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_GREATER, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_QUESTION, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_QUESTION, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_DIVIDE, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_DIVIDE, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_MULTIPLY, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_MULTIPLY, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_SUBTRACT, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_SUBTRACT, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_ADD, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_ADD, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_DECIMAL, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_DECIMAL, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_1, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_1, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_2, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_2, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_3, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_3, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_4, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_4, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_5, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_5, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_6, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_6, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_7, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_7, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_8, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_8, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_9, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_9, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_0, LayoutModifiers::Normal, fixed_numpad)
            .is_some());
        assert!(layout
            .get_char_for_key(VC_KP_0, LayoutModifiers::AltGr, fixed_numpad)
            .is_some());
    }
}
