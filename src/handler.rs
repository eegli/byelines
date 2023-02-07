use arboard::Clipboard;
use regex::Regex;

const REPLACEMENT_PATTERN: &str = r"\r\n|\n|\r";

pub struct ClipboardString {
    cached: Option<String>,
    cp: Clipboard,
    re: Regex,
}

impl ClipboardString {
    pub fn new() -> Self {
        let mut this = Self {
            cached: None,
            cp: Clipboard::new().unwrap(),
            re: Regex::new(&REPLACEMENT_PATTERN).unwrap(),
        };
        this.cached = this.get_content();
        this
    }

    pub fn strip_newlines(&self, content: &str) -> String {
        self.re.replace_all(content, " ").to_string()
    }

    pub fn get_content(&mut self) -> Option<String> {
        let content = self.cp.get_text().ok()?;
        println!("{:?}", Some(&content) == self.cached.as_ref());
        match Some(&content) == self.cached.as_ref() {
            true => None,
            false => {
                self.cached = Some(content.clone());
                Some(content)
            }
        }
    }

    pub fn set_content(&mut self, content: String) -> Option<()> {
        self.cp.set_text(content).ok()
    }
}
