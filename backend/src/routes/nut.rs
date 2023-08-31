use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::MysqlConnection;
use crate::models::nut::{Nut, PatchNut};
use crate::schema::nuts::{self, fk_tournaments, fk_users};

#[get("/tournoix/<id>/user/<id_user>/nut")]
pub async fn get_nut(
    connection: MysqlConnection,
    id: i32,
    id_user: i32,
) -> Result<Json<Nut>, (Status, String)> {
    match connection.run(
        move |c| nuts::table.filter(fk_tournaments.eq(id)).filter(fk_users.eq(id_user)).first::<Nut>(c)
    ).await.map(Json) {
        Ok(nut) => {
           return Ok(nut)
        },

        Err(_e) => {
            return Err((
                Status::NotFound,
                "Nuts not found".to_string()
            ))
        }
    }
}

#[patch("/nut/<id>", data = "<data>")]
pub async fn update_nut(
    connection: MysqlConnection,
    id: i32,
    data: Json<PatchNut>,
) -> Result<Json<Nut>, (Status, String)> {


    match connection.run(
       move |c| c.transaction(|c| {
            diesel::update(nuts::table.find(id))
                .set(data.0)
                .execute(c)?;

            let nut = nuts::table.order(nuts::id.desc()).first::<Nut>(c).map(Json)?;

            diesel::result::QueryResult::Ok(nut)
        })
    ).await {
        Ok(nut) => {
            return Ok(nut);
        },

        Err(_e) => {
            return Err((
                Status::InternalServerError,
                "No nuts found".to_string()
            ))
        }
    }
}