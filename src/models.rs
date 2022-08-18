use diesel_geometry::sql_types::Point;

#[derive(Queryable)]
pub struct Weather {
    pub id: Uuid,
    pub location: Point,
    pub timestamp: Timestamp,
}

#[derive(Queryable)]
pub struct WaterLevel {
    pub id: Uuid,
    pub location: Point,
    pub timestamp: Timestamp,
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
    pub last_sent: Timestamp,
    pub event: String,
}
