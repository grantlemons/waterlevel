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

pub const ROOT: &str = "/api/v1/";

pub fn run_migrations(database: Option<&Database>) {
    let conn = match database {
        Some(v) => get_connection(v),
        None => get_connection(&Database(get_pool())),
    };
    embed_migrations!();
    if embedded_migrations::run(&conn).is_ok() {
        rocket::log::private::log!(rocket::log::private::Level::Info, "Ran migrations");
    };
}

#[launch]
pub fn entrypoint() -> _ {
    // Create database struct
    let database = Database(get_pool());

    // Run database migrations
    run_migrations(Some(&database));
    
    // Create rocket routes
    rocket::build()
        .manage(database)
        .mount(ROOT.to_owned() + "health", routes![health])
        .mount(
            ROOT.to_owned() + "analytics",
            routes![routes::analytics::get_default],
        )
        .mount(
            ROOT.to_owned() + "config",
            routes![
                routes::config::get_all,
                routes::config::get_value,
                routes::config::create,
                routes::config::modify
            ],
        )
        .mount(
            ROOT.to_owned() + "waterlevel",
            routes![
                routes::waterlevel::get_all,
                routes::waterlevel::get_on_date,
                routes::waterlevel::get_at_level,
                routes::waterlevel::get_above_level,
                routes::waterlevel::get_below_level,
                routes::waterlevel::add_waterlevel,
            ],
        )
        .mount(
            ROOT.to_owned() + "webhooks",
            routes![
                routes::webhooks::get_all,
                routes::webhooks::create,
                routes::webhooks::modify
            ],
        )
}

#[get("/")]
fn health(db: &rocket::State<Database>) -> &'static str {
    get_connection(db); // check connection to db
    "Healthy!"
}
