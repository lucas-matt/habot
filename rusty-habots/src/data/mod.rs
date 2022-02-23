use mongodb::bson::{Bson, doc};
use mongodb::sync::{Client, Database};
use mongodb::sync::Collection;
use crate::data::Error::{ParseErr, WriteErr};
use crate::model::*;

pub trait UserRepository {

    fn find(&self, id: String) -> Option<User>;

    fn update(&self, user:User) -> Result<(), Error>;

}

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

pub struct MongoDB {
    user: Collection<User>
}

impl MongoDB {
    fn new() -> MongoDB {
        let creds = read_creds();
        let client = Client::with_uri_str(creds.connection).expect("Failed to create client");
        let mut db = client.database(&*creds.database);
        MongoDB {
            user: db.collection("users")
        }
    }
}

pub enum Error {
    ParseErr,
    WriteErr
}

impl UserRepository for MongoDB {
    fn find(&self, id: String) -> Option<User> {
        todo!()
    }

    fn update(&self, user: User) -> Result<(), Error>{
        self.user.insert_one(user, None)
            .ok()
            .ok_or(WriteErr)?;
        Ok(())
    }
}

pub fn db() -> MongoDB {
    MongoDB::new()
}
