use waterlevel_backend::routes::config;
use waterlevel_backend::routes::config::Input;
use rocket::{http::Status, serde::json::Json};

#[test]
fn test_get_all() -> Result<(), Status> {
    match config::get_all() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn test_get_value() -> Result<(), Status> {
    match config::get_value("1") {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn test_create() -> Result<(), Status> {
    let data = Input {
        key: String::from(""),
        value: String::from(""),
    };
    match config::create(Json(data.clone())) {
        Ok(v) => {
            assert_eq!(v.key, data.key);
            assert_eq!(v.value, data.value);
            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}

#[test]
fn test_modify() -> Result<(), Status> {
    let data = Input {
        key: String::from(""),
        value: String::from(""),
    };
    match config::modify("1", Json(data.clone())) {
        Ok(v) => {
            assert_eq!(v.key, data.key);
            assert_eq!(v.value, data.value);
            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}
