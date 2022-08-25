use waterlevel_backend::routes::waterlevel;
// use waterlevel_backend::routes::waterlevel::Input;
use rocket::{http::Status, local::blocking::Client, uri};
use rocket;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client
        .get(uri!(waterlevel::get_all))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_on_date() {
    let client = get_client();
    let response = client
        .get(uri!(waterlevel::get_on_date("2022-08-25")))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_at_level() {
    let client = get_client();
    let response = client
        .get(uri!(waterlevel::get_at_level(5.0)))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_above_level() {
    let client = get_client();
    let response = client
        .get(uri!(waterlevel::get_above_level(5.0)))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_below_level() {
    let client = get_client();
    let response = client
        .get(uri!(waterlevel::get_below_level(5.0)))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
