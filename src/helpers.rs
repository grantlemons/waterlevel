use diesel::r2d2::PooledConnection;
use dotenv::dotenv;
use std::env;

/// Possible events that call specific webhooks configured to listen for them
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WebhookEvent {
    All,
}

impl std::fmt::Display for WebhookEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WebhookEvent::All => write!(f, "All"),
        }
    }
}

#[derive(serde::Serialize)]
struct WebhookBody {
    event: String,
}

/// Send POST requests to all relevent urls based on webhook config
pub async fn trigger_webhooks(event: WebhookEvent) {
    use crate::diesel::prelude::*;
    use crate::models::Webhook;
    use crate::schema::webhooks::{dsl, table};

    let connection = get_pool()
        .get()
        .expect("Unable to get connection from pool");
    let client = reqwest::Client::new();
    let urls = table
        .filter(dsl::event.eq(event.to_string()))
        .load::<Webhook>(&connection)
        .expect("Unable to get records");
    for i in urls {
        if client
            .post(&i.url)
            .json(&WebhookBody {
                event: event.to_string(),
            })
            .send()
            .await
            .is_ok()
        {
            rocket::log::private::log!(
                rocket::log::private::Level::Debug,
                "Sent POST request to {}",
                i.url,
            );
        };
    }
}

/// Shared state struct for connection pool
pub struct Database(pub ConnPool);

pub type ConnPool = diesel::r2d2::Pool<PgConnManager>;
pub type PgConnManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;

/// Get a connection from the shared state
pub fn get_connection(db: &Database) -> PooledConnection<PgConnManager> {
    db.0.get().expect("Unable to get connection from pool")
}

/// Get a connection pool
pub fn get_pool() -> ConnPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: PgConnManager = diesel::r2d2::ConnectionManager::new(database_url);
    diesel::r2d2::Pool::new(manager).expect("Unable to create connection pool")
}

/// Serialize the output of a diesel request and handle errors
pub fn get_json_vec<Model>(
    res: Result<Vec<Model>, diesel::result::Error>,
    log: Option<&'static str>,
) -> Result<rocket::serde::json::Json<Vec<Model>>, rocket::http::Status> {
    match res {
        Ok(v) => Ok(rocket::serde::json::Json(v)),
        Err(e) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "Error when getting from DB: {}",
                e
            );
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "{}",
                log.unwrap_or("Unable to get/insert records!")
            );
            Err(rocket::http::Status::InternalServerError)
        }
    }
}
