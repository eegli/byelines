use clipboard_master::{CallbackResult, ClipboardHandler};

use regex::Regex;
use std::io;

const REPLACEMENT_PATTERN: &str = r"\r\n|\n|\r";

pub struct Clipboard {
    cached: Option<String>,
    clipboard: arboard::Clipboard,
    re: Regex,
}

impl ClipboardHandler for Clipboard {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        if let Some(content) = self.get_content() {
            let formatted = self.strip_newlines(&content);
            match self.set_content(formatted) {
                Some(_) => {
                    log::info!("Clipboard content updated: {:.20}", content);
                }
                None => (),
            };
        };
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        log::info!("Clipboard content not updated");
        CallbackResult::Next
    }
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

    pub fn strip_newlines(&self, content: &str) -> String {
        self.re.replace_all(content, " ").to_string()
    }

    pub fn get_content(&mut self) -> Option<String> {
        let content = self.clipboard.get_text().ok()?;
        match Some(&content) == self.cached.as_ref() {
            true => None,
            false => {
                self.cached = Some(content.clone());
                Some(content)
            }
        }
    }

    pub fn set_content(&mut self, content: String) -> Option<()> {
        self.clipboard.set_text(content).ok()
    }
}
