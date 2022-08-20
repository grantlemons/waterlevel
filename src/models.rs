use diesel::*;

use diesel_geometry::pg::data_types::PgPoint;
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::schema::*;
use serde::{Serialize};

#[derive(Serialize, Insertable, Queryable, AsChangeset, Associations, Identifiable, Debug)]
#[table_name="weather"]
pub struct Weather {
    pub id: Uuid,
    pub location: PgPoint,
    pub timestamp: NaiveDateTime,
}

#[derive(Serialize, Insertable, Queryable, AsChangeset, Associations, Identifiable, Debug)]
#[belongs_to(Weather)]
#[table_name="water_levels"]
pub struct WaterLevel {
    pub id: Uuid,
    pub location: PgPoint,
    pub timestamp: NaiveDateTime,
    pub weather_id: Option<Uuid>,
    pub level: f64,
}

#[derive(Serialize, Insertable, Queryable, AsChangeset, Identifiable, Debug)]
#[primary_key(key)]
#[table_name="config"]
pub struct Config {
    pub key: String,
    pub value: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Serialize, Insertable, Queryable, AsChangeset, Identifiable, Debug)]
#[table_name="webhooks"]
pub struct Webhook {
    pub id: Uuid,
    pub url: String,
    pub last_sent: Option<NaiveDateTime>,
    pub event: String,
}
