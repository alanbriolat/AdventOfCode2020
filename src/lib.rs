#[macro_use]
pub mod util;

pub mod error;
pub use error::{Error, Result};
pub mod runner;
pub mod solutions;
