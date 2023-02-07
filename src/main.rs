#![allow(unused)]
use clipboard_master::Master as Runner;
use std::error::Error;

mod clipboard;

use clipboard::Clipboard;
use simple_logger::SimpleLogger;

fn main() -> () {
    SimpleLogger::new().init().unwrap();
    let _ = Runner::new(Clipboard::new()).run();
}
