use log::info;
use rocket::{local::blocking::Client, fairing::Fairing, http::hyper};
use crate::routes::auth::{LoginResponse, register};

use super::super::rocket;
use rocket::http::{Status, ContentType};

#[cfg(test)]

fn client() -> Client {
    Client::tracked(rocket()).expect("valid rocket instance")
}

/*
#[test]
fn successful_register_request() {
    const TEST_USER_EMAIL: &str = "john.doe@tournoix.com";
    const TEST_USER_PASSWORD: &str ="Password123!";
    const TEST_USER_NAME: &str = "John Doe";

    use log::info;

    use crate::models::user::NewUser;

    let c = client();
    
    let register_request = NewUser {
        email: TEST_USER_EMAIL.to_owned(),
        password: TEST_USER_PASSWORD.to_owned(),
        name: TEST_USER_NAME.to_owned()
    };

    let json_register_request = serde_json::to_string(&register_request);

    let response = c.post("/api/auth/register")
        .header(ContentType::JSON)
        .body(json_register_request.unwrap())
        .dispatch();


    info!("Register Response: {:?}", response);
    assert_eq!(response.status(), Status::Ok);
}

 */
#[test]
fn successful_login_logoff_request(){
    use crate::routes::auth::LoginRequest;
    use rocket::http::Header;
    use crate::models::user::NewUser;


    std::thread::sleep(std::time::Duration::from_secs(3));

    const TEST_USER_EMAIL: &str = "john.doe2@tournoix.com";
    const TEST_USER_PASSWORD: &str ="Password123!2";
    const TEST_USER_NAME: &str = "John Doe 2";

    let c = client();

    let json_register_request = "{\"email\":\"TEST_USER_EMAIL\",\"password\":\"TEST_USER_PASSWORD\",\"name\":\"TEST_USER_NAME\"}";

    let response = c.post("/api/auth/register")
        .header(ContentType::JSON)
        .body(json_register_request)
        .dispatch();

    // Wait for user to be registered
    std::thread::sleep(std::time::Duration::from_secs(3));


    info!("Register Response: {:?}", response);
    assert_eq!(response.status(), Status::Ok);
    
    let json_login_request = "{\"email\":\"TEST_USER_EMAIL\",\"password\":\"TEST_USER_PASSWORD\"}";

    let response = c.post("/api/auth/login")
        .header(ContentType::JSON)
        .body(json_login_request)
        .dispatch();

    info!("Login Response: {:?}", response);
    assert_eq!(response.status(), Status::Ok);

    // Save token from response
    // "{\"token\":\"f35e9be6-db56-4e0a-8d26-82abb507e828\",\"expiration_date\":\"2023-09-08T02:41:49.201130\"}"
    let token = response.into_string().unwrap().split(":").nth(1).unwrap().split(",").nth(0).unwrap().replace("\"", "");
    
    info!("Token: {:?}", token);

    let headers = Header::new("Authorization", format!("Bearer {}", token));

    std::thread::sleep(std::time::Duration::from_secs(3));


    let response = c.post("/api/auth/logout")
        .header(ContentType::JSON)
        .header(headers)
        .dispatch();

    info!("Logout Response: {:?}", response);
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn unsuccessful_login_request() {
    const TEST_USER_EMAIL: &str = "john.doe3@tournoix.com";
    const TEST_USER_PASSWORD: &str ="WrongPassword123!";

    use crate::routes::auth::LoginRequest;
    let c = client();
    
    let login_request = LoginRequest {
        email: TEST_USER_EMAIL.to_owned(),
        password: TEST_USER_PASSWORD.to_owned() 
    };
    
    let json_login_request = serde_json::to_string(&login_request);



    let response = c.post("/api/auth/login")
        .header(ContentType::JSON)
        .body(json_login_request.unwrap())
        .dispatch();

    info!("Login Response: {:?}", response);
    assert_eq!(response.status(), Status::Unauthorized);
}


/* #[test]
fn successful_logout_request() {
    use rocket::http::Header;
    
    let c = client();

    // Create header

    let mut headers = Header::new("Authorization", format!("Bearer {}", "14b08dc3-192e-441f-bb06-ad2bf09960eb"));

    let response = c.post("/api/auth/logout")
        .header(ContentType::JSON)
        .header(headers)
        .dispatch();

    info!("Logout Response: {:?}", response);
    assert_eq!(response.status(), Status::Ok);
} */

// #[test]
// fn unsuccessful_logout_request() {
//     use rocket::http::Header;
    
//     let c = client();

//     // Create header

//     let mut headers = Header::new("Authorization", format!("Bearer {}", "14b08dc3-192e-441f-bb06-ad2bf09960eb"));

//     let response = c.post("/api/auth/logout")
//         .header(ContentType::JSON)
//         .header(headers)
//         .dispatch();

//     info!("Logout Response: {:?}", response);
//     assert_eq!(response.status(), Status::Unauthorized);
// } 