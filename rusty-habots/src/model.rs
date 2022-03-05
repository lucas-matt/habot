use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Habit {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", alias="id")]
    pub id: Option<String>,

    #[serde(rename = "user")]
    pub user: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "action")]
    pub action: String,

    #[serde(rename = "action")]
    pub habit: String,

    #[serde(rename = "type")]
    pub habit_type: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", alias="id")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", alias="id")]
    pub id: Option<String>,

    #[serde(rename = "habit")]
    pub habit: String,

    #[serde(rename = "user")]
    pub user: String,

    #[serde(rename = "timestamp")]
    pub time: u32

}