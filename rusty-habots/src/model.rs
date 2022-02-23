use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Habit {

}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "habits")]
    pub habits: Vec<Habit>
}
