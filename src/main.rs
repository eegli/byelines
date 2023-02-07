#![allow(unused)]

mod clipboard;

use clipboard::Clipboard;
use simple_logger::SimpleLogger;

fn main() -> () {
    SimpleLogger::new().init().unwrap();
    let mut clipboard = Clipboard::new();

    clipboard.start(1000);
}
