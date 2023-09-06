use crate::schema::games;
use rocket::serde::{Deserialize, Serialize};

use super::team::Team;


#[derive(Serialize, Deserialize, Queryable, Identifiable, Clone, Debug)]
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
    pub status: i32,
    pub has_gained_nut: bool,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Clone)]
#[diesel(belongs_to(Team))]
#[diesel(table_name = games)]
pub struct GameWithGroup {
    pub id: i32,
    pub fk_team1: i32,
    pub fk_team2: i32,
    pub score1: i32,
    pub score2: i32,
    pub phase: i32,
    pub place: i32,
    pub status: i32,
    pub has_gained_nut: bool,
    pub group: i32,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Clone)]
#[diesel(belongs_to(Team))]
#[diesel(table_name = games)]
pub struct GameWithTeams {
    pub id: i32,
    pub team1: Team,
    pub team2: Team,
    pub score1: i32,
    pub score2: i32,
    pub phase: i32,
    pub place: i32,
    pub status: i32,
    pub has_gained_nut: bool,
    pub group: i32,
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
    pub status: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(belongs_to(Team))]
#[diesel(table_name = games)]
pub struct PatchGame {
    pub fk_team1: Option<i32>,
    pub fk_team2: Option<i32>,
    pub place: Option<i32>,
    pub status: Option<i32>,
    pub has_gained_nut: Option<bool>,
}
