use crate::data::UberRepository;
use crate::model::Metric;
use log::{info, warn};
use rocket::http::{RawStr, Status};
use rocket::State;
use rocket_contrib::json::Json;

#[post("/users/<user_id>/habits/<habit_id>/metrics", data = "<json>")]
pub fn log_metric(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    user_id: &RawStr,
    habit_id: &RawStr,
    json: Json<Metric>,
) -> Result<Json<Metric>, Status> {
    info!("Submitted user {:?}", json);
    let mut metric = json.into_inner();
    metric.user = user_id.to_string();
    metric.habit = habit_id.to_string();
    let result = repo.log_metric(&mut metric);
    if let Err(err) = result {
        warn!("Encountered error {:?}", err);
        return Err(Status::InternalServerError);
    }
    Ok(Json(metric))
}

#[get("/users/<_user_id>/habits/<habit_id>/metrics")]
pub fn get_metrics(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    _user_id: &RawStr,
    habit_id: &RawStr,
) -> Result<Json<Vec<Metric>>, Status> {
    info!("Find metrics for {:?}", habit_id);
    let result = repo.find_metrics(habit_id.to_string());
    match result {
        Some(metrics) => Ok(Json(metrics)),
        None => Err(Status::NotFound),
    }
}
