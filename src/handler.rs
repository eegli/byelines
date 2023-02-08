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

#[derive(Debug, PartialEq)]
enum ClipboardOpResult {
    Updated(String),
    CacheHit,
    NoContent,
    Content(String),
    Error(ClipboardError),
}

#[derive(Debug, PartialEq)]
enum ClipboardError {
    ReadError(String),
    WriteError(String),
}

impl<'a, T> Handler<'a, T>
where
    T: ClipboardIO + 'a,
{
    pub fn new(clipboard: &'a mut T) -> Self {
        Self {
            cached: None,
            clipboard,
            re: Regex::new(&REPLACEMENT_PATTERN).unwrap(),
        }
    }

    /// Start the clipboard handler with a polling intervall in milliseconds
    pub fn launch(&mut self, ms_intervall: i16) -> () {
        let i = Duration::from_millis(ms_intervall as u64);
        loop {
            thread::sleep(i);
            match self.handle_change() {
                ClipboardOpResult::Updated(content) => {
                    println!("Updated clipboard, {content:20.}");
                }
                _ => {}
            };
        }
    }

    fn handle_change(&mut self) -> ClipboardOpResult {
        use ClipboardOpResult::*;
        match self.get_content() {
            Content(content) => {
                let formatted = self.strip_newlines(&content);
                if formatted == content {
                    return NoContent;
                }
                self.set_content(&formatted)
            }
            status => status,
        }
    }

    fn strip_newlines(&self, content: &str) -> String {
        self.re.replace_all(content, " ").trim().to_string()
    }

    fn get_content(&mut self) -> ClipboardOpResult {
        use ClipboardOpResult::*;
        let content = self.clipboard.get_text();
        if let Err(err) = content {
            let err = err.to_string();
            return Error(ClipboardError::ReadError(err));
        }
        let content = content.unwrap();
        match Some(&content) == self.cached.as_ref() {
            true => ClipboardOpResult::CacheHit,
            false => {
                self.cached = Some(content.clone());
                ClipboardOpResult::Content(content)
            }
        }
    }

    fn set_content(&mut self, content: &str) -> ClipboardOpResult {
        use ClipboardOpResult::*;
        match self.clipboard.set_text(content) {
            Ok(_) => Updated(content.to_string()),
            Err(err) => Error(ClipboardError::WriteError(err.to_string())),
        }
    }
}

#[cfg(test)]
mod test_clipboard_rw_success {

    use super::*;
    use anyhow::Result;

    #[derive(Default)]
    struct MockClipboard(String);

    impl ClipboardIO for MockClipboard {
        fn get_text(&mut self) -> Result<String> {
            Ok(self.0.clone())
        }
        // Preserve the faked clipboard content, don't overwrite it
        fn set_text(&mut self, _text: &str) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn strips_clipboard_content() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = "\ntest\r\ntest".to_string();

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        assert_eq!(res, ClipboardOpResult::Updated("test test".to_string()));
    }
    #[test]
    fn skips_updating_on_cache_hit() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = "\ntest\r\ntest".to_string();

        let mut handler = Handler::new(&mut mock_clipboard);

        let _ = handler.handle_change();
        let res = handler.handle_change();

        assert_eq!(res, ClipboardOpResult::CacheHit);
    }
    #[test]
    fn skips_update_on_no_newlines() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = "test test".to_string();

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        assert_eq!(res, ClipboardOpResult::NoContent);
        assert_eq!(mock_clipboard.0, "test test".to_string());
    }
}

#[cfg(test)]
mod test_clipboard_rw_failure {

    use super::*;
    use anyhow::{anyhow, Result};
    use ClipboardOpResult::*;

    #[derive(Default)]
    struct MockClipboard(Option<String>);

    impl ClipboardIO for MockClipboard {
        fn get_text(&mut self) -> Result<String> {
            match self.0 {
                Some(ref text) => Ok(text.clone()),
                None => Err(anyhow!("CP_READ_ERROR")),
            }
        }
        fn set_text(&mut self, _text: &str) -> Result<()> {
            Err(anyhow!("CP_WRITE_ERROR"))
        }
    }

    #[test]
    fn skips_update_on_failed_cp_read() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = None;

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        assert!(matches!(res, Error(ClipboardError::ReadError(_))));
        // Assert that the right error message is returned
        assert!(matches!(res, Error(ClipboardError::ReadError(_))));
    }

    #[test]
    fn skips_update_on_failed_cp_write() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = Some("test\n".to_string());

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        assert!(matches!(res, Error(ClipboardError::WriteError(_))));
    }
}
