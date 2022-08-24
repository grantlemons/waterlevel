use rocket::{http::Status, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::Config;
use crate::schema::config::table;

use crate::lib::*;

#[get("/")]
pub fn get_all() -> Result<Json<Vec<Config>>, Status> {
    let connection = establish_connection();
    get_json_vec(table.load::<Config>(&connection), None)
}

#[get("/<key>")]
pub fn get_value(key: &str) -> Result<Json<Config>, Status> {
    let connection = establish_connection();
    get_json(table.find(key).load::<Config>(&connection), None)
}

#[derive(serde::Deserialize, Clone)]
pub struct Input {
    pub key: String,
    pub value: String,
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
