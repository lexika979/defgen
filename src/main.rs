use std::sync::{self, mpsc::{Sender, Receiver}};

use gui::GuiMessage;
use processing::ProcessingMessage;
use simple_logger::SimpleLogger;

mod gui;
mod processing;

fn main() {
    SimpleLogger::new().with_colors(true).init().expect("Failed to initialize logger");

    log::info!("Starting defgen");

    let (gui_sender, processing_receiver): (Sender<ProcessingMessage>, Receiver<ProcessingMessage>) = sync::mpsc::channel();
    let (processing_sender, gui_receiver): (Sender<GuiMessage>, Receiver<GuiMessage>) = sync::mpsc::channel();

    std::thread::spawn(move || {
        log::info!("Started processing thread");
        processing::run(processing_sender, processing_receiver);
    });

    log::info!("Running GUI");
    gui::run(gui_sender, gui_receiver);
} 