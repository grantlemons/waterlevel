use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::Webhook;
use crate::schema::webhooks::table;

#[get("/")]
pub fn get() -> Result<Json<Vec<Webhook>>, Status> {
    crate::lib::get_all::<table, Webhook>(table)
}

// Struct for input to functions that need it
#[derive(serde::Deserialize)]
pub struct Input {
    url: String,
    event: String,
}

//TODO: Change behavior to only update rows
#[put("/", format = "json", data = "<data>")]
pub fn create(data: Json<Input>) -> Result<Json<Webhook>, Status> {
    let connection = crate::lib::establish_connection();
    let new_config = Webhook {
        id: uuid::Uuid::new_v4(),
        url: data.url.clone(),
        last_sent: None,
        event: data.event.clone(),
    };

    crate::lib::get_json::<Webhook>(
        diesel::insert_into(table)
            .values(&new_config)
            .get_results::<Webhook>(&connection),
        None,
    )
}

#[put("/<id>", format = "json", data = "<data>")]
pub fn modify(id: &str, data: Json<Input>) -> Result<Json<Webhook>, Status> {
    let connection = crate::lib::establish_connection();
    match uuid::Uuid::parse_str(id) {
        Ok(id) => crate::lib::get_json::<Webhook>(
            diesel::insert_into(table)
                .values(Webhook {
                    id,
                    url: data.url.clone(),
                    last_sent: None,
                    event: data.event.clone(),
                })
                .get_results::<Webhook>(&connection),
            None,
        ),
        Err(_) => {
            log!(Level::Error, "Unable to parse UUID!");
            Err(Status::BadRequest)
        }
    }
}
