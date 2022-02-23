use rocket::response::content::Json;
use rocket::State;
use crate::data::MongoDB;
use crate::{User, UserRepository};

#[get("/hello")]
pub fn hello(db:State<MongoDB>) -> Json<&'static str> {
    db.update(User{
        id: Some("123".to_string()),
        name: "Bert".to_string(),
        habits: vec![]
    });
    Json("{
      'status': 'success',
      'message': 'Hello API!'
    }")
}