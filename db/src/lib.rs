#[macro_use]
extern crate diesel;

mod connection;
mod model;
mod repository;
mod schema;
mod updator;

pub use repository::*;

pub fn setup() {
    dotenv::dotenv().ok();
}
