use std::path::Path;

pub trait PathExt {
    fn ext_str(&self) -> &str;
    fn file_name_str(&self) -> &str;
}

impl PathExt for Path {
    fn ext_str(&self) -> &str {
        self.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
    }

    fn file_name_str(&self) -> &str {
        self.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
    }
}