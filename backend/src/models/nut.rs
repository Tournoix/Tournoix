use crate::schema::nuts;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = nuts)]
pub struct Nut {
    pub id: i32,
    pub fk_users: i32,
    pub fk_tournaments: i32,
    pub stock: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = nuts)]
pub struct NewNut {
    pub fk_users: i32,
    pub fk_tournaments: i32,
    pub stock: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = nuts)]
pub struct PatchNut {
    pub stock: Option<i32>,
}