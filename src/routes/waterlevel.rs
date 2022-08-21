use chrono::NaiveDate;
use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::WaterLevel;
use crate::schema::water_levels::{dsl, table};

#[get("/")]
pub fn get_all() -> Result<Json<Vec<WaterLevel>>, Status> {
    use crate::schema::water_levels::table;
    crate::lib::get_all::<table, WaterLevel>(table)
}

/// Gets all data recorded on a certain date
#[get("/date/<date>")]
pub fn get_on_date(date: &str) -> Result<Json<Vec<WaterLevel>>, Status> {
    // Gives different responses depending on the validity of the passed date
    match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            log!(Level::Info, "inputted date is {}", &date);

            let connection = crate::lib::establish_connection();

            //TODO: Restrict to entries that match day
            let res = table.load::<WaterLevel>(&connection);
            crate::lib::get_json_vec(res, None)
        }
        Err(e) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "date from parameter failed: {:?}",
                e
            );
            Err(rocket::http::Status::BadRequest)
        }
    }
}

#[get("/level/at/<level>")]
pub fn get_at_level(level: f32) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = crate::lib::establish_connection();
    crate::lib::get_json_vec(
        table
            .filter(dsl::level.eq(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

#[get("/level/above/<level>")]
pub fn get_above_level(level: f32) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = crate::lib::establish_connection();
    crate::lib::get_json_vec(
        table
            .filter(dsl::level.gt(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

#[get("/level/below/<level>")]
pub fn get_below_level(level: f32) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = crate::lib::establish_connection();
    crate::lib::get_json_vec(
        table
            .filter(dsl::level.eq(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}
