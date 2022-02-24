use crate::data::UberRepository;
use crate::model::Metric;
use log::{info, warn};
use rocket::http::{RawStr, Status};
use rocket::State;
use rocket_contrib::json::Json;

#[post("/users/<_user_id>/habits/<_habit_id>/metrics", data = "<json>")]
pub fn log_metric(
    repo: State<Box<dyn UberRepository + Sync + Send>>,
    _user_id: &RawStr,
    _habit_id: &RawStr,
    json: Json<Metric>,
) -> Result<Json<Metric>, Status> {
    info!("Submitted user {:?}", json);
    let metric = json.into_inner();
    let result = repo.log_metric(&metric);
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
