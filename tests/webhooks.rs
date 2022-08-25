use waterlevel_backend::routes::webhooks;
use waterlevel_backend::routes::webhooks::Input;
use rocket::{http::Status, serde::json::Json};

#[test]
fn test_get_all() -> Result<(), Status> {
    match webhooks::get_all() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

// #[test]
// fn test_get_by_id() -> Result<(), Status> {
//     match webhooks::get_by_id("1") {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e)
//     }
// }

#[test]
fn test_create() -> Result<(), Status> {
    let data = Input {
        url: String::from(""),
        event: String::from(""),
    };
    match webhooks::create(Json(data.clone())) {
        Ok(v) => {
            assert_eq!(v.url, data.url);
            assert_eq!(v.event, data.event);
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
        url: String::from(""),
        event: String::from(""),
    };
    match webhooks::modify("2", Json(data.clone())) {
        Ok(v) => {
            assert_eq!(v.url, data.url);
            assert_eq!(v.event, data.event);
            Ok(())
        },
        Err(e) => {
            Err(e)
        }
    }
}
