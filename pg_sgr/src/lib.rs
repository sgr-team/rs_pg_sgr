//! # pg_sgr
//! 
//! Sugar for postgres
//! 
//! ## Features
//! 
//! - from_row - macros for deserializing rust data structures from postgres rows
#[cfg(feature = "from_row")] pub use pg_sgr_from_row::*;