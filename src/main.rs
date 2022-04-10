#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
    println!("Hello, world!");
}
