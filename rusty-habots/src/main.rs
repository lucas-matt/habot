#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

mod data;
mod model;
mod routes;

fn main() {
    let db = data::db();
    rocket::ignite()
        .mount("/api", routes![routes::users::add_user])
        .mount("/api", routes![routes::users::get_user])
        .mount("/api", routes![routes::habits::add_habit])
        .mount("/api", routes![routes::habits::get_habits])
        .mount("/api", routes![routes::habits::remove_habit])
        .mount("/api", routes![routes::metrics::log_metric])
        .mount("/api", routes![routes::metrics::get_metrics])
        .manage(db)
        .launch();
}
