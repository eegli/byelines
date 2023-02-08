use regex::Regex;
use std::{thread, time::Duration};

const REPLACEMENT_PATTERN: &str = r"\r\n|\n|\r";

use crate::clipboard::ClipboardIO;

pub struct Handler<'a, T>
where
    T: ClipboardIO + 'a,
{
    cached: Option<String>,
    clipboard: &'a mut T,
    re: Regex,
}

impl<'a, T> Handler<'a, T>
where
    T: ClipboardIO + 'a,
{
    pub fn new(clipboard: &'a mut T) -> Self {
        let mut this = Self {
            cached: None,
            clipboard,
            re: Regex::new(&REPLACEMENT_PATTERN).unwrap(),
        };
        this.cached = this.get_content();
        this
    }

    /// Start the clipboard handler with a polling intervall in milliseconds
    pub fn launch(&mut self, ms_intervall: i16) {
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
            match self.set_content(&formatted) {
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

    fn set_content(&mut self, content: &str) -> Option<()> {
        self.clipboard.set_text(content).ok()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    #[derive(Default, Debug)]
    struct MockClipboard(String);

    impl ClipboardIO for MockClipboard {
        fn get_text(&mut self) -> Result<String> {
            Ok(self.0.clone())
        }
        fn set_text(&mut self, text: &str) -> Result<()> {
            self.0 = text.to_string();
            Ok(())
        }
    }
    #[test]
    fn handles_cipboard_io_ops() {
        const INPUT_STR: &str = "\ntest\r\n";
        const OUTPUT_STR: &str = " test        ";
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = INPUT_STR.to_string();

        let mut handler = Handler::new(&mut mock_clipboard);
        handler.handle_change();
        assert_eq!(mock_clipboard.get_text().unwrap(), OUTPUT_STR);
    }
}
