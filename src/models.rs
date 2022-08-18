use diesel_geometry::pg::sql_types::PgPoint;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Weather {
    pub id: Uuid,
    pub location: PgPoint,
    pub timestamp: NaiveDateTime,
}

#[derive(Queryable)]
pub struct WaterLevel {
    pub id: Uuid,
    pub location: PgPoint,
    pub timestamp: NaiveDateTime,
    pub weather_id: Uuid,
    pub level: i8,
}

#[derive(Queryable)]
pub struct Config {
    pub key: String,
    pub value: String,
}

#[derive(Queryable)]
pub struct Webhook {
    pub id: Uuid,
    pub url: String,
    pub last_sent: NaiveDateTime,
    pub event: String,
}
