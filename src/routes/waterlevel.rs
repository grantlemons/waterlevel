use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json};

use crate::diesel::prelude::*;
use crate::models::{WaterLevel, Weather};
use crate::schema::water_levels::{dsl, table};

use crate::lib::*;

#[get("/")]
pub fn get_all() -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = establish_connection();
    get_json_vec(table.load::<WaterLevel>(&connection), None)
}

/// Gets all data recorded on a certain date
#[get("/date/<date>")]
pub fn get_on_date(date: &str) -> Result<Json<Vec<WaterLevel>>, Status> {
    // Gives different responses depending on the validity of the passed date
    match chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => {
            log!(Level::Info, "inputted date is {}", &date);

            let connection = establish_connection();

            //TODO: Restrict to entries that match day
            let res = table.load::<WaterLevel>(&connection);
            get_json_vec(res, None)
        }
        Err(e) => {
            rocket::log::private::log!(
                rocket::log::private::Level::Error,
                "date from parameter failed: {:?}",
                e
            );
            Err(rocket::http::Status::BadRequest)
        }
    }
}

#[get("/level/at/<level>")]
pub fn get_at_level(level: f32) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = establish_connection();
    get_json_vec(
        table
            .filter(dsl::level.eq(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

#[get("/level/above/<level>")]
pub fn get_above_level(level: f32) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = establish_connection();
    get_json_vec(
        table
            .filter(dsl::level.gt(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

#[get("/level/below/<level>")]
pub fn get_below_level(level: f32) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = establish_connection();
    get_json_vec(
        table
            .filter(dsl::level.lt(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

#[derive(serde::Deserialize)]
pub struct Input {
    location: (f64, f64),
    level: f64,
}

#[post("/", format = "json", data = "<data>")]
pub async fn add_waterlevel(data: Json<Input>) -> Result<Json<WaterLevel>, Status> {
    let connection = establish_connection();

    let weather = get_weather(data.location.0, data.location.1)
        .await
        .expect("Unable to get weather data from third-party API");

    let new_waterlevel = WaterLevel {
        id: uuid::Uuid::new_v4(),
        location: diesel_geometry::data_types::PgPoint(data.location.0, data.location.1),
        timestamp: chrono::Utc::now().naive_utc(),
        weather_id: Some(weather.id),
        level: data.level,
    };
    get_json::<WaterLevel>(
        diesel::insert_into(table)
            .values(&new_waterlevel)
            .get_results::<WaterLevel>(&connection),
        None,
    )
}

#[derive(Debug, serde::Deserialize)]
struct Response {
    main: Data,
    dt: i64,
}

#[derive(Debug, serde::Deserialize)]
struct Data {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64,
    humidity: i16,
}

struct WeatherData {
    id: i64,
    main: String,
}

async fn get_weather(lat: f64, lon: f64) -> Result<Weather, reqwest::Error> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&units=imperial&appid={key}",
        lat = lat,
        lon = lon,
        key = dotenv_codegen::dotenv!("WEATHER_API_KEY")
    );
    let response = reqwest::get(&url).await?;
    let json: Response = response.json().await?;
    let data: Data = json.main;
    let weather_data: WeatherData = json.weather_data;
    Ok(
        Weather {
            id: uuid::Uuid::new_v4(),
            location: diesel_geometry::data_types::PgPoint(lat, lon),
            timestamp: chrono::NaiveDateTime::from_timestamp(json.dt, 0),
            temp: data.temp,
            temp_min: data.temp_min,
            temp_max: data.temp_max,
            pressure: data.pressure,
            humidity: data.humidity,
            weather_id: weather_data.id,
            weather_name: weather_data.main,
        }
    )
}
