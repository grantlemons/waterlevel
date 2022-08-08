#[macro_use]
extern crate rocket;

mod routes {
    pub mod hello;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![health])
        .mount("/api/v1/hello", routes![routes::hello::world])
}

#[get("/")]
fn health() {}
