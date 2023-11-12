mod macro_from_json;
mod macro_from_row;
mod macro_from_rows;

use postgres::{Client, NoTls};

pub fn connect() -> Client {
  Client::connect(CONNECTION_STRING, NoTls).unwrap()
}

const CONNECTION_STRING: &str = "postgresql://postgres:123456@localhost:25432/main";