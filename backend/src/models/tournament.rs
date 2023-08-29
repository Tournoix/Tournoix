use crate::schema::tournaments;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tournaments)]
pub struct Tournament {
    pub id: i32,
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: String,
    pub phase: i32,
    pub size_group: Option<i32>,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tournaments)]
pub struct NewTournament {
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: String,
    pub phase: i32,
    pub size_group: Option<i32>,
}