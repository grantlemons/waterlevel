use diesel::*;

use crate::schema::*;
use chrono::NaiveDateTime;
use diesel_geometry::pg::data_types::PgPoint;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Insertable, Queryable, Associations, Identifiable, Debug)]
#[table_name = "weather"]
pub struct Weather {
    pub id: Uuid,
    pub location: PgPoint,
    pub timestamp: NaiveDateTime,
    pub temp: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: f64,
    pub humidity: i16,
    pub weather_id: i16,
    pub weather_name: String,
}

#[derive(Serialize, Insertable, Queryable, Associations, Identifiable, Debug)]
#[belongs_to(Weather)]
#[table_name = "water_levels"]
pub struct WaterLevel {
    pub id: Uuid,
    pub location: PgPoint,
    pub timestamp: NaiveDateTime,
    pub weather_id: Option<Uuid>,
    pub level: f64,
}

#[derive(Serialize, Insertable, Queryable, AsChangeset, Identifiable, Debug)]
#[primary_key(key)]
#[table_name = "config"]
pub struct Config {
    pub key: String,
    pub value: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Serialize, Insertable, Queryable, AsChangeset, Identifiable, Debug)]
#[table_name = "webhooks"]
pub struct Webhook {
    pub id: Uuid,
    pub url: String,
    pub last_sent: Option<NaiveDateTime>,
    pub event: String,
}
