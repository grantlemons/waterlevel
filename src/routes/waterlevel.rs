use chrono::NaiveDate;

use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::WaterLevel;
use crate::schema::water_levels::columns;
use crate::schema::water_levels::table;

#[get("/")]
pub fn get_all() -> Result<Json<Vec<WaterLevel>>, Status> {
    use crate::schema::water_levels::table;
    crate::get_all::<table, WaterLevel>(table)
}

/// Gets all data recorded on a certain date
#[get("/date/<date>")]
pub fn get_on_date(date: &str) {
    // Gives different responses depending on the validity of the passed date
    let _date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            log!(Level::Info, "inputted date is {}", &date);
            Ok(Json(format!("response: {}", &date)))
        }
        Err(e) => {
            log!(Level::Error, "date from parameter failed: {}", e);
            Err(Status::BadRequest)
        }
    };

    let connection = crate::establish_connection();
    table
        //.filter(columns::timestamp.date().eq(date))
        .load::<WaterLevel>(&connection)
        .expect("Error loading config");
}

#[get("/level/at/<level>")]
pub fn get_at_level(level: f32) {
    let connection = crate::establish_connection();
    table
        .filter(columns::level.eq(level as f64))
        .load::<WaterLevel>(&connection)
        .expect("Error loading config");
}

#[get("/level/above/<level>")]
pub fn get_above_level(level: f32) {
    let connection = crate::establish_connection();
    table
        .filter(columns::level.gt(level as f64))
        .load::<WaterLevel>(&connection)
        .expect("Error loading config");
}

#[get("/level/below/<level>")]
pub fn get_below_level(level: f32) {
    let connection = crate::establish_connection();
    table
        .filter(columns::level.lt(level as f64))
        .load::<WaterLevel>(&connection)
        .expect("Error loading config");
}
