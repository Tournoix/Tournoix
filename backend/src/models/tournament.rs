use crate::{schema::{tournaments, subscriptions}, MysqlConnection};
use diesel::{QueryDsl, ExpressionMethods};
use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

use super::{user::UserInfo, subscription::Subscription};

#[derive(Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tournaments)]
pub struct Tournament {
    pub id: i32,
    pub fk_users: i32,
    pub name: String,
    pub description: String,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub phase: i32,
    pub size_group: Option<i32>,
    pub code: String
}

impl Tournament {
    pub async fn user_has_rights(&self, connection: &MysqlConnection, user: UserInfo) -> bool {
        if user.id == self.fk_users {
            return true;
        }

        let tournoix_id = self.id;
        let user_id = user.id;
        match connection.run(move |c| subscriptions::table.filter(subscriptions::fk_tournaments.eq(tournoix_id)).filter(subscriptions::fk_users.eq(user_id)).first::<Subscription>(c)).await {
            Ok(_subscription) => true,
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tournaments)]
pub struct NewTournament {
    pub fk_users: i32,
    pub name: String,
    pub description: Option<String>,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub phase: i32,
    pub size_group: Option<i32>,
    pub code: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tournaments)]
pub struct PatchTournament {
    pub name: Option<String>,
    pub description: Option<String>,
    pub date: Option<chrono::NaiveDateTime>,
    pub location: Option<String>,
    pub phase: Option<i32>,
    pub size_group: Option<i32>,
}
