use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::professors_model::{InsertableProfessor, Professors, UpdatableProfessor};
use crate::schema;
use crate::DbConn;

#[post("/professors", data = "<professor>")]
pub fn create_professor(
    conn: DbConn,
    professor: Json<InsertableProfessor>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::professors::table)
        .values(&professor.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/professors")]
pub fn read_professors(conn: DbConn) -> Result<Json<Vec<Professors>>, String> {
    use crate::schema::professors::dsl::*;

    professors
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(Json)
}

#[get("/professors/<id>")]
pub fn read_professor(id: i32, conn: DbConn) -> Result<Json<Vec<Professors>>, String> {
    schema::professors::table
        .find(id)
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying professor: {:?}", err);
            "Error querying professor from the database".into()
        })
        .map(Json)
}

#[put("/professors/<id>", data = "<professor>")]
pub fn update_professor(
    id: i32,
    conn: DbConn,
    professor: Json<UpdatableProfessor>,
) -> Result<String, String> {
    let inserted_rows = diesel::update(schema::professors::table.find(id))
        .set(&professor.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error updating row: {:?}", err);
            "Error updating row into database".into()
        })?;

    Ok(format!("Updated {} row(s).", inserted_rows))
}

#[delete("/professors/<id>")]
pub fn delete_professor(id: i32, conn: DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(schema::professors::table.find(id))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting row: {:?}", err);
            "Error deleting row into database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
