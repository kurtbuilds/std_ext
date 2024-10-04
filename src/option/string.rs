pub trait OptionStringExt {
    fn as_str(&self) -> &str;
}

impl OptionStringExt for Option<String> {
    fn as_str(&self) -> &str {
        self.as_deref().unwrap_or_default()
    }
}