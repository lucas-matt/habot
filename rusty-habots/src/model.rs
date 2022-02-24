use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Habit {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none", alias="id")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "habits", skip_serializing_if = "Option::is_none")]
    pub habits: Option<Vec<Habit>>
}
