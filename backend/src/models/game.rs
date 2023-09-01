use crate::schema::games;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Clone)]
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
    pub is_open: bool,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
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

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(belongs_to(Team))]
#[diesel(table_name = games)]
pub struct PatchGame {
    pub fk_team1: Option<i32>,
    pub fk_team2: Option<i32>,
    pub place: Option<i32>,
    pub is_open: Option<bool>,
}