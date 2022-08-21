use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> diesel::pg::PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match diesel::pg::PgConnection::establish(&database_url) {
        Ok(v) => v,
        Err(_) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "Unable to connect to Database!"
            );
            panic!("Unable to connect to Database!");
        }
    }
}

pub fn get_json<Model>(
    res: Result<Vec<Model>, diesel::result::Error>,
    log: Option<&'static str>,
) -> Result<rocket::serde::json::Json<Model>, rocket::http::Status> {
    match res {
        Ok(mut v) => Ok(rocket::serde::json::Json(v.remove(0))),
        Err(_) => {
            let s = match log {
                Some(s) => s,
                None => "Unable to get/insert records!",
            };
            rocket::log::private::log!(rocket::log::private::Level::Error, "{}", s);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

pub fn get_json_vec<Model>(
    res: Result<Vec<Model>, diesel::result::Error>,
    log: Option<&'static str>,
) -> Result<rocket::serde::json::Json<Vec<Model>>, rocket::http::Status> {
    match res {
        Ok(v) => Ok(rocket::serde::json::Json(v)),
        Err(_) => {
            let s = match log {
                Some(s) => s,
                None => "Unable to get/insert records!",
            };
            rocket::log::private::log!(rocket::log::private::Level::Error, "{}", s);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}
