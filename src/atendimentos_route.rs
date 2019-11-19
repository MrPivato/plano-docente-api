use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::atendimentos_model::{Atendimentos, InsertableAtendimento};
use crate::schema;
use crate::DbConn;

#[post("/atendimentos/<id_professor>", data = "<atendimentos>")]
pub fn create_atendimentos(
    conn: DbConn,
    atendimentos: Json<Vec<InsertableAtendimento>>,
    id_professor: i32,
) -> Result<String, String> {
    // deleta os atendimentos com id do prof antes do insert
    delete_atendimentos(&id_professor, &conn);

    let inserted_rows = diesel::insert_into(schema::atendimentos::table)
        .values(&atendimentos.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/atendimentos/<id_professor>")]
pub fn read_atendimentos(
    id_professor: i32,
    conn: DbConn,
) -> Result<Json<Vec<Atendimentos>>, String> {
    schema::atendimentos::table
        .filter(schema::atendimentos::id_professor.eq(id_professor))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying atendimentos: {:?}", err);
            "Error querying atendimentos from the database".into()
        })
        .map(Json)
}

pub fn delete_atendimentos(id_professor: &i32, conn: &DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(
        schema::atendimentos::table.filter(schema::atendimentos::id_professor.eq(id_professor)),
    )
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting row: {:?}", err);
        "Error deleting row into database".into()
    })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
