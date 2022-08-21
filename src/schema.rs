table! {
    config (key) {
        key -> Bpchar,
        value -> Bpchar,
        timestamp -> Timestamp,
    }
}

table! {
    water_levels (id) {
        id -> Uuid,
        location -> diesel_geometry::sql_types::Point,
        timestamp -> Timestamp,
        weather_id -> Nullable<Uuid>,
        level -> Float8,
    }
}

table! {
    weather (id) {
        id -> Uuid,
        location -> diesel_geometry::sql_types::Point,
        timestamp -> Timestamp,
    }
}

table! {
    webhooks (id) {
        id -> Uuid,
        url -> Bpchar,
        last_sent -> Nullable<Timestamp>,
        event -> Bpchar,
    }
}

allow_tables_to_appear_in_same_query!(config, water_levels, weather, webhooks,);
