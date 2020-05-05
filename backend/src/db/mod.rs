use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url: {}", database_url);

    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", &database_url))
}

pub fn create_task(connection: &PgConnection, title: & str ) {
    let task = models::NewTask { title };

    println!("create_task");
    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_tasks(connection: &PgConnection) -> Vec<models::Task> {
    schema::task::table
        .load(connection)
        .expect("Error loading tasks")

}