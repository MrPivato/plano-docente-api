use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::atividades_administrativas_model::{
    AtividadesAdministrativas, InsertableAtividadeAdministrativa,
};
use crate::schema;
use crate::DbConn;

#[post(
    "/atividades_administrativas/<id_professor>",
    data = "<atividades_administrativas>"
)]
pub fn create_atividades_administrativas(
    conn: DbConn,
    atividades_administrativas: Json<Vec<InsertableAtividadeAdministrativa>>,
    id_professor: i32,
) -> Result<String, String> {
    match delete_atividades_administrativas(id_professor, &conn) {
        Ok(_) => println!("OK"),
        Err(_) => println!("Error"),
    };

    let inserted_rows = diesel::insert_into(schema::atividades_administrativas::table)
        .values(&atividades_administrativas.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/atividades_administrativas/<id_professor>")]
pub fn read_atividades_administrativas(
    id_professor: i32,
    conn: DbConn,
) -> Result<Json<Vec<AtividadesAdministrativas>>, String> {
    schema::atividades_administrativas::table
        .filter(schema::atividades_administrativas::id_professor.eq(id_professor))
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying atividades_administrativas: {:?}", err);
            "Error querying atividades_administrativas from the database".into()
        })
        .map(Json)
}

pub fn delete_atividades_administrativas(
    id_professor: i32,
    conn: &DbConn,
) -> Result<String, String> {
    let deleted_rows = diesel::delete(
        schema::atividades_administrativas::table
            .filter(schema::atividades_administrativas::id_professor.eq(id_professor)),
    )
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting row: {:?}", err);
        "Error deleting row into database".into()
    })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
