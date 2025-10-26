pub trait Clipboard {
    fn set_text(&self, text: &str);
    fn get_text(&self) -> Option<String>;
}