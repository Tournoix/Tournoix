use crate::schema::teams;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i32,
    pub fk_tournaments: i32,
    pub name: String,
    pub group: i32,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = teams)]
pub struct NewTeam {
    pub fk_tournaments: i32,
    pub name: String,
    pub group: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(belongs_to(Tournament))]
#[diesel(table_name = teams)]
pub struct PatchTeam {
    pub name: Option<String>,
    pub group: Option<i32>,
}
