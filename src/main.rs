#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv_codegen;

mod routes {
    pub mod analytics;
    pub mod config;
    pub mod waterlevel;
    pub mod webhooks;
}
pub mod lib;
pub mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
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
                routes::waterlevel::get_below_level,
                routes::waterlevel::add_waterlevel,
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
fn health() -> &'static str {
    lib::establish_connection(); // check connection to db
    "Healthy!"
}
