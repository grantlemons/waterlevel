use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::Webhook;
use crate::schema::webhooks::columns;
use crate::schema::webhooks::table;

#[get("/")]
pub fn get() {}

#[derive(serde::Deserialize)]
pub struct Input {
    url: String,
    event: String,
}

#[put("/", format = "json", data = "<data>")]
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

#[put("/<id>", format = "json", data = "<data>")]
pub fn modify(id: &str, data: Json<Input>) -> Result<Json<Vec<Webhook>>, Status> {
    let connection = crate::establish_connection();

    match uuid::Uuid::parse_str(id) {
        Ok(id) => {
            let vec_val = diesel::insert_into(table)
                .values(
                    Webhook {
                        id,
                        url: data.url.clone(),
                        last_sent: None,
                        event: data.event.clone(),
                    })
                .get_results::<Webhook>(&connection);
            match vec_val {
                Ok(vec) => Ok(Json(vec)),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => {
            Err(Status::BadRequest)
        }
    }
}
