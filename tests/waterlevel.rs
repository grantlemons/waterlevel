// use waterlevel_backend::routes::waterlevel::Input;

use rocket::{http::Status, local::blocking::Client};

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client
        .get(waterlevel_backend::ROOT.to_owned() + "waterlevel/")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_on_date() {
    let client = get_client();
    let response = client
        .get(format!(
            "{}waterlevel/date/{}",
            waterlevel_backend::ROOT.to_owned(),
            "2022-08-25"
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_at_level() {
    let client = get_client();
    let response = client
        .get(format!(
            "{}waterlevel/level/at/{}",
            waterlevel_backend::ROOT.to_owned(),
            5.0
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_above_level() {
    let client = get_client();
    let response = client
        .get(format!(
            "{}waterlevel/level/above/{}",
            waterlevel_backend::ROOT.to_owned(),
            5.0
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_below_level() {
    let client = get_client();
    let response = client
        .get(format!(
            "{}waterlevel/level/below/{}",
            waterlevel_backend::ROOT.to_owned(),
            5.0
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
