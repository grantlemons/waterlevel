use rocket::{http::Status, local::blocking::Client};
use waterlevel_backend::routes::config::Input;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_create() {
    let data = Input {
        key: String::from(""),
        value: String::from(""),
    };
    let client = get_client();
    let response = client
        .post(waterlevel_backend::ROOT.to_owned() + "config/")
        .header(rocket::http::ContentType::JSON)
        .json(&data)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_all() {
    let client = get_client();
    let response = client
        .get(waterlevel_backend::ROOT.to_owned() + "config/")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_get_value() {
    let client = get_client();
    let response = client
        .get(format!(
            "{}config/{}",
            waterlevel_backend::ROOT.to_owned(),
            1
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
#[ignore]
fn test_modify() {
    let data = Input {
        key: String::from(""),
        value: String::from(""),
    };
    let client = get_client();
    let response = client
        .put(format!(
            "{}config/{}",
            waterlevel_backend::ROOT.to_owned(),
            1 // UUID
        ))
        .header(rocket::http::ContentType::JSON)
        .json(&data)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
