pub trait StrExt {
    fn into_option(self) -> Option<String>;
}

impl StrExt for &str {
    fn into_option(self) -> Option<String> {
        if self.is_empty() {
            None
        } else {
            Some(self.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_string() {
        let s = "hello".to_string();
        assert_eq!(s.into_option().as_deref(), Some("hello"));
    }
}
