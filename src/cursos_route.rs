use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::cursos_model::{Cursos, InsertableCurso, UpdatableCurso};
use crate::schema;
use crate::DbConn;

#[post("/cursos", data = "<curso>")]
pub fn create_curso(conn: DbConn, curso: Json<InsertableCurso>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::cursos::table)
        .values(&curso.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/cursos")]
pub fn read_cursos(conn: DbConn) -> Result<Json<Vec<Cursos>>, String> {
    use crate::schema::cursos::dsl::*;

    cursos
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(Json)
}

#[get("/cursos/<id>")]
pub fn read_curso(id: i32, conn: DbConn) -> Result<Json<Vec<Cursos>>, String> {
    schema::cursos::table
        .find(id)
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying curso: {:?}", err);
            "Error querying curso from the database".into()
        })
        .map(Json)
}

#[put("/cursos/<id>", data = "<curso>")]
pub fn update_curso(id: i32, conn: DbConn, curso: Json<UpdatableCurso>) -> Result<String, String> {
    let inserted_rows = diesel::update(schema::cursos::table.find(id))
        .set(&curso.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error updating row: {:?}", err);
            "Error updating row into database".into()
        })?;

    Ok(format!("Updated {} row(s).", inserted_rows))
}

#[delete("/cursos/<id>")]
pub fn delete_curso(id: i32, conn: DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(schema::cursos::table.find(id))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting row: {:?}", err);
            "Error deleting row into database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
