//! Handlers for getting, creating, and modifying webhook configuration in the database

use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json, State};

use crate::diesel::prelude::*;
use crate::models::Webhook;
use crate::schema::webhooks::table;

use crate::helpers::*;

/// Get all webhook configurations from the database
#[get("/")]
pub async fn get_all(db: &State<Database>) -> Result<Json<Vec<Webhook>>, Status> {
    let connection = get_connection(db);
    get_json_vec(table.load::<Webhook>(&connection), None)
}

/// Body data format for PUT and POST requests
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebhookForm {
    /// Url to sent POST request upon event
    pub url: String,
    /// String that represents an event, possibilities defined in [`WebhookEvent`]
    pub event: WebhookEvent,
}

//TODO: Change behavior to only update rows
/// Create a new webhook configuration within the database
///
/// # Body
///
/// Body data should match [`WebhookForm`]
#[post("/", format = "json", data = "<data>")]
pub async fn create(
    data: Json<WebhookForm>,
    db: &State<Database>,
) -> Result<Json<Vec<Webhook>>, Status> {
    let connection = get_connection(db);
    let new_config = Webhook {
        id: uuid::Uuid::new_v4(),
        url: data.url.clone(),
        last_sent: None,
        event: data.event.to_string(),
    };

    get_json_vec::<Webhook>(
        diesel::insert_into(table)
            .values(&new_config)
            .get_results::<Webhook>(&connection),
        None,
    )
}

/// Modifies a webhook configuration within the database
///
/// # Body
///
/// Body data should match [`WebhookForm`]
#[put("/<id>", format = "json", data = "<data>")]
pub async fn modify(
    id: &str,
    data: Json<WebhookForm>,
    db: &State<Database>,
) -> Result<Json<Vec<Webhook>>, Status> {
    let connection = get_connection(db);
    match uuid::Uuid::parse_str(id) {
        Ok(id) => {
            let new_config = Webhook {
                id,
                url: data.url.clone(),
                last_sent: None,
                event: data.event.to_string(),
            };
            let target = table.find(id);
            get_json_vec(
                diesel::update(target)
                    .set(&new_config)
                    .get_results::<Webhook>(&connection),
                None,
            )
        }
        Err(_) => {
            log!(Level::Error, "Unable to parse UUID!");
            Err(Status::BadRequest)
        }
    }
}
