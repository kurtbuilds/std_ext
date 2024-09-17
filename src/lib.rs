mod path_ext;
mod vec_ext;
pub mod path2;

pub use path_ext::*;
pub use vec_ext::*;

pub fn default<T: Default>() -> T {
    T::default()
}