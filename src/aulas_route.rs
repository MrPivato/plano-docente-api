use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::aulas_model::{AulasCC, AulasFull, InsertableAula};
use crate::schema;
use crate::DbConn;

#[post("/aulas/<id_professor>", data = "<aulas>")]
pub fn create_aulas(
    conn: DbConn,
    aulas: Json<Vec<InsertableAula>>,
    id_professor: i32,
) -> Result<String, String> {
    match delete_aulas(id_professor, &conn) {
        Ok(_) => println!("OK"),
        Err(_) => println!("Error"),
    };

    let inserted_rows = diesel::insert_into(schema::aulas::table)
        .values(&aulas.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/aulas/<id_professor>/cc")]
pub fn read_aulas_cc(id_professor: i32, conn: DbConn) -> Result<Json<Vec<AulasCC>>, String> {
    schema::aulas::table
        .filter(schema::aulas::id_professor.eq(id_professor))
        .select((
            schema::aulas::curso_id,
            schema::aulas::componente_curricular_id,
        ))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying aulas: {:?}", err);
            "Error querying aulas from the database".into()
        })
        .map(Json)
}

#[get("/aulas/<id_professor>/full")]
pub fn read_aulas_full(id_professor: i32, conn: DbConn) -> Result<Json<Vec<AulasFull>>, String> {
    schema::aulas::table
        .inner_join(schema::componentes_curriculares::table)
        .inner_join(schema::cursos::table)
        .filter(schema::aulas::id_professor.eq(id_professor))
        .select((
            schema::aulas::id,
            schema::aulas::id_professor,
            schema::aulas::curso_id,
            schema::aulas::componente_curricular_id,
            schema::cursos::nome,
            schema::cursos::nivel_ensino_id,
            schema::componentes_curriculares::nome,
            schema::componentes_curriculares::ch_semanal,
        ))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying aulas: {:?}", err);
            "Error querying aulas from the database".into()
        })
        .map(Json)
}

pub fn delete_aulas(id_professor: i32, conn: &DbConn) -> Result<String, String> {
    let deleted_rows =
        diesel::delete(schema::aulas::table.filter(schema::aulas::id_professor.eq(id_professor)))
            .execute(&conn.0)
            .map_err(|err| -> String {
                println!("Error deleting row: {:?}", err);
                "Error deleting row into database".into()
            })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
