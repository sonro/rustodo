#[macro_use]
extern crate diesel;

mod connection;
mod model;
mod repository;
mod schema;
mod updator;

pub use repository::*;
