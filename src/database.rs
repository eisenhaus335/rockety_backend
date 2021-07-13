use diesel::{QueryDsl, RunQueryDsl};
use rocket::{Error, fairing::AdHoc};
use rocket::{Rocket, Build};
use rocket::response::{Debug, status::Created};
use rocket_sync_db_pools::{database, diesel};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[database("sqlite_logs")] 
pub struct Db(diesel::SqliteConnection);


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Sqlite3 Service", |rocket| async {
        rocket.attach(Db::fairing())
    })
}
