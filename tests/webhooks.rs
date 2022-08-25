use waterlevel_backend::routes::webhooks::Input;
use rocket::{http::Status, local::blocking::Client};
use rocket;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client
        .get("/api/v1/webhooks/")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

// #[test]
// fn test_get_by_id() {
    // let client = get_client();
    // let response = client
    // .get(format!("/api/v1/webhooks/{}", 2))
    // .dispatch();
    // assert_eq!(response.status(), Status::Ok);
// }

#[test]
fn test_create() {
    let data = Input {
        url: String::from(""),
        event: String::from(""),
    };
    let client = get_client();
    let response = client
        .post("/api/v1/webhooks/")
        .body(bincode::serialize(&data).expect("Unable to serialize input"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_modify() {
    let data = Input {
        url: String::from(""),
        event: String::from(""),
    };
    let client = get_client();
    let response = client
        .put(format!("/api/v1/webhooks/{}", 2))
        .body(bincode::serialize(&data).expect("Unable to serialize input"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
