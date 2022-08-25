

use rocket::{http::Status, local::blocking::Client};
use waterlevel_backend::routes::config::Input;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client.get("/api/v1/config/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_value() {
    let client = get_client();
    let response = client.get(format!("/api/v1/config/{}", 2)).dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_create() {
    let data = Input {
        key: String::from(""),
        value: String::from(""),
    };
    let client = get_client();
    let response = client
        .post("/api/v1/config/")
        .header(rocket::http::ContentType::JSON)
        .body(bincode::serialize(&data).expect("Unable to serialize input"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_modify() {
    let data = Input {
        key: String::from(""),
        value: String::from(""),
    };
    let client = get_client();
    let response = client
        .put(format!("/api/v1/config/{}", 2))
        .header(rocket::http::ContentType::JSON)
        .body(bincode::serialize(&data).expect("Unable to serialize input"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
