use std::sync::mpsc::{Sender, Receiver};

use crate::processing::ProcessingMessage;

pub enum GuiMessage {

}

pub fn run(gui_sender: Sender<ProcessingMessage>, gui_receiver: Receiver<GuiMessage>) {

} 