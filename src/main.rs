mod clipboard;
mod handler;

use handler::Handler;
use simple_logger::SimpleLogger;

fn main() -> () {
    SimpleLogger::new().init().unwrap();
    let mut clipboard = arboard::Clipboard::new().unwrap();
    Handler::new(&mut clipboard).launch(1000);
}
