use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::schema;
use crate::users_model::{InsertableUser, Users};
use crate::DbConn;

#[post("/users", data = "<user>")]
pub fn create_user(conn: DbConn, user: Json<InsertableUser>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::users::table)
        .values(&user.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/users")]
pub fn read_users(conn: DbConn) -> Result<Json<Vec<Users>>, String> {
    use crate::schema::users::dsl::*;

    users
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(Json)
}
