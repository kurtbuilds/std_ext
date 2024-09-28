mod path_ext;
mod vec_ext;
pub mod path2;
mod command_ext;

pub use path_ext::*;
pub use vec_ext::*;

pub fn default<T: Default>() -> T {
    T::default()
}