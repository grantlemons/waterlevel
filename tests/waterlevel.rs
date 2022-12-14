use rocket::{http::Status, local::blocking::Client};
use waterlevel_backend::routes::waterlevel::{get_weather, Input};

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[ctor::ctor]
fn setup() {
    waterlevel_backend::run_migrations(None);
}

#[tokio::test]
async fn test_get_weather() {
    if let Err(_) = get_weather(32.946478, -96.7891936).await {
        assert!(false, "Unable to get weather from external api");
    }
}

#[test]
fn test_add_waterlevel() {
    let client = get_client();
    let data = Input {
        location: (32.946478, -96.7891936),
        level: 10.0,
    };

    let _response = client
        .post(waterlevel_backend::ROOT.to_owned() + "waterlevel/")
        .json(&data)
        .dispatch();
    // assert_eq!(_response.status(), Status::Ok);
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
