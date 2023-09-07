use crate::schema::bets;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Clone)]
#[diesel(belongs_to(Nut))]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Team))]
#[diesel(table_name = bets)]
pub struct Bet {
    pub id: i32,
    pub fk_users: i32,
    pub fk_games: i32,
    pub fk_teams: i32,
    pub nb_nut: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(belongs_to(Nut))]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Team))]
#[diesel(table_name = bets)]
pub struct NewBet {
    pub fk_users: i32,
    pub fk_games: i32,
    pub fk_teams: i32,
    pub nb_nut: i32,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(belongs_to(Nut))]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Team))]
#[diesel(table_name = bets)]
pub struct PathBet {
    pub fk_teams: Option<i32>,
    pub nb_nut: Option<i32>,
}
