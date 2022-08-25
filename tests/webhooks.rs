use waterlevel_backend::routes::webhooks;
use waterlevel_backend::routes::webhooks::Input;
use rocket::{http::Status, local::blocking::Client, uri};
use rocket;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client
        .get(uri!(webhooks::get_all))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

// #[test]
// fn test_get_by_id() -> Result<(), Status> {
    // let client = get_client();
    // let response = client
    // .get(uri!(waterlevel::get_all))
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
        .post(uri!(webhooks::create))
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
        .put(uri!(webhooks::modify("2")))
        .body(bincode::serialize(&data).expect("Unable to serialize input"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
