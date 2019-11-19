use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::reunioes_formacao_docente_model::{
    InsertableReuniaoFormacaoDocente, ReunioesFormacaoDocente,
};
use crate::schema;
use crate::DbConn;

#[post(
    "/reunioes_formacao_docente/<id_professor>",
    data = "<reunioes_formacao_docente>"
)]
pub fn create_reunioes_formacao_docente(
    conn: DbConn,
    reunioes_formacao_docente: Json<Vec<InsertableReuniaoFormacaoDocente>>,
    id_professor: i32,
) -> Result<String, String> {
    delete_reunioes_formacao_docente(&id_professor, &conn);

    let inserted_rows = diesel::insert_into(schema::reunioes_formacao_docente::table)
        .values(&reunioes_formacao_docente.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/reunioes_formacao_docente/<id_professor>")]
pub fn read_reunioes_formacao_docente(
    id_professor: i32,
    conn: DbConn,
) -> Result<Json<Vec<ReunioesFormacaoDocente>>, String> {
    schema::reunioes_formacao_docente::table
        .filter(schema::reunioes_formacao_docente::id_professor.eq(id_professor))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying reunioes_formacao_docente: {:?}", err);
            "Error querying reunioes_formacao_docente from the database".into()
        })
        .map(Json)
}

pub fn delete_reunioes_formacao_docente(
    id_professor: &i32,
    conn: &DbConn,
) -> Result<String, String> {
    let deleted_rows = diesel::delete(
        schema::reunioes_formacao_docente::table
            .filter(schema::reunioes_formacao_docente::id_professor.eq(id_professor)),
    )
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting row: {:?}", err);
        "Error deleting row into database".into()
    })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
