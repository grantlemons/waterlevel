// use waterlevel_backend::routes::waterlevel::Input;

use rocket::{http::Status, local::blocking::Client};

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client.get("/api/v1/waterlevel/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_on_date() {
    let client = get_client();
    let response = client
        .get(format!("/api/v1/waterlevel/date/{}", "2022-08-25"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_at_level() {
    let client = get_client();
    let response = client
        .get(format!("/api/v1/waterlevel/level/at/{}", 5.0))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_above_level() {
    let client = get_client();
    let response = client
        .get(format!("/api/v1/waterlevel/level/above/{}", 5.0))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_below_level() {
    let client = get_client();
    let response = client
        .get(format!("/api/v1/waterlevel/level/below/{}", 5.0))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
