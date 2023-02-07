use anyhow::{anyhow, Result};
use clipboard::{ClipboardContext, ClipboardProvider};

pub struct ClipboardString {
    cached: Option<String>,
    ctx: clipboard::ClipboardContext,
}

impl ClipboardString {
    pub fn new() -> Self {
        let ctx = ClipboardContext::new().unwrap();
        Self { cached: None, ctx }
    }

    pub fn format_log(content: &str) -> String {
        format!("Converted \"{:.20}\"", content)
    }

    pub fn strip_newlines(&self, content: &str) -> String {
        content.lines().fold("".to_string(), |acc, f| acc + f)
    }

    pub fn get_content(&mut self) -> Option<String> {
        self.cached = if let Ok(context) = self.ctx.get_contents() {
            match Some(&context) != self.cached.as_ref() {
                true => Some(context),
                false => None,
            }
        } else {
            None
        };
        self.cached
            .as_ref()
            .and_then(|content| Some(self.strip_newlines(&content)))
    }

    pub fn set_content(&mut self, content: String) -> Option<()> {
        self.ctx.set_contents(content).ok()
    }
}
