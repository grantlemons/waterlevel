use rocket::{http::Status, local::blocking::Client};

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[ctor::ctor]
fn setup() {
    waterlevel_backend::run_migrations(None);
}

#[test]
fn test_health() {
    let client = get_client();
    let response = client
        .get(waterlevel_backend::ROOT.to_owned() + "health/")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}
