mod path;
mod vec_ext;
pub mod path2;
mod command;
mod option;
mod map;

pub use path::*;
pub use vec_ext::*;
pub use command::*;
pub use option::*;

pub fn default<T: Default>() -> T {
    T::default()
}