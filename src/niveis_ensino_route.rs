use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::schema;
use crate::niveis_ensino_model::{InsertableNiveisEnsino, UpdatableNiveisEnsino, NiveisEnsino};
use crate::DbConn;

#[post("/niveis_ensino", data = "<niveis_ensino>")]
pub fn create_niveis_ensino(conn: DbConn, niveis_ensino: Json<InsertableNiveisEnsino>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::niveis_ensino::table)
    .values(&niveis_ensino.0)
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error inserting row: {:?}", err);
        "Error inserting row into database".into()
    })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/niveis_ensino")]
pub fn read_niveis_ensino(conn: DbConn) -> Result<Json<Vec<NiveisEnsino>>, String> {
    use crate::schema::niveis_ensino::dsl::*;

    niveis_ensino
    .load(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying page views: {:?}", err);
        "Error querying page views from the database".into()
    })
    .map(Json)
}

#[get("/niveis_ensino/<id>")]
pub fn read_niveis_ensino_unique(id: i32, conn: DbConn) -> Result<Json<Vec<NiveisEnsino>>, String> {
    schema::niveis_ensino::table.find(id)
    .load(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying niveis_ensino: {:?}", err);
        "Error querying niveis_ensino from the database".into()
    })
    .map(Json)
}


#[put("/niveis_ensino/<id>", data = "<niveis_ensino>")]
pub fn update_niveis_ensino(id: i32, conn: DbConn, niveis_ensino: Json<UpdatableNiveisEnsino>) -> Result<String, String> {
     let inserted_rows = diesel::update(schema::niveis_ensino::table.find(id))
    .set(&niveis_ensino.0)
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error updating row: {:?}", err);
        "Error updating row into database".into()
    })?;

    Ok(format!("Updated {} row(s).", inserted_rows))

}

#[delete("/niveis_ensino/<id>")]
pub fn delete_niveis_ensino(id: i32, conn: DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(schema::niveis_ensino::table.find(id))
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting row: {:?}", err);
        "Error deleting row into database".into()
    })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}