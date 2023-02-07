use copypasta::{ClipboardContext, ClipboardProvider};
use regex::Regex;

const REPLACEMENT_PATTERN: &str = r"\r\n|\n|\r";

pub struct ClipboardString {
    cached: Option<String>,
    ctx: ClipboardContext,
    re: Regex,
}

impl ClipboardString {
    pub fn new() -> Self {
        let mut this = Self {
            cached: None,
            ctx: ClipboardContext::new().unwrap(),
            re: Regex::new(&REPLACEMENT_PATTERN).unwrap(),
        };
        this.cached = this.get_content();
        this
    }

    pub fn strip_newlines(&self, content: &str) -> String {
        self.re.replace_all(content, " ").to_string()
    }

    pub fn get_content(&mut self) -> Option<String> {
        let content = self.ctx.get_contents().ok()?;
        match Some(&content) == self.cached.as_ref() {
            true => None,
            false => {
                self.cached = Some(content.clone());
                Some(content)
            }
        }
    }

    pub fn set_content(&mut self, content: String) -> Option<()> {
        self.ctx.set_contents(content).ok()
    }
}
