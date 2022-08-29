// use waterlevel_backend::routes::analytics;

#[ctor::ctor]
fn setup() {
    waterlevel_backend::run_migrations(None);
}
