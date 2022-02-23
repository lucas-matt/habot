#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use std::borrow::Borrow;
use std::ops::Deref;
use crate::data::UserRepository;
use crate::model::User;

mod routes;
mod data;
mod model;

fn main() {
    let db = data::db();
    db.update(User{
        id: Some("123".to_string()),
        name: "Bert".to_string(),
        habits: vec![]
    });
    rocket::ignite()
        .mount("/api", routes![routes::hello])
        .launch();
}
