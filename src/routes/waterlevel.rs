use chrono::NaiveDate;
use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::WaterLevel;
use crate::schema::water_levels::{dsl, table};

#[get("/")]
pub fn get_all() -> Result<Json<Vec<WaterLevel>>, Status> {
    use crate::schema::water_levels::table;
    crate::get_all::<table, WaterLevel>(table)
}

/// Gets all data recorded on a certain date
#[get("/date/<date>")]
pub fn get_on_date(date: &str) -> Result<Json<WaterLevel>, Status> {
    // Gives different responses depending on the validity of the passed date
    match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            log!(Level::Info, "inputted date is {}", &date);

            let connection = crate::establish_connection();

            //TODO: Restrict to entries that match day
            match table
                .load::<WaterLevel>(&connection)
            {
                Ok(mut v) => Ok(rocket::serde::json::Json(v.remove(0))),
                Err(_) => {
                    rocket::log::private::log!(
                        rocket::log::private::Level::Error,
                        "Unable to get records!"
                    );
                    Err(rocket::http::Status::InternalServerError)
                }
            }
        }
        Err(e) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "date from parameter failed: {}",
                e
            );
            Err(rocket::http::Status::BadRequest)
        }
    }
}

#[get("/level/at/<level>")]
pub fn get_at_level(level: f32) {
    let connection = crate::establish_connection();
    table
        .filter(dsl::level.eq(level as f64))
        .load::<WaterLevel>(&connection)
        .expect("Error loading config");
}

#[get("/level/above/<level>")]
pub fn get_above_level(level: f32) {
    let connection = crate::establish_connection();
    table
        .filter(dsl::level.gt(level as f64))
        .load::<WaterLevel>(&connection)
        .expect("Error loading config");
}

#[get("/level/below/<level>")]
pub fn get_below_level(level: f32) {
    let connection = crate::establish_connection();
    table
        .filter(dsl::level.lt(level as f64))
        .load::<WaterLevel>(&connection)
        .expect("Error loading config");
}
