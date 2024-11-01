mod command;
mod map;
mod option;
mod path;
pub mod path2;
mod result;
mod str;
mod vec;

pub use command::*;
pub use option::*;
pub use path::*;
pub use result::*;
pub use str::*;
pub use vec::*;

pub fn default<T: Default>() -> T {
    T::default()
}
