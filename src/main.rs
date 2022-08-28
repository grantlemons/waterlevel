//! API used to insert and get waterlevel values into/from a postgres database as well as provide analytics and additional info based on an external weather API
//! Provides webhook support with the [`webhooks`](crate::routes::waterlevel) endpoint
//! Built with the [`rocket`] framework and [`diesel`] ORM

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

/// Route specific handlers
pub mod handlers {
    /// Handlers for waterlevel analytics
    pub mod analytics;
    /// Configuration Handlers
    pub mod config;
    /// Handlers for getting and inserting water levels into the database
    pub mod waterlevel;
    /// Handlers for webhook setup
    pub mod webhooks;
}
/// Extra functions used in handlers and main function
pub mod helpers;
/// Database model structs to change from SQL types in [`schema`](crate::schema) to rust types
pub mod models;
/// Database schema file used by [`diesel`]
pub mod schema;

use helpers::{get_connection, get_pool, Database};

/// API routing root as constant for easy configuration
pub const ROOT: &str = "/api/v1/";

/// Runs migration files with [`diesel_migrations`]
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
/// Rocket entrypoint
/// Configures routes and shared state and runs migrations on start
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
            routes![handlers::analytics::get_default],
        )
        .mount(
            ROOT.to_owned() + "config",
            routes![
                handlers::config::get_all,
                handlers::config::get_value,
                handlers::config::create,
                handlers::config::modify
            ],
        )
        .mount(
            ROOT.to_owned() + "waterlevel",
            routes![
                handlers::waterlevel::get_all,
                handlers::waterlevel::get_on_date,
                handlers::waterlevel::get_at_level,
                handlers::waterlevel::get_above_level,
                handlers::waterlevel::get_below_level,
                handlers::waterlevel::add_waterlevel,
            ],
        )
        .mount(
            ROOT.to_owned() + "webhooks",
            routes![
                handlers::webhooks::get_all,
                handlers::webhooks::create,
                handlers::webhooks::modify
            ],
        )
}

#[get("/")]
/// Healthcheck handler
/// Checks database connection and and returns `Healthy!`
fn health(db: &rocket::State<Database>) -> &'static str {
    get_connection(db); // check connection to db
    "Healthy!"
}
