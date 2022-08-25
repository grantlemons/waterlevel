use rocket::{http::Status, local::blocking::Client, uri};
use rocket;

fn get_client() -> Client {
    Client::tracked(waterlevel_backend::entrypoint()).expect("valid rocket instance")
}

#[test]
fn test_health() {
    let client = get_client();
    let response = client
        .get(uri!(waterlevel_backend::health))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}