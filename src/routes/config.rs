use rocket::{http::Status, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::Config;
use crate::schema::config::table;

use crate::lib::*;

#[get("/")]
pub fn get_all() {
    let connection = establish_connection();
    table
        .load::<Config>(&connection)
        .expect("Error loading config");
}

#[get("/<key>")]
pub fn get_value(key: &str) -> Result<Json<Config>, Status> {
    get_by_id::<table, Config, &str>(table, key)
}

#[derive(serde::Deserialize)]
pub struct Input {
    key: String,
    value: String,
}

#[post("/", format = "json", data = "<data>")]
pub fn create(data: Json<Input>) -> Result<Json<Config>, Status> {
    let connection = establish_connection();
    let new_config = Config {
        key: data.key.clone(),
        value: data.value.clone(),
        timestamp: chrono::Utc::now().naive_utc(),
    };

    get_json(
        diesel::insert_into(table)
            .values(&new_config)
            .get_results::<Config>(&connection),
        None,
    )
}

#[put("/<key>", format = "json", data = "<data>")]
pub fn modify(key: &str, data: Json<Input>) -> Result<Json<Config>, Status> {
    let connection = establish_connection();
    let new_config = Config {
        key: String::from(key),
        value: data.value.clone(),
        timestamp: chrono::Utc::now().naive_utc(),
    };

    get_json(
        diesel::insert_into(table)
            .values(&new_config)
            .get_results::<Config>(&connection),
        None,
    )
}
