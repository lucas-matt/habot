use log::{info, warn};
use mongodb::bson::{Bson, doc};
use mongodb::sync::Client;
use mongodb::sync::Collection;

use crate::data::Error::{ParseErr, ReadErr, WriteErr};
use crate::model::*;

pub trait UserRepository {

    fn find(&self, id: String) -> Option<User>;

    fn update(&self, user:&User) -> Result<(), Error>;

}

pub trait HabitRepository {

}

pub trait MetricRespository {

}

pub trait UberRepository: UserRepository + HabitRepository + MetricRespository {}

pub type Repository = dyn UberRepository + Send + Sync;

struct Creds {
    connection: String,
    database: String
}

fn read_creds() -> Creds {
    let connection = std::env::var_os("MONGO")
        .expect("Environment variable MONGO is required")
        .to_str()
        .expect("Failed to read MONGO")
        .to_string();
    let database = std::env::var_os("DB")
        .expect("Environment variable DB is required")
        .to_str()
        .expect("Failed to read DB")
        .to_string();
    return Creds {
        connection,
        database
    }
}

struct MongoDB {
    user: Collection<User>
}

impl MongoDB {
    fn new() -> MongoDB {
        let creds = read_creds();
        let client = Client::with_uri_str(creds.connection).expect("Failed to create client");
        let db = client.database(&*creds.database);
        MongoDB {
            user: db.collection("users")
        }
    }
}

impl UserRepository for MongoDB {
    fn find(&self, id: String) -> Option<User> {
        let result = self.user.find_one(doc! {
            "_id": &id
        }, None);
        match result {
            Ok(found) => found,
            Err(error) => {
                warn!("Reading user {:?} encountered error {:?}", &id, error);
                None
            }
        }
    }

    fn update(&self, user: &User) -> Result<(), Error> {
        self.user.insert_one(user, None)
            .ok()
            .ok_or(WriteErr)?;
        Ok(())
    }
}

impl HabitRepository for MongoDB {

}

impl MetricRespository for MongoDB {

}

impl UberRepository for MongoDB {

}

#[derive(Debug)]
pub enum Error {
    ParseErr,
    WriteErr,
    ReadErr
}

pub fn db() -> Box<Repository> {
    Box::new(MongoDB::new())
}
