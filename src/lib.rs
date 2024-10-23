mod command;
mod map;
mod option;
mod path;
pub mod path2;
mod str;
mod vec_ext;

pub use command::*;
pub use option::*;
pub use path::*;
pub use str::*;
pub use vec_ext::*;

pub fn default<T: Default>() -> T {
    T::default()
}
