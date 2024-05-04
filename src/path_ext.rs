use std::path::Path;

pub trait PathExt {
    fn ext_str(&self) -> &str;
}

impl PathExt for Path {
    fn ext_str(&self) -> &str {
        self.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
    }
}