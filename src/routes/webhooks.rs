use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::models::Webhook;
use crate::diesel::prelude::*;
use crate::schema::webhooks::table;
use crate::schema::webhooks::columns;

#[get("/")]
pub fn get() {}

#[derive(serde::Deserialize)]
pub struct Input {
    url: String,
    event: String,
}

#[put("/", format="json", data="<data>")]
pub fn create(data: Json<Input>) {
    let connection = crate::establish_connection();
    let new_config = Webhook {
        id: uuid::Uuid::new_v4(),
        url: data.url.clone(),
        last_sent: None,
        event: data.event.clone(),
    };

    diesel::insert_into(table)
        .values(&new_config)
        .get_results::<Webhook>(&connection)
        .expect("Error editing config value");
}

#[put("/<id>", format="json", data="<data>")]
pub fn modify(id: &str, data: Json<Input>) -> Result<Vec<Webhook, u8>, uuid::Error> {
    let connection = crate::establish_connection();
    let new_webhook = match uuid::Uuid::parse_str(id) {
        Ok(parsed_uuid) => {
            log!(Level::Info, "UUID: {}", &parsed_uuid);
            Webhook {
                id: parsed_uuid,
                url: data.url.clone(),
                last_sent: None,
                event: data.event.clone(),
            }
        }
        Err(e) => {
            log!(Level::Error, "Parsing UUID Input failed: {}", e);
        }
    };
    diesel::insert_into(table)
        .values(&new_webhook)
        .get_results::<Webhook>(&connection)
        .expect("Unable to update value")
}