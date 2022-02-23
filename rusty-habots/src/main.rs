#![feature(decl_macro)]
#[macro_use] extern crate rocket;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/api", routes![routes::hello])
        .launch();
}
