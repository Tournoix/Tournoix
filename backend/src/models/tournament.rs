use crate::schema::tournaments;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = tournaments)]
pub struct Tournament {
    pub id: i32,
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDate>,
    pub location: String,
    pub phase: i32,
    pub size_group: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = tournaments)]
pub struct NewTournament {
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDate>,
    pub location: String,
    pub phase: i32,
    pub size_group: i32,
}