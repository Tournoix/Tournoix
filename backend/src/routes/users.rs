use rocket::http::Status;

use crate::MysqlConnection;

#[get("/users/<id>")]
pub async fn get_user(
    _connection: MysqlConnection,
    id: String,
) -> Result<String, (Status, String)> {
    Ok(format!("User: {}", id))
}
