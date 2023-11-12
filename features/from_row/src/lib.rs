//! # FromRow
//! 
//! Part of pg_sgr: macros for deserializing rust data structures from postgres rows.
//! 
//! ## Macros
//! 
//! Crate contains 3 derive macros:
//! 
//! - [FromRow](derive.FromRow.html) - for deserializing rust data structures from &postgres::Row
//! - [FromRows](derive.FromRows.html) - for deserializing rust data structures from Vec\<postgres::Row\>
//! - [FromJson](derive.FromJson.html) - for deserializing rust data structures from JSON & JSONB columns
mod macro_from_json;
mod macro_from_row;
mod macro_from_rows;

use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;

/// # Description
/// 
/// The derive macro allows to deserilize rust data structure from &postgres::Row
/// 
/// # Attributes
/// 
/// Each field can be described by the from_row attribute.
/// 
/// Attribute supports 3 properties:
/// 
/// - from - specifies the expression used to obtain data from a row (variable "row" available)
/// - column - allows to rename a column
/// - default - allows to specify a default value for nullable columns
/// 
/// # Example
/// 
/// ```rust
/// use pg_sgr_from_row::FromRow;
/// 
/// #[derive(FromRow)]
/// struct Book {
///   pub isbn: String,
///   pub name: String,
///   #[from_row(from = 0)]
///   pub count: i32,
///   #[from_row(column = "bookDescription", default = "".into())]
///   pub description: String,
/// }
/// 
/// fn get_book_by_id(client: &mut postgres::Client, id: &str) -> Result<Book, postgres::Error> {
///   let row = client.query_one("SQL", &[ &id ])?;
///   Ok((&row).into())
/// }
/// ```
#[proc_macro_derive(FromRow, attributes(from_row))]
pub fn from_row_derive(input: TokenStream) -> TokenStream {
  TokenStream::from(macro_from_row::execute_macro(parse_macro_input!(input as DeriveInput)))
}

/// # Description
/// 
/// The derive macro allows to deserilize rust data structure from Vec\<postgres::Row\>
/// 
/// # Example
/// 
/// ```rust
/// use pg_sgr_from_row::{FromRow, FromRows};
/// 
/// #[derive(FromRow, FromRows)]
/// struct Book {
///   pub isbn: String,
///   pub name: String,
/// }
/// 
/// fn get_books(client: &mut postgres::Client) -> Result<Vec<Book>, postgres::Error> {
///   client.query("SQL", &[]).map(Book::from_rows)
/// }
/// ```
#[proc_macro_derive(FromRows)]
pub fn from_rows_derive(input: TokenStream) -> TokenStream {
  TokenStream::from(macro_from_rows::execute_macro(parse_macro_input!(input as DeriveInput)))
}

/// # Description
/// 
/// The derive macro allows to deserilize rust data structure from JSON or JSONB
/// 
/// # Example
/// 
/// ```rust
/// use pg_sgr_from_row::{FromRow, FromJson};
/// 
/// use serde::Deserialize;
/// 
/// #[derive(FromRow)]
/// struct Book {
///   pub isbn: String,
///   pub name: String,
///   pub author: Author,
/// }
/// 
/// #[derive(Deserialize, FromJson)]
/// #[serde(rename_all = "camelCase")]
/// struct Author {
///   pub first_name: String,
///   pub last_name: String,
/// }
/// 
/// fn get_book_by_id(client: &mut postgres::Client, id: &str) -> Result<Book, postgres::Error> {
///   let row = client.query_one("SQL", &[ &id ])?;
///   Ok((&row).into())
/// }
/// ```
#[proc_macro_derive(FromJson)]
pub fn from_json_derive(input: TokenStream) -> TokenStream {
  TokenStream::from(macro_from_json::execute_macro(parse_macro_input!(input as DeriveInput)))
}