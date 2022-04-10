#![feature(plugin, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket<P> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![static_files::all, static_files::index])
}

fn main() {
    rocket().launch();
}
