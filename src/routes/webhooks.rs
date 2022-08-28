use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json, State};

use crate::diesel::prelude::*;
use crate::models::Webhook;
use crate::schema::webhooks::table;

use crate::helpers::*;

#[get("/")]
pub async fn get_all(db: &State<Database>) -> Result<Json<Vec<Webhook>>, Status> {
    let connection = get_connection(db);
    get_json_vec(table.load::<Webhook>(&connection), None)
}

// Struct for input to functions that need it
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Input {
    pub url: String,
    pub event: String,
}

//TODO: Change behavior to only update rows
#[post("/", format = "json", data = "<data>")]
pub async fn create(data: Json<Input>, db: &State<Database>) -> Result<Json<Vec<Webhook>>, Status> {
    let connection = get_connection(db);
    let new_config = Webhook {
        id: uuid::Uuid::new_v4(),
        url: data.url.clone(),
        last_sent: None,
        event: data.event.clone(),
    };

    get_json_vec::<Webhook>(
        diesel::insert_into(table)
            .values(&new_config)
            .get_results::<Webhook>(&connection),
        None,
    )
}

#[put("/<id>", format = "json", data = "<data>")]
pub async fn modify(
    id: &str,
    data: Json<Input>,
    db: &State<Database>,
) -> Result<Json<Vec<Webhook>>, Status> {
    let connection = get_connection(db);
    match uuid::Uuid::parse_str(id) {
        Ok(id) => {
            let new_config = Webhook {
                id,
                url: data.url.clone(),
                last_sent: None,
                event: data.event.clone(),
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
