use std::ops::{Deref, Div, RangeFull};
use std::path::{Path, PathBuf as StdPathBuf};

pub struct PathBuf(StdPathBuf);

impl From<StdPathBuf> for PathBuf {
    fn from(value: StdPathBuf) -> Self {
        Self(value)
    }
}

impl From<&Path> for PathBuf {
    fn from(value: &Path) -> Self {
        Self(value.to_path_buf())
    }
}

impl From<&str> for PathBuf {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl std::fmt::Debug for PathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for PathBuf {
    type Target = StdPathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl PathBuf {
    pub fn new() -> Self {
        Self(StdPathBuf::new())
    }
}

impl PartialEq<StdPathBuf> for PathBuf {
    fn eq(&self, other: &StdPathBuf) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Path> for PathBuf {
    fn eq(&self, other: &Path) -> bool {
        self.0 == *other
    }
}

impl Div<&str> for PathBuf {
    type Output = PathBuf;

    fn div(mut self, rhs: &str) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl Div<&Path> for PathBuf {
    type Output = PathBuf;

    fn div(mut self, rhs: &Path) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl Div<PathBuf> for PathBuf {
    type Output = PathBuf;

    fn div(mut self, rhs: PathBuf) -> Self::Output {
        self.0.push(rhs.0);
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
        assert_eq!(p, StdPathBuf::from("foo"));
    }
}