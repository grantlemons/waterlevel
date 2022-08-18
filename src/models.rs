use diesel::pg::types::sql_types::*

#[derive(Queryable)]
pub struct Weather {
    pub id: sql_types::Uuid,
    pub location: sql_types::Point,
    pub timestamp: sql_types::Date,
}

#[derive(Queryable)]
pub struct WaterLevel {
    pub id: sql_types::Uuid,
    pub location: sql_types::Point,
    pub timestamp: sql_types::Date,
    pub weather_id: sql_types::Uuid,
    pub level: i8,
}

#[derive(Queryable)]
pub struct Config {
    pub key: str,
    pub value: str,
}

#[derive(Queryable)]
pub struct Webhook {
    pub id: sql_types::Uuid,
    pub url: str,
    pub last_sent: sql_types::Date,
    pub event: str,
}
