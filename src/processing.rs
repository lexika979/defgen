use std::sync::mpsc::{Sender, Receiver};

use crate::gui::GuiMessage;

pub enum ProcessingMessage {

}

pub fn run(processing_sender: Sender<GuiMessage>, processing_receiver: Receiver<ProcessingMessage>) {

} 