
use super::super::rocket;
use serial_test::serial;

#[cfg(test)]

fn client() -> rocket::local::blocking::Client {
    rocket::local::blocking::Client::tracked(rocket()).expect("valid rocket instance")
}


#[test]
#[serial]
fn successful_register_request() {
    use rocket::http::{Status, ContentType};

    const TEST_USER_EMAIL: &str = "john.doe@tournoix.com";
    const TEST_USER_PASSWORD: &str ="Password123!";
    const TEST_USER_NAME: &str = "John Doe";

    let c = client();
    

    let json_register_request = format!("{{\"email\":\"{}\",\"password\":\"{}\",\"name\":\"{}\"}}", TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_NAME);

    let response = c.post("/api/auth/register")
        .header(ContentType::JSON)
        .body(json_register_request)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
#[serial]
fn successful_login_logoff_request(){
    use rocket::http::Header;
    use rocket::http::{Status, ContentType};

    std::thread::sleep(std::time::Duration::from_secs(3));

    const TEST_USER_EMAIL: &str = "john.doe2@tournoix.com";
    const TEST_USER_PASSWORD: &str ="Password123!2";
    const TEST_USER_NAME: &str = "John Doe 2";

    let c = client();

    let json_register_request = format!("{{\"email\":\"{}\",\"password\":\"{}\",\"name\":\"{}\"}}", TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_NAME);

    let response = c.post("/api/auth/register")
        .header(ContentType::JSON)
        .body(json_register_request)
        .dispatch();

    // Wait for user to be registered
    std::thread::sleep(std::time::Duration::from_secs(3));

    assert_eq!(response.status(), Status::Ok);
    
    let json_login_request = format!("{{\"email\":\"{}\",\"password\":\"{}\"}}", TEST_USER_EMAIL, TEST_USER_PASSWORD);

    let response = c.post("/api/auth/login")
        .header(ContentType::JSON)
        .body(json_login_request)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    // Save token from response
    // "{\"token\":\"f35e9be6-db56-4e0a-8d26-82abb507e828\",\"expiration_date\":\"2023-09-08T02:41:49.201130\"}"
    let token = response.into_string().unwrap().split(":").nth(1).unwrap().split(",").nth(0).unwrap().replace("\"", "");
    

    let headers = Header::new("Authorization", format!("Bearer {}", token));

    std::thread::sleep(std::time::Duration::from_secs(3));


    let response = c.post("/api/auth/logout")
        .header(ContentType::JSON)
        .header(headers)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
#[serial]
fn unsuccessful_login_request() {
    use rocket::http::{Status, ContentType};

    const TEST_USER_EMAIL: &str = "john.doe3@tournoix.com";
    const TEST_USER_PASSWORD: &str ="WrongPassword123!";
    let c = client();
    
    
    let json_login_request = format!("{{\"email\":\"{}\",\"password\":\"{}\"}}", TEST_USER_EMAIL, TEST_USER_PASSWORD);



    let response = c.post("/api/auth/login")
        .header(ContentType::JSON)
        .body(json_login_request)
        .dispatch();

    info!("Login Response: {:?}", response);
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
#[serial]
fn unsuccessful_logout_request() {
    use rocket::http::Header;
    use rocket::http::{Status, ContentType};
    
    let c = client();

    // Create header

    let headers = Header::new("Authorization", format!("Bearer {}", "14b08dc3-192e-441f-bb06-ad2bf09960eb"));

    let response = c.post("/api/auth/logout")
        .header(ContentType::JSON)
        .header(headers)
        .dispatch();

    info!("Logout Response: {:?}", response);
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
#[serial]
fn unsuccessful_register_request() {
    use rocket::http::{Status, ContentType};

    const TEST_USER_EMAIL: &str = "john.doe@tournoix.com";
    const TEST_USER_PASSWORD: &str ="Password123!";
    const TEST_USER_NAME: &str = "John Doe";

    let c = client();
    

    let json_register_request = format!("{{\"email\":\"{}\",\"password\":\"{}\",\"name\":\"{}\"}}", TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_NAME);

    let response = c.post("/api/auth/register")
        .header(ContentType::JSON)
        .body(json_register_request)
        .dispatch();

    assert_eq!(response.status(), Status::Conflict);
}

#[test]
#[serial]
fn get_user_info_request() {
    use rocket::http::Header;
    use rocket::http::{Status, ContentType};

    std::thread::sleep(std::time::Duration::from_secs(3));

    const TEST_USER_EMAIL: &str = "john.doe4@tournoix.com";
    const TEST_USER_PASSWORD: &str ="Password123!4";
    const TEST_USER_NAME: &str = "John Doe 4";

    let c = client();

    let json_register_request = format!("{{\"email\":\"{}\",\"password\":\"{}\",\"name\":\"{}\"}}", TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_NAME);

    let response = c.post("/api/auth/register")
        .header(ContentType::JSON)
        .body(json_register_request)
        .dispatch();
    
    let json_login_request = format!("{{\"email\":\"{}\",\"password\":\"{}\"}}", TEST_USER_EMAIL, TEST_USER_PASSWORD);

    let response = c.post("/api/auth/login")
        .header(ContentType::JSON)
        .body(json_login_request)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    // Save token from response
    // "{\"token\":\"f35e9be6-db56-4e0a-8d26-82abb507e828\",\"expiration_date\":\"2023-09-08T02:41:49.201130\"}"
    let token = response.into_string().unwrap().split(":").nth(1).unwrap().split(",").nth(0).unwrap().replace("\"", "");
    

    let headers = Header::new("Authorization", format!("Bearer {}", token));

    let response = c.get("/api/users/@me")
        .header(ContentType::JSON)
        .header(headers.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response = c.get("/api/users/1000")
        .header(ContentType::JSON)
        .header(headers.clone())
        .dispatch();

    assert_eq!(response.status(), Status::Forbidden);
}