use rocket::fairing::AdHoc;
use rocket::figment::providers::Serialized;
use rocket::request::FromRequest;
use rocket::response::{Debug, status::{Created}};
use rocket::serde::{Serialize, Deserialize, json::Json };
use rocket::outcome::{Outcome, try_outcome};
use rocket_sync_db_pools::diesel;

use crate::database::Db;
use self ::diesel::prelude::*;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug)]
struct User {
    id: i32,
    username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r User {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let db : Db = try_outcome!(request.guard::<Db>().await);
        let ids = request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse::<i32>().ok())
            .unwrap();
        db.run(move |conn| {
            users::table
                .filter(users::id.eq(ids))
                .first(conn);
        });

        Outcome::Success(&User {
            id: 32,
            username: String::from("t"),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="users"]
struct StoredUser {
    id: Option<i32>,
    username: String,
    password: String,
}

table! {
    users(id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
    }
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Authentication Service", |rocket| async {
        rocket.mount("/users", routes![create])
    })
}