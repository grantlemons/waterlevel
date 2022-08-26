use diesel::r2d2::PooledConnection;
use dotenv::dotenv;
use std::env;

pub struct Database(pub ConnPool);

pub type ConnPool = diesel::r2d2::Pool<PgConnManager>;
pub type PgConnManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;

pub fn get_connection(db: &Database) -> PooledConnection<PgConnManager> {
    db.0.get().expect("Unable to get connection from pool")
}

pub fn get_pool() -> ConnPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: PgConnManager = diesel::r2d2::ConnectionManager::new(database_url);
    diesel::r2d2::Pool::new(manager).expect("Unable to create connection pool")
}

pub fn get_json_vec<Model>(
    res: Result<Vec<Model>, diesel::result::Error>,
    log: Option<&'static str>,
) -> Result<rocket::serde::json::Json<Vec<Model>>, rocket::http::Status> {
    match res {
        Ok(v) => Ok(rocket::serde::json::Json(v)),
        Err(_) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "{}",
                log.unwrap_or("Unable to get/insert records!")
            );
            Err(rocket::http::Status::InternalServerError)
        }
    }
}
