#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;

mod post;
mod database;
mod user;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::stage())
        .attach(user::stage())
}