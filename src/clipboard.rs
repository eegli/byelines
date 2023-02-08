use anyhow::Result;

pub use arboard::Clipboard;

pub trait ClipboardIO {
    fn get_text(&mut self) -> Result<String>;
    fn set_text(&mut self, text: &str) -> Result<()>;
}

impl ClipboardIO for arboard::Clipboard {
    fn get_text(&mut self) -> Result<String> {
        self.get_text().map_err(|err| err.into())
    }

    fn set_text(&mut self, text: &str) -> Result<()> {
        self.set_text(text).map_err(|err| err.into())
    }
}
