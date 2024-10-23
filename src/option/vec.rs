pub trait OptionVecExt<T> {
    fn first(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
}

impl<T> OptionVecExt<T> for Option<Vec<T>> {
    fn first(&self) -> Option<&T> {
        self.as_ref().and_then(|v| v.first())
    }
    fn is_empty(&self) -> bool {
        self.as_ref().map_or(true, |v| v.is_empty())
    }
}
