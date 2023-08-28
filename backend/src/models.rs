use diesel::prelude::Queryable;
use rocket::serde::Serialize;


#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}