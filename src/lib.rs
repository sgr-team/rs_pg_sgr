//! # pg_sgr
//! 
//! Sugar for postgres
//! 
//! ## Features
//! 
//! - sync - use postgres crate
//! - tokio - use tokio_postgres crate
//! - [from_row](https://docs.rs/pg_sgr_from_row/latest/pg_sgr_from_row/) - derive macros for deserializing rust data structures from postgres rows

#[cfg(feature = "from_row")]
mod from_row { pub use pg_sgr_from_row::*; }