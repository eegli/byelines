#![allow(unused)]

use std::error::Error;
use std::{thread, time};

mod handler;

use handler::ClipboardString;
use simple_logger::SimpleLogger;

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();
    let mut clipboard = ClipboardString::new();
    let one_sec = time::Duration::from_millis(2000);

    loop {
        thread::sleep(one_sec);
        if let Some(content) = clipboard.get_content() {
            let formatted = clipboard.strip_newlines(&content);
            match clipboard.set_content(formatted) {
                Some(_) => log::info!("Clipboard content updated: {:.20}", content),
                None => log::info!("Clipboard content not updated"),
            };
        }
    }
}
