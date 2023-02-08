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
enum ClipboardChangeResult {
    NoChange(String),
    Updated,
}

#[derive(Debug, PartialEq)]
enum ClipboardContentRequest {
    CacheHit,
    RequestError(String),
    Content(String),
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
    pub fn launch(&mut self, ms_intervall: i16) {
        let i = Duration::from_millis(ms_intervall as u64);
        loop {
            thread::sleep(i);
            self.handle_change();
        }
    }

    fn handle_change(&mut self) -> ClipboardChangeResult {
        use ClipboardChangeResult::*;
        use ClipboardContentRequest::*;
        match self.get_content() {
            Content(content) => {
                let formatted = self.strip_newlines(&content);
                if formatted == content {
                    return NoChange(format!("Skipping update (no newlines found)"));
                }
                match self.set_content(&formatted) {
                    Some(_) => Updated,
                    // The underlying clipboard crate does not expose the error type
                    None => NoChange(format!("Error writing to clipboard")),
                }
            }
            RequestError(err) => NoChange(format!("Error reading from clipboard: {}", err)),
            CacheHit => NoChange(format!("Reading clipboard value from cache")),
        }
    }

    fn strip_newlines(&self, content: &str) -> String {
        self.re.replace_all(content, " ").trim().to_string()
    }

    fn get_content(&mut self) -> ClipboardContentRequest {
        let content = self.clipboard.get_text();
        if let Err(err) = content {
            let err = err.to_string();
            return ClipboardContentRequest::RequestError(err);
        }
        let content = content.unwrap();
        match Some(&content) == self.cached.as_ref() {
            true => ClipboardContentRequest::CacheHit,
            false => {
                self.cached = Some(content.clone());
                ClipboardContentRequest::Content(content)
            }
        }
    }

    fn set_content(&mut self, content: &str) -> Option<()> {
        self.clipboard.set_text(content).ok()
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
        fn set_text(&mut self, text: &str) -> Result<()> {
            self.0 = text.to_string();
            Ok(())
        }
    }

    #[test]
    fn strips_clipboard_content() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = "\ntest\r\ntest".to_string();

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        assert_eq!(res, ClipboardChangeResult::Updated);
        assert_eq!(mock_clipboard.0, "test test".to_string());
    }
    #[test]
    fn skips_updating_on_cache_hit() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = "\ntest\r\ntest".to_string();

        let mut handler = Handler::new(&mut mock_clipboard);

        let _ = handler.handle_change();
        let res = handler.handle_change();

        assert!(matches!(res, ClipboardChangeResult::NoChange(_)));
        assert_eq!(mock_clipboard.0, "test test".to_string());
    }
    #[test]
    fn skips_update_on_no_newlines() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = "test test".to_string();

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        println!("{:?}", res);
        assert!(matches!(res, ClipboardChangeResult::NoChange(_)));
        assert_eq!(mock_clipboard.0, "test test".to_string());
    }
}

mod test_clipboard_rw_failure {

    use super::*;
    use anyhow::Result;

    #[derive(Default)]
    struct MockClipboard(Option<String>);

    impl ClipboardIO for MockClipboard {
        fn get_text(&mut self) -> Result<String> {
            match self.0 {
                Some(ref text) => Ok(text.clone()),
                None => anyhow::bail!("CP_READ_ERROR"),
            }
        }
        fn set_text(&mut self, _text: &str) -> Result<()> {
            anyhow::bail!("CP_WRITE_ERROR")
        }
    }

    #[test]
    fn skips_update_on_failed_cp_read() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = None;

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        assert_eq!(
            res,
            ClipboardChangeResult::NoChange(
                "Error reading from clipboard: CP_READ_ERROR".to_string()
            )
        );
    }

    #[test]
    fn skips_update_on_failed_cp_write() {
        let mut mock_clipboard = MockClipboard::default();
        mock_clipboard.0 = Some("test\n".to_string());

        let mut handler = Handler::new(&mut mock_clipboard);

        let res = handler.handle_change();

        assert_eq!(
            res,
            ClipboardChangeResult::NoChange("Error writing to clipboard".to_string())
        );
    }
}
