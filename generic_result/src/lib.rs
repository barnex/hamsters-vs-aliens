//! Utilities for handling Errors and Results where we don't care about the concrete Error type.
mod error;
mod result;

pub use error::*;
pub use result::*;
