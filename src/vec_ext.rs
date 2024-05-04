use std::ops::Deref;

pub trait VecExt<T> {
    fn any<F: Fn(&T) -> bool>(&self, f: F) -> bool;
}

pub trait DerefVec<T> where T: Deref {
    fn includes(&self, item: &T::Target) -> bool;
}

impl<T> VecExt<T> for Vec<T> {
    fn any<F: Fn(&T) -> bool>(&self, f: F) -> bool {
        self.iter().any(f)
    }
}

impl<T> DerefVec<T> for Vec<T> where T: Deref + PartialEq<T::Target> {
    fn includes(&self, item: &T::Target) -> bool {
        self.iter().any(|i| i == item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let s = vec!["a".to_string(), "b".to_string()];
        assert!(s.includes("a"));
    }
}