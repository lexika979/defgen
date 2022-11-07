use std::sync::mpsc::{Receiver, Sender};

use crate::gui::GuiMessage;

pub enum ProcessingMessage {}

pub fn run(
    processing_sender: Sender<GuiMessage>,
    processing_receiver: Receiver<ProcessingMessage>,
) {
}

#[cfg(test)]
mod tests {
    #[test]
    fn dump_json() {
        let app_id = 1879630;
    }
}
