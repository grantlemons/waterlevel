use waterlevel_backend::routes::config;
use waterlevel_backend::routes::config::Input;
use rocket::{http::Status, local::blocking::Client, uri};
use rocket;
use bincode;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client
        .get(uri!(config::get_all))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_value() {
    let client = get_client();
    let response = client
        .get(uri!(config::get_value("2")))
        .dispatch();
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
        .post(uri!(config::create))
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
        .put(uri!(config::modify("2")))
        .header(rocket::http::ContentType::JSON)
        .body(bincode::serialize(&data).expect("Unable to serialize input"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
