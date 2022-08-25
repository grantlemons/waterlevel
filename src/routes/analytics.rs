#![allow(unused)]
use rocket::{http::Status, serde::json::Json, State};
use crate::helpers::*;

use crate::diesel::prelude::*;
use crate::models::Config;
use crate::models::WaterLevel;
use crate::schema::config::table as c_table;
use crate::schema::water_levels::table as wl_table;

#[get("/")]
pub fn get_default(db: &State<Database>) {}
