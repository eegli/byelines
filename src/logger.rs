pub use log;
use simple_logger::SimpleLogger;

pub fn init() {
    SimpleLogger::new().init().unwrap();
}
