mod string;
pub use string::*;

mod vec;
pub use vec::*;

pub trait OptionExt<T> {
    fn map_ref<U>(&self, f: impl FnOnce(&T)-> U) -> Option<U>;
}

impl<T> OptionExt<T> for Option<T> {
    fn map_ref<U>(&self, f: impl FnOnce(&T)-> U) -> Option<U> {
        self.as_ref().map(f)
    }
}