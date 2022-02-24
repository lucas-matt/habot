use log::{info, warn};
use rocket::http::{RawStr, Status};
use rocket::State;
use rocket_contrib::json::Json;

use crate::data::UberRepository;
use crate::model::User;

#[post("/users/<id>", data = "<json>")]
pub fn add_user(repo:State<Box<dyn UberRepository + Sync + Send>>, id: &RawStr, json:Json<User>) -> Result<Json<User>, Status> {
    info!("Submitted user {:?}", json);
    let mut user = json.into_inner();
    user.id = Some(id.to_string());

    let result = repo.update_user(&user);
    if let Err(err) = result {
        warn!("Encountered error {:?}", err);
        return Err(Status::InternalServerError)
    }

    Ok(Json(user))
}

#[get("/users/<id>")]
pub fn get_user(repo:State<Box<dyn UberRepository + Sync + Send>>, id: &RawStr) -> Result<Json<User>, Status> {
    info!("Find user {:?}", id);
    let result = repo.find_user(id.to_string());
    match result {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound)
    }
}