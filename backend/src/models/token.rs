use crate::schema::tokens;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
#[diesel(primary_key(token))]
#[diesel(belongs_to(User))]
#[diesel(table_name = tokens)]
pub struct Token {
    pub token: String,
    pub fk_users: i32,
    pub created_at: chrono::NaiveDateTime,
    pub expiration_date: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(primary_key(token))]
#[diesel(belongs_to(User))]
#[diesel(table_name = tokens)]
pub struct NewToken {
    pub token: String,
    pub fk_users: i32,
    pub expiration_date: chrono::NaiveDateTime,
}
