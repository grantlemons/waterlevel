#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

pub mod routes {
    pub mod analytics;
    pub mod config;
    pub mod waterlevel;
    pub mod webhooks;
}
pub mod helpers;
pub mod models;
pub mod schema;

use helpers::{get_connection, get_pool, Database};

#[launch]
pub fn entrypoint() -> _ {
    // Create database struct
    let database = Database(get_pool());
    let conn = get_connection(&database);

    // Run database migrations
    embed_migrations!();
    if let Ok(_) = embedded_migrations::run(&conn) {
        rocket::log::private::log!(rocket::log::private::Level::Info, "Ran migrations");
    };
    // Create rocket routes
    rocket::build()
        .manage(database)
        .mount("/api/v1/health", routes![health])
        .mount("/api/v1/analytics", routes![routes::analytics::get_default])
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
                routes::webhooks::get_all,
                routes::webhooks::create,
                routes::webhooks::modify
            ],
        )
}

#[get("/")]
fn health(db: &rocket::State<Database>) -> &'static str {
    get_connection(&db); // check connection to db
    "Healthy!"
}
