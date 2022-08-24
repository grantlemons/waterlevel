use diesel_migrations::embed_migrations;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

mod routes {
    pub mod analytics;
    pub mod config;
    pub mod waterlevel;
    pub mod webhooks;
}

pub mod lib;
pub mod models;
pub mod schema;

#[cfg(test)]
mod tests {
    mod database_connection;
    mod endpoint_tests {
        pub mod analytics;
        pub mod config;
        pub mod waterlevel;
        pub mod webhooks;
    }
}

#[launch]
fn rocket() -> _ {
    // Run database migrations
    embed_migrations!();
    embedded_migrations::run(&lib::establish_connection())
        .expect("Unable to run migrations");
    
    // Create rocket routes
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
fn health() -> &'static str {
    lib::establish_connection(); // check connection to db
    "Healthy!"
}
