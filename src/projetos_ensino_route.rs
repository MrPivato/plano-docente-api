use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::projetos_ensino_model::{InsertableProjetoEnsino, ProjetosEnsino};
use crate::schema;
use crate::DbConn;

#[post("/projetos_ensino/<id_professor>", data = "<projetos_ensino>")]
pub fn create_projetos_ensino(
    conn: DbConn,
    projetos_ensino: Json<Vec<InsertableProjetoEnsino>>,
    id_professor: i32,
) -> Result<String, String> {
    match delete_projetos_ensino(&id_professor, &conn) {
        Ok(_) => println!("OK"),
        Err(_) => println!("Error"),
    };

    let inserted_rows = diesel::insert_into(schema::projetos_ensino::table)
        .values(&projetos_ensino.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/projetos_ensino/<id_professor>")]
pub fn read_projetos_ensino(
    id_professor: i32,
    conn: DbConn,
) -> Result<Json<Vec<ProjetosEnsino>>, String> {
    schema::projetos_ensino::table
        .filter(schema::projetos_ensino::id_professor.eq(id_professor))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying projetos_ensino: {:?}", err);
            "Error querying projetos_ensino from the database".into()
        })
        .map(Json)
}

pub fn delete_projetos_ensino(id_professor: &i32, conn: &DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(
        schema::projetos_ensino::table
            .filter(schema::projetos_ensino::id_professor.eq(id_professor)),
    )
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting row: {:?}", err);
        "Error deleting row into database".into()
    })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
