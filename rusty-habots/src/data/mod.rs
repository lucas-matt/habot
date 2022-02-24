use log::{info, warn};
use mongodb::bson::{Bson, doc};
use mongodb::sync::Client;
use mongodb::sync::Collection;

use crate::data::Error::{DeleteErr, ParseErr, ReadErr, WriteErr};
use crate::model::*;

pub trait UserRepository {

    fn find_user(&self, id: String) -> Option<User>;

    fn update_user(&self, user:&User) -> Result<(), Error>;

}

pub trait HabitRepository {

    fn add_habit(&self, habit:&Habit) -> Result<(), Error>;

    fn remove_habit(&self, id: String) -> Result<(), Error>;

    fn find_habit(&self, user_id: String) -> Option<Vec<Habit>>;

}

pub trait MetricRespository {

    fn log_metric(&self, metric: &Metric) -> Result<(), Error>;

    fn find_metrics(&self, habit_id: String) -> Option<Vec<Metric>>;

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
    users: Collection<User>,
    habits: Collection<Habit>,
    metrics: Collection<Metric>
}

impl MongoDB {
    fn new() -> MongoDB {
        let creds = read_creds();
        let client = Client::with_uri_str(creds.connection).expect("Failed to create client");
        let db = client.database(&*creds.database);
        MongoDB {
            users: db.collection("users"),
            habits: db.collection("habits"),
            metrics: db.collection("metrics")
        }
    }
}

impl UserRepository for MongoDB {
    fn find_user(&self, id: String) -> Option<User> {
        let result = self.users.find_one(doc! {
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

    fn update_user(&self, user: &User) -> Result<(), Error> {
        self.users.insert_one(user, None)
            .ok()
            .ok_or(WriteErr)?;
        Ok(())
    }
}

impl HabitRepository for MongoDB {
    fn add_habit(&self, habit: &Habit) -> Result<(), Error> {
        self.habits.insert_one(habit, None)
            .ok()
            .ok_or(WriteErr)?;
        Ok(())
    }

    fn remove_habit(&self, id: String) -> Result<(), Error> {
        self.habits.delete_one(doc! {"_id": id}, None)
            .ok()
            .ok_or(DeleteErr)?;
        Ok(())
    }

    fn find_habit(&self, user_id: String) -> Option<Vec<Habit>> {
        self.habits.find(doc! {"user": user_id}, None)

    }
}

impl MetricRespository for MongoDB {
    fn log_metric(&self, metric: &Metric) -> Result<(), Error> {
        todo!()
    }

    fn find_metrics(&self, habit_id: String) -> Option<Vec<Metric>> {
        todo!()
    }
}

impl UberRepository for MongoDB {

}

#[derive(Debug)]
pub enum Error {
    ParseErr,
    WriteErr,
    DeleteErr,
    ReadErr
}

pub fn db() -> Box<Repository> {
    Box::new(MongoDB::new())
}
