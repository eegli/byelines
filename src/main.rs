#![allow(unused)]

mod handler;

use handler::ClipboardHandler;
use simple_logger::SimpleLogger;

fn main() -> () {
    SimpleLogger::new().init().unwrap();
    ClipboardHandler::new().launch(1000);
}
