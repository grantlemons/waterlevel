#[macro_use]
extern crate rocket;

mod routes {
    pub mod analytics;
    pub mod config;
    pub mod waterlevel;
    pub mod webhooks;
}

use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1/health", routes![health])
        .mount("/api/v1/analytics", routes![analytics::get_default])
        .mount(
            "/api/v1/config",
            routes![
                config::get_all,
                config::get_value,
                config::create,
                config::modify
            ],
        )
        .mount(
            "/api/v1/waterlevel",
            routes![
                waterlevel::get_all,
                waterlevel::get_on_date,
                waterlevel::get_at_level,
                waterlevel::get_above_level,
                waterlevel::get_below_level
            ],
        )
        .mount(
            "/api/v1/webhooks",
            routes![webhooks::get, webhooks::create, webhooks::modify],
        )
}

#[get("/")]
fn health() -> &'static str {
    "Healthy!"
}
