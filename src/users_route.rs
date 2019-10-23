use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::schema;
use crate::users_model::{InsertableUser, UpdatableUser, Users};
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

#[get("/users/<id>")]
pub fn read_user(id: i32, conn: DbConn) -> Result<Json<Vec<Users>>, String> {
    schema::users::table
        .find(id)
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying user: {:?}", err);
            "Error querying user from the database".into()
        })
        .map(Json)
}

#[put("/users/<id>", data = "<user>")]
pub fn update_user(id: i32, conn: DbConn, user: Json<UpdatableUser>) -> Result<String, String> {
    let inserted_rows = diesel::update(schema::users::table.find(id))
        .set(&user.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error updating row: {:?}", err);
            "Error updating row into database".into()
        })?;

    Ok(format!("Updated {} row(s).", inserted_rows))
}

#[delete("/users/<id>")]
pub fn delete_user(id: i32, conn: DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(schema::users::table.find(id))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting row: {:?}", err);
            "Error deleting row into database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
