use chrono::NaiveDate;

use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

#[get("/")]
pub fn get_all() {}

/// Gets all data recorded on a certain date
#[get("/date/<date>")]
pub fn get_on_date(date: &str) -> Result<Json<String>, Status> {
    // Gives different responses depending on the validity of the passed date
    let response = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            log!(Level::Info, "inputted date is {}", &date);
            Ok(Json(format!("response: {}", &date)))
        }
        Err(e) => {
            log!(Level::Error, "date from parameter failed: {}", e);
            Err(Status::BadRequest)
        }
    };
    response
}

#[get("/level/at/<level>")]
pub fn get_at_level(level: i16) {}

#[get("/level/above/<level>")]
pub fn get_above_level(level: i16) {}

#[get("/level/below/<level>")]
pub fn get_below_level(level: i16) {}
