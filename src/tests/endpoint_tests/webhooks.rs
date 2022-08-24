// use crate::routes::webhooks;
use crate::routes::webhooks::Input;

#[test]
fn test_get_all() {}

#[test]
fn test_get_by_id() {}

#[test]
fn test_create() {
    let data = Input {
        url: String::from(""),
        event: String::from(""),
    };
}

#[test]
fn test_modify() {
    let data = Input {
        url: String::from(""),
        event: String::from(""),
    };
}
