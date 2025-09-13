use std::ops::Deref;

pub trait VecExt<T> {
    fn any<F: Fn(&T) -> bool>(&self, f: F) -> bool;
    fn fold<F, U>(&self, f: F, init: U) -> U
    where
        F: Fn(U, &T) -> U;
    fn into_first(self) -> Option<T>;
    fn into_last(self) -> Option<T>;
    fn recollect<U: From<T>>(self) -> Vec<U>;
    fn merge_grouped<U>(
        &mut self,
        children: Vec<U>,
        matcher: impl Fn(&T, &U) -> bool,
        merger: impl Fn(&mut T, U),
    );
}

pub trait DerefVec<T>
where
    T: Deref,
{
    fn includes(&self, item: &T::Target) -> bool;
}

impl<T> VecExt<T> for Vec<T> {
    fn any<F: Fn(&T) -> bool>(&self, f: F) -> bool {
        self.iter().any(f)
    }

    fn fold<F, U>(&self, f: F, init: U) -> U
    where
        F: Fn(U, &T) -> U,
    {
        self.iter().fold(init, f)
    }

    fn into_first(self) -> Option<T> {
        self.into_iter().next()
    }

    fn into_last(mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.remove(self.len() - 1))
    }
    fn recollect<U: From<T>>(self) -> Vec<U> {
        self.into_iter().map(Into::into).collect()
    }

    fn merge_grouped<U>(
        &mut self,
        children: Vec<U>,
        matcher: impl Fn(&T, &U) -> bool,
        merger: impl Fn(&mut T, U),
    ) {
        let mut it = self.iter_mut().peekable();
        for child in children {
            while !matcher(it.peek().unwrap(), &child) {
                it.next();
            }
            let current = it.peek_mut().unwrap();
            merger(current, child);
        }
    }
}

impl<T> DerefVec<T> for Vec<T>
where
    T: Deref + PartialEq<T::Target>,
{
    fn includes(&self, item: &T::Target) -> bool {
        self.iter().any(|i| i == item)
    }
}

#[macro_export]
macro_rules! vec_into {
    ($(($($item:expr),*)),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push(($($item.into()),*));)*
            v
        }
    };
    ($($item:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($item.into());)*
            v
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let s = vec!["a".to_string(), "b".to_string()];
        assert!(s.includes("a"));
    }

    #[test]
    fn test_vec_into() {
        let s: Vec<String> = vec_into!["a", "b"];
        assert_eq!(s, vec!["a".to_string(), "b".to_string()]);
    }
}
