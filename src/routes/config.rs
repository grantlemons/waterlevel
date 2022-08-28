//! Handlers for getting, creating, and modifying configuration key-value pairs in the database

use rocket::{http::Status, serde::json::Json, State};

use crate::diesel::prelude::*;
use crate::models::Config;
use crate::schema::config::table;

use crate::helpers::*;

#[get("/")]
/// Gets all configuration pairs set in database
pub async fn get_all(db: &State<Database>) -> Result<Json<Vec<Config>>, Status> {
    let connection = get_connection(db);
    get_json_vec(table.load::<Config>(&connection), None)
}

#[get("/<key>")]
/// Gets the value of a speficic key from the database
///
/// # Parameters
///
/// `/<key>` configuration option name as string
pub async fn get_value(key: &str, db: &State<Database>) -> Result<Json<Vec<Config>>, Status> {
    let connection = get_connection(db);
    get_json_vec(table.find(key).load::<Config>(&connection), None)
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
/// Form for inputs to put and post handlers in [config](self)
pub struct ConfigForm {
    /// Configuration option name as string
    pub key: String,
    /// Configuration value as string
    pub value: String,
}

#[post("/", format = "json", data = "<data>")]
/// Create new configuration key-value pair in the database
///
/// # Body
///
/// Body data should match [`ConfigForm`]
pub async fn create(
    data: Json<ConfigForm>,
    db: &State<Database>,
) -> Result<Json<Vec<Config>>, Status> {
    let connection = get_connection(db);
    let new_config = Config {
        key: data.key.clone(),
        value: data.value.clone(),
        timestamp: chrono::Utc::now().naive_utc(),
    };

    get_json_vec(
        diesel::insert_into(table)
            .values(&new_config)
            .get_results::<Config>(&connection),
        None,
    )
}

#[put("/<key>", format = "json", data = "<data>")]
/// Modify an existing key-value pair in the database
///
/// # Parameters
///
/// `/<key>` configuration option name as string
///
/// # Body
///
/// Body data should match [`ConfigForm`]
pub async fn modify(
    key: &str,
    data: Json<ConfigForm>,
    db: &State<Database>,
) -> Result<Json<Vec<Config>>, Status> {
    let connection = get_connection(db);
    let new_config = Config {
        key: String::from(key),
        value: data.value.clone(),
        timestamp: chrono::Utc::now().naive_utc(),
    };

    let target = table.find(key);
    get_json_vec(
        diesel::update(target)
            .set(&new_config)
            .get_results::<Config>(&connection),
        None,
    )
}
