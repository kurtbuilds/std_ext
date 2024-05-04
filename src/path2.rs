use std::ops::{Deref, Div, RangeFull};
use std::path::{Path as StdPath, PathBuf as StdPathBuf};

pub struct Path(StdPath);

pub struct PathBuf(StdPathBuf);

impl Deref for Path {
    type Target = StdPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for PathBuf {
    type Target = StdPathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PathBuf {
    pub fn new() -> Self {
        Self(StdPathBuf::new())
    }
}

impl Div<&str> for PathBuf {
    type Output = PathBuf;

    fn div(mut self, rhs: &str) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl Div<RangeFull> for PathBuf {
    type Output = PathBuf;

    fn div(mut self, _: RangeFull) -> Self::Output {
        self.0.pop();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path() {
        let p = PathBuf::new() / "foo" / "bar" / ..;
        assert_eq!(p.0, StdPathBuf::from("foo"));
    }
}