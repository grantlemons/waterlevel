use rocket::{serde::json::Json, http::Status};

use crate::models::Config;
use crate::diesel::prelude::*;
use crate::schema::config::table;

#[get("/", format = "json")]
pub fn get_all() {
    let connection = crate::establish_connection();
    table.load::<Config>(&connection)
        .expect("Error loading config");
}

#[get("/<key>", format = "json")]
pub fn get_value(key: &str) -> Result<Json<Vec<Config>>, Status> {
    use crate::schema::config::table;
    crate::get_by_id::<table, Config, &str>(table, key)
}

#[derive(serde::Deserialize)]
pub struct Input {
    key: String,
    value: String
}

#[post("/", format="json", data="<data>")]
pub fn create(data: Json<Input>) {
    let connection = crate::establish_connection();
    let new_config = Config {
        key: data.key.clone(),
        value: data.value.clone(),
        timestamp: chrono::Utc::now().naive_utc()
    };

    diesel::insert_into(table)
        .values(&new_config)
        .get_results::<Config>(&connection)
        .expect("Error creating new config value");
}

#[put("/<key>", format="json", data="<data>")]
pub fn modify(key: &str, data: Json<Input>) {
    let connection = crate::establish_connection();
    let new_config = Config {
        key: String::from(key),
        value: data.value.clone(),
        timestamp: chrono::Utc::now().naive_utc()
    };

    diesel::insert_into(table)
        .values(&new_config)
        .get_results::<Config>(&connection)
        .expect("Error editing config value");
}
