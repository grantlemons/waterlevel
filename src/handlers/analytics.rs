#![allow(unused, clippy::let_unit_value)] // remove after implementing functions
use crate::helpers::*;
use rocket::{http::Status, serde::json::Json, State};

use crate::diesel::prelude::*;
use crate::models::Config;
use crate::models::WaterLevel;
use crate::schema::config::table as c_table;
use crate::schema::water_levels::table as wl_table;

#[get("/")]
pub async fn get_default(db: &State<Database>) {}
