use crate::schema::subscribers;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = subscribers)]
pub struct Subscriber {
    pub id: i32,
    pub fk_users: i32,
    pub fk_tournaments: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = subscribers)]
pub struct NewSubscriber {
    pub fk_users: i32,
    pub fk_tournaments: i32,
}

