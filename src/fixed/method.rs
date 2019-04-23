use crate::context::Method;
use crate::fixed::parser::{LayoutModifiers, LayoutParser};
use crate::keycodes::*;
use crate::suggestion::Suggestion;
use crate::utility::get_modifiers;

pub(crate) struct FixedMethod {
    buffer: String,
    handled: bool,
}

impl FixedMethod {
    fn create_suggestion(&self) -> Suggestion {
        Suggestion::empty()
    }
}

impl Method for FixedMethod {
    fn get_suggestion(&mut self, key: u16, modifier: u8) -> Suggestion {
        let mods = get_modifiers(modifier);
        let (shift, ctrl, alt) = mods;
        // Don't catch Ctrl or Alt without AltGr combination.
        if (ctrl && !alt) || (!ctrl && alt) {
            // Handle edge cases
            if key == VC_SHIFT || key == VC_ALT {
                if !self.buffer.is_empty() {
                    self.handled = true;
                    return self.create_suggestion();
                } else {
                    self.handled = false;
                    return Suggestion::empty();
                }
            } else {
                self.handled = false;
                return Suggestion::empty();
            }
        }

        let modifier: LayoutModifiers = mods.into();

        Suggestion::empty()
    }

    fn handle_special_key(&mut self, key: u16) -> u8 {
        0
    }

    fn key_handled(&self) -> bool {
        self.handled
    }
}
