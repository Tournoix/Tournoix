use crate::schema::games;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(belongs_to(Team))]
#[diesel(table_name = games)]
pub struct Game {
    pub id: i32,
    pub fk_team1: i32,
    pub fk_team2: i32,
    pub score1: i32,
    pub score2: i32,
    pub phase: i32,
    pub place: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(belongs_to(Team))]
#[diesel(table_name = games)]
pub struct NewGame {
    pub fk_team1: i32,
    pub fk_team2: i32,
    pub score1: i32,
    pub score2: i32,
    pub phase: i32,
    pub place: i32,
}