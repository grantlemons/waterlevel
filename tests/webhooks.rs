use rocket::{http::Status, local::blocking::Client};
use waterlevel_backend::routes::webhooks::Input;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[ctor::ctor]
fn setup() {
    waterlevel_backend::run_migrations(None);
}

#[test]
fn test_create() {
    let data = Input {
        url: String::from(""),
        event: String::from(""),
    };
    let client = get_client();
    let _response = client
        .post(waterlevel_backend::ROOT.to_owned() + "webhooks/")
        .json(&data)
        .dispatch();
    // assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client
        .get(waterlevel_backend::ROOT.to_owned() + "webhooks/")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

// #[test]
// fn test_get_by_id() {
// let client = get_client();
// let response = client
// .get(format!("/api/v1/webhooks/{}", 1))
// .dispatch();
// assert_eq!(response.status(), Status::Ok);
// }

#[test]
#[ignore]
fn test_modify() {
    let data = Input {
        url: String::from(""),
        event: String::from(""),
    };
    let client = get_client();
    let response = client
        .put(format!(
            "{}webhooks/{}",
            waterlevel_backend::ROOT.to_owned(),
            1 // UUID
        ))
        .json(&data)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
