use crate::routes::waterlevel;
// use crate::routes::waterlevel::Input;
use rocket::{http::Status, serde::json::Json};

#[test]
fn test_get_all() -> Result<(), Status> {
    match waterlevel::get_all() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn test_get_on_date() -> Result<(), Status> {
    match waterlevel::get_on_date("2022-08-24") {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn test_get_at_level() -> Result<(), Status> {
    match waterlevel::get_at_level(20 as f32) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn test_get_above_level() -> Result<(), Status> {
    match waterlevel::get_above_level(20 as f32) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[test]
fn test_get_below_level() -> Result<(), Status> {
    match waterlevel::get_below_level(20 as f32) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}
