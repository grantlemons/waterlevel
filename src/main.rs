#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod routes {
    // pub mod analytics;
    pub mod config;
    pub mod waterlevel;
    pub mod webhooks;
}
pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/health", routes![health])
        // .mount("/api/v1/analytics", routes![routes::analytics::get_default])
        .mount(
            "/api/v1/config",
            routes![
                routes::config::get_all,
                routes::config::get_value,
                routes::config::create,
                routes::config::modify
            ],
        )
        .mount(
            "/api/v1/waterlevel",
            routes![
                routes::waterlevel::get_all,
                routes::waterlevel::get_on_date,
                routes::waterlevel::get_at_level,
                routes::waterlevel::get_above_level,
                routes::waterlevel::get_below_level
            ],
        )
        .mount(
            "/api/v1/webhooks",
            routes![
                routes::webhooks::get,
                routes::webhooks::create,
                routes::webhooks::modify
            ],
        )
}

pub fn establish_connection() -> diesel::pg::PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::pg::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all<Table, Model>(
    table: Table,
) -> Result<rocket::serde::json::Json<Vec<Model>>, rocket::http::Status>
where
    Table: diesel::query_dsl::LoadQuery<diesel::pg::PgConnection, Model>,
{
    let connection = crate::establish_connection();
    match table.load::<Model>(&connection) {
        Ok(v) => Ok(rocket::serde::json::Json(v)),
        Err(_) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "Unable to get records!"
            );
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

pub fn get_by_id<Table, Model, PK>(
    table: Table,
    id: PK,
) -> Result<rocket::serde::json::Json<Vec<Model>>, rocket::http::Status>
where
    Table: diesel::query_dsl::methods::FindDsl<PK>,
    Table::Output: diesel::query_dsl::LoadQuery<diesel::pg::PgConnection, Model>,
{
    let connection = crate::establish_connection();
    match table.find(id).load::<Model>(&connection) {
        Ok(v) => Ok(rocket::serde::json::Json(v)),
        Err(_) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "Unable to get record!"
            );
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/")]
fn health() -> &'static str {
    "Healthy!"
}
