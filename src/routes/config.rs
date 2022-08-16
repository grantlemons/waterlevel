#[get("/")]
pub fn get_all() {}

#[get("/<key>")]
pub fn get_value(key: &str) {}

#[post("/")]
pub fn create() {}

#[put("/<key>")]
pub fn modify(key: &str) {}
