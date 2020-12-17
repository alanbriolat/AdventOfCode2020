#[macro_use]
pub mod util;

pub mod error;
pub use error::{Error, Result};
pub mod runner;
pub use runner::Runner;
pub mod solutions;
pub mod vector;
