use regex::Regex;
use std::io;
use std::{thread, time::Duration};

const REPLACEMENT_PATTERN: &str = r"\r\n|\n|\r";

pub struct Clipboard {
    cached: Option<String>,
    clipboard: arboard::Clipboard,
    re: Regex,
}

impl Clipboard {
    pub fn new() -> Self {
        let mut this = Self {
            cached: None,
            clipboard: arboard::Clipboard::new().unwrap(),
            re: Regex::new(&REPLACEMENT_PATTERN).unwrap(),
        };
        this.cached = this.get_content();
        this
    }

    pub fn start(&mut self, ms_intervall: i16) {
        let i = Duration::from_millis(ms_intervall as u64);
        loop {
            thread::sleep(i);
            self.handle_change();
        }
    }

    fn handle_change(&mut self) -> () {
        if let Some(content) = self.get_content() {
            let formatted = self.strip_newlines(&content);
            if formatted == content {
                log::info!("CP update skipped");
                return;
            }
            match self.set_content(formatted) {
                Some(_) => {
                    log::info!("CP updated: {:.20}", content);
                }
                None => log::error!("Error updating clipboard"),
            };
        };
    }

    fn strip_newlines(&self, content: &str) -> String {
        self.re.replace_all(content, " ").to_string()
    }

    fn get_content(&mut self) -> Option<String> {
        let content = self.clipboard.get_text().ok()?;
        match Some(&content) == self.cached.as_ref() {
            true => None,
            false => {
                self.cached = Some(content.clone());
                Some(content)
            }
        }
    }

    fn set_content(&mut self, content: String) -> Option<()> {
        self.clipboard.set_text(content).ok()
    }
}
