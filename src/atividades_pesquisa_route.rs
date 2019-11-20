use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::atividades_pesquisa_model::{AtividadesPesquisa, InsertableAtividadePesquisa};
use crate::schema;
use crate::DbConn;

#[post("/atividades_pesquisa/<id_professor>", data = "<atividades_pesquisa>")]
pub fn create_atividades_pesquisa(
    conn: DbConn,
    atividades_pesquisa: Json<Vec<InsertableAtividadePesquisa>>,
    id_professor: i32,
) -> Result<String, String> {
    match delete_atividades_pesquisa(&id_professor, &conn) {
        Ok(_) => println!("OK"),
        Err(_) => println!("Error"),
    };

    let inserted_rows = diesel::insert_into(schema::atividades_pesquisa::table)
        .values(&atividades_pesquisa.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/atividades_pesquisa/<id_professor>")]
pub fn read_atividades_pesquisa(
    id_professor: i32,
    conn: DbConn,
) -> Result<Json<Vec<AtividadesPesquisa>>, String> {
    schema::atividades_pesquisa::table
        .filter(schema::atividades_pesquisa::id_professor.eq(id_professor))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying atividades_pesquisa: {:?}", err);
            "Error querying atividades_pesquisa from the database".into()
        })
        .map(Json)
}

pub fn delete_atividades_pesquisa(id_professor: &i32, conn: &DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(
        schema::atividades_pesquisa::table
            .filter(schema::atividades_pesquisa::id_professor.eq(id_professor)),
    )
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting row: {:?}", err);
        "Error deleting row into database".into()
    })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
