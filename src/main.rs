#![allow(unused)]

use std::error::Error;
use std::{thread, time};

mod cp_handler;
mod logger;

use cp_handler::ClipboardString;
use logger::log;

fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    let mut clipboard = ClipboardString::new();
    let one_sec = time::Duration::from_millis(2000);

    loop {
        thread::sleep(one_sec);
        if let Some(content) = clipboard.get_content() {
            let formatted = clipboard.strip_newlines(&content);
            log::info!("{}", formatted);
            match clipboard.set_content(formatted) {
                Some(_) => log::info!("Clipboard content updated"),
                None => log::info!("Clipboard content not updated"),
            };
        }
    }
}
