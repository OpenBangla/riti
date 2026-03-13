use crate::{context::Method, keycodes::keycode_to_char, suggestion::Suggestion};


struct PredictivePhoneticMethod {
    buffer: String,
}

impl Method for PredictivePhoneticMethod {
    fn get_suggestion(
        &mut self,
        key: u16,
        modifier: u8,
        selection: u8,
        data: &crate::data::Data,
        config: &crate::config::Config,
    ) -> (Suggestion, bool) {
        let character = keycode_to_char(key);
        self.buffer.push(character);
        
        todo!()
    }

    fn candidate_committed(&mut self, index: usize, config: &crate::config::Config) {
        todo!()
    }

    fn update_engine(&mut self, config: &crate::config::Config) {
        todo!()
    }

    fn ongoing_input_session(&self) -> bool {
        todo!()
    }

    fn finish_input_session(&mut self) {
        todo!()
    }

    fn backspace_event(&mut self, ctrl: bool, data: &crate::data::Data, config: &crate::config::Config) -> Suggestion {
        todo!()
    }
}
    
