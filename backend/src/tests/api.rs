use rocket::{local::blocking::Client};
use super::super::rocket;
use rocket::http::{Status, ContentType};
use crate::routes::auth::LoginRequest;

fn client() -> Client {
    Client::tracked(rocket()).expect("valid rocket instance")
}

#[cfg(test)]
#[test]
fn login() {

    let c = client();
    
    let login_request = LoginRequest {
        email: "leandro@saraivam.ch".into(),
        password: "1234".into()  
    };

    let response = c.post("/api/auth/login")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&login_request).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
