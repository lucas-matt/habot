#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use crate::data::{Repository};

mod routes;
mod data;
mod model;

fn main() {
    let db = data::db();
    rocket::ignite()
        .mount("/api", routes![routes::users::add_user])
        .mount("/api", routes![routes::users::get_user])
        .manage(db)
        .launch();
}
