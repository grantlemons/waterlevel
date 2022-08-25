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

#[derive(Serialize, Insertable, Queryable, Identifiable, Debug)]
#[primary_key(key)]
#[table_name = "config"]
pub struct Config {
    pub key: String,
    pub value: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Serialize, Insertable, Queryable, Identifiable, Debug)]
#[table_name = "webhooks"]
pub struct Webhook {
    pub id: Uuid,
    pub url: String,
    pub last_sent: Option<NaiveDateTime>,
    pub event: String,
}
