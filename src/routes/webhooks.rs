use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::Webhook;
use crate::schema::webhooks::table;

use crate::lib::*;

#[get("/")]
pub fn get_all() -> Result<Json<Vec<Webhook>>, Status> {
    let connection = establish_connection();
    get_json_vec(table.load::<Webhook>(&connection), None)
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
    let connection = establish_connection();
    let new_config = Webhook {
        id: uuid::Uuid::new_v4(),
        url: data.url.clone(),
        last_sent: None,
        event: data.event.clone(),
    };

    get_json::<Webhook>(
        diesel::insert_into(table)
            .values(&new_config)
            .get_results::<Webhook>(&connection),
        None,
    )
}

#[put("/<id>", format = "json", data = "<data>")]
pub fn modify(id: &str, data: Json<Input>) -> Result<Json<Webhook>, Status> {
    let connection = establish_connection();
    match uuid::Uuid::parse_str(id) {
        Ok(id) => get_json::<Webhook>(
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
