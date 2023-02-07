use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use std::{thread, time};

mod content;
mod logger;
use logger::log;

fn main() -> Result<(), Box<dyn Error>> {
    logger::init();
    let mut ctx = ClipboardContext::new()?;
    let mut cached: Option<String> = None;
    let ten_millis = time::Duration::from_millis(1000);

    loop {
        thread::sleep(ten_millis);
        if let Ok(context) = ctx.get_contents() {
            if Some(&context) != cached.as_ref() {
                let text: String = context.lines().fold("".to_string(), |acc, f| acc + f);
                log::info!("Converted \"{:.20}\"", text);
                cached = Some(context);
                ctx.set_contents(text)?;
            }
        }
    }
}
