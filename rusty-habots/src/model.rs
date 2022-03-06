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

    #[serde(rename = "habit")]
    pub habit: String,

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

    #[serde(skip_serializing_if = "Option::is_none", rename = "timestamp")]
    pub time: Option<u64>

}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Charts {

    #[serde(rename = "progress")]
    pub progress: String,

    // #[serde(rename = "timeline")]
    // pub timeline: String

}


