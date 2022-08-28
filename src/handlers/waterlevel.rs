//! Handlers for getting and creating waterlevel entries in the database

use chrono::NaiveDate;
use rocket::{http::Status, log::private::log, log::private::Level, serde::json::Json, State};
use std::env;

use crate::diesel::prelude::*;
use crate::models::{WaterLevel, Weather};
use crate::schema::water_levels::{dsl, table};

use crate::helpers::*;

/// Get all waterlevel entries from database
#[get("/")]
pub async fn get_all(db: &State<Database>) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = get_connection(db);
    get_json_vec(table.load::<WaterLevel>(&connection), None)
}

/// Get all waterlevel entries recorded on a certain date
///
/// # Parameters
///
/// `/date/<date>` date in ISO 8601 format (YYYY-MM-DD)
#[get("/date/<date>")]
pub async fn get_on_date(
    date: &str,
    db: &State<Database>,
) -> Result<Json<Vec<WaterLevel>>, Status> {
    // Gives different responses depending on the validity of the passed date
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => {
            log!(Level::Info, "inputted date is {}", &date);

            let connection = get_connection(db);

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

/// Get all waterlevel entries that have a water level that matches the input parameter
///
/// # Parameters
///
/// `/level/at/<level>` water level that entries should match
#[get("/level/at/<level>")]
pub async fn get_at_level(
    level: f32,
    db: &State<Database>,
) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = get_connection(db);
    get_json_vec(
        table
            .filter(dsl::level.eq(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

/// Get all waterlevel entries that have a water level that is above the input parameter
///
/// # Parameters
///
/// `/level/above/<level>` water level that entries should exceed
#[get("/level/above/<level>")]
pub async fn get_above_level(
    level: f32,
    db: &State<Database>,
) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = get_connection(db);
    get_json_vec(
        table
            .filter(dsl::level.gt(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

/// Get all waterlevel entries that have a water level that is below the input parameter
///
/// # Parameters
///
/// `/level/below/<level>` water level that entries should be less than
#[get("/level/below/<level>")]
pub async fn get_below_level(
    level: f32,
    db: &State<Database>,
) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = get_connection(db);
    get_json_vec(
        table
            .filter(dsl::level.lt(level as f64))
            .load::<WaterLevel>(&connection),
        None,
    )
}

/// Body of post request sent to / endpoint
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WaterlevelForm {
    pub location: (f64, f64),
    pub level: f64,
}

/// Adds new level and weather entry for the location given
/// Used only by the IOT to record data into the API
///
/// # Body
///
/// Body data should match [`WaterlevelForm`]
#[post("/", format = "json", data = "<data>")]
pub async fn add_waterlevel(
    data: Json<WaterlevelForm>,
    db: &State<Database>,
) -> Result<Json<Vec<WaterLevel>>, Status> {
    let connection = get_connection(db);

    // Get weather data
    let weather = get_weather(data.location.0, data.location.1)
        .await
        .expect("Unable to get weather data from third-party API");

    // Insert weather data
    if let Err(e) = add_weather(&weather, db).await {
        log!(Level::Error, "Error adding weather entry: {}", e);
    }

    // Waterlevel data in struct
    let new_waterlevel = WaterLevel {
        id: uuid::Uuid::new_v4(),
        location: diesel_geometry::data_types::PgPoint(data.location.0, data.location.1),
        timestamp: chrono::Utc::now().naive_utc(),
        weather_id: Some(weather.id),
        level: data.level,
    };

    // Insert and get waterlevel data
    get_json_vec::<WaterLevel>(
        diesel::insert_into(table)
            .values(&new_waterlevel)
            .get_results::<WaterLevel>(&connection),
        None,
    )
}

/// Internal function used to insert weather entry into database
/// Called by [`add_waterlevel`]
async fn add_weather(
    weather_data: &Weather,
    db: &State<Database>,
) -> Result<Json<Vec<Weather>>, Status> {
    let connection = get_connection(db);
    let weather_table = crate::schema::weather::table;

    get_json_vec::<Weather>(
        diesel::insert_into(weather_table)
            .values(weather_data)
            .get_results::<Weather>(&connection),
        None,
    )
}

/// Response fields retrieved from the OpenWeather API
#[derive(Debug, serde::Deserialize)]
struct WeatherResponse {
    weather: Vec<WeatherData>,
    main: TemperatureData,
    dt: i64,
}

/// Temperature data from the OpenWeather API
/// Made to nest within [`WeatherResponse`] struct
#[derive(Debug, serde::Deserialize)]
struct TemperatureData {
    temp: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64,
    humidity: i16,
}

/// Weather data retrieved from the OpenWeather API
/// Made to nest within [`WeatherResponse`] struct
#[derive(Debug, serde::Deserialize, Clone)]
struct WeatherData {
    id: i16,
    main: String,
}

/// Gets weather from OpenWeather api for the passed location
/// Called by [`add_waterlevel`] in order to get weather data at the location of the arduino measuring device
pub async fn get_weather(lat: f64, lon: f64) -> Result<Weather, reqwest::Error> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&units=imperial&appid={key}",
        lat = lat,
        lon = lon,
        key = env::var("WEATHER_API_KEY")
            .expect("Unable to get weather api key environment variable")
    );
    let response = reqwest::get(&url).await?;
    let json: WeatherResponse = response.json().await?;
    let data: TemperatureData = json.main;
    let weather_data: &WeatherData = &json.weather[0];
    Ok(Weather {
        id: uuid::Uuid::new_v4(),
        location: diesel_geometry::data_types::PgPoint(lat, lon),
        timestamp: chrono::NaiveDateTime::from_timestamp(json.dt, 0),
        temp: data.temp,
        temp_min: data.temp_min,
        temp_max: data.temp_max,
        pressure: data.pressure,
        humidity: data.humidity,
        weather_id: weather_data.id,
        weather_name: weather_data.main.clone(),
    })
}
