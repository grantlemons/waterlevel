#[get("/")]
pub fn get() {}

#[post("/")]
pub fn create() {}

#[put("/<id>")]
pub fn modify(id: i16) {}
