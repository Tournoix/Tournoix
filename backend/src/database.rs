use std::env;

use diesel::prelude::*;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}