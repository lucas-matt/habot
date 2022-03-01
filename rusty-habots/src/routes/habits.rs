use crate::data::UberRepository;
use crate::model::Habit;
use log::{info, warn};
use rocket::http::{RawStr, Status};
use rocket::State;
use rocket_contrib::json::Json;

#[post("/users/<user_id>/habits", data = "<json>")]
pub fn add_habit(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    user_id: &RawStr,
    json: Json<Habit>,
) -> Result<Json<Habit>, Status> {
    info!("Submitted habit {:?}", json);
    let mut habit = json.into_inner();
    habit.user = user_id.to_string();
    let result = repo.add_habit(&mut habit);
    if let Err(err) = result {
        warn!("Encountered error {:?}", err);
        return Err(Status::InternalServerError);
    }
    Ok(Json(habit))
}

#[delete("/users/<_user_id>/habits/<habit_id>")]
pub fn remove_habit(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    _user_id: &RawStr,
    habit_id: &RawStr,
) -> Status {
    info!("Submitted habit deletion {:?}", habit_id);
    let result = repo.remove_habit(habit_id.to_string());
    if let Err(err) = result {
        warn!("Encountered error {:?}", err);
        return Status::InternalServerError;
    }
    Status::Ok
}

#[get("/users/<user_id>/habits")]
pub fn get_habits(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    user_id: &RawStr,
) -> Result<Json<Vec<Habit>>, Status> {
    info!("Find habits for user {:?}", user_id);
    let result = repo.find_habits(user_id.to_string());
    match result {
        Some(habits) => Ok(Json(habits)),
        None => Err(Status::NotFound),
    }
}
