use crate::schema::subscriptions;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = subscriptions)]
pub struct Subscription {
    pub id: i32,
    pub fk_users: i32,
    pub fk_tournaments: i32,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = subscriptions)]
pub struct NewSubscription {
    pub fk_users: i32,
    pub fk_tournaments: i32,
}

