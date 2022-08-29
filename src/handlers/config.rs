//! Handlers for getting, creating, and modifying configuration key-value pairs in the database

use rocket::{http::Status, serde::json::Json, State};

use crate::models::Config;
use crate::schema::config::table;
use crate::{diesel::prelude::*, helpers};

use crate::helpers::*;

/// Gets all configuration pairs set in database
#[get("/")]
pub async fn get_all(db: &State<Database>) -> Result<Json<Vec<Config>>, Status> {
    let connection = get_connection(db);
    get_json_vec(table.load::<Config>(&connection), None)
}

/// Gets the value of a speficic key from the database
///
/// # Parameters
///
/// `/<key>` configuration option name as string
#[get("/<key>")]
pub async fn get_value(key: &str, db: &State<Database>) -> Result<Json<Vec<Config>>, Status> {
    let connection = get_connection(db);
    get_json_vec(table.find(key).load::<Config>(&connection), None)
}

/// Form for inputs to put and post handlers in [config](self)
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ConfigForm {
    /// Configuration option name as string
    pub key: String,
    /// Configuration value as string
    pub value: String,
}

/// Create new configuration key-value pair in the database
///
/// # Body
///
/// Body data should match [`ConfigForm`]
#[post("/", format = "json", data = "<data>")]
pub async fn create(
    data: Json<ConfigForm>,
    db: &State<Database>,
) -> Result<Json<Vec<Config>>, Status> {
    helpers::trigger_webhooks(WebhookEvent::CreateConfig);

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

/// Modify an existing key-value pair in the database
///
/// # Parameters
///
/// `/<key>` configuration option name as string
///
/// # Body
///
/// Body data should match [`ConfigForm`]
#[put("/<key>", format = "json", data = "<data>")]
pub async fn modify(
    key: &str,
    data: Json<ConfigForm>,
    db: &State<Database>,
) -> Result<Json<Vec<Config>>, Status> {
    helpers::trigger_webhooks(WebhookEvent::ModifyConfig);

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
