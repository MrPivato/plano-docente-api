use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::atividades_extensao_model::{InsertableAtividadeExtensao, AtividadesExtensao};
use crate::schema;
use crate::DbConn;

#[post("/atividades_extensao/<id_professor>", data = "<atividades_extensao>")]
pub fn create_atividades_extensao(
    conn: DbConn,
    atividades_extensao: Json<Vec<InsertableAtividadeExtensao>>,
    id_professor: i32,
) -> Result<String, String> {

    delete_atividades_extensao(&id_professor, &conn);

    let inserted_rows = diesel::insert_into(schema::atividades_extensao::table)
        .values(&atividades_extensao.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/atividades_extensao/<id_professor>")]
pub fn read_atividades_extensao(id_professor: i32, conn: DbConn) -> Result<Json<Vec<AtividadesExtensao>>, String> {
    schema::atividades_extensao::table
        .filter(schema::atividades_extensao::id_professor
        .eq(id_professor))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying atividades_extensao: {:?}", err);
            "Error querying atividades_extensao from the database".into()
        })
        .map(Json)
}

pub fn delete_atividades_extensao(id_professor: &i32, conn: &DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(schema::atividades_extensao::table.filter(schema::atividades_extensao::id_professor.eq(id_professor)))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting row: {:?}", err);
            "Error deleting row into database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
