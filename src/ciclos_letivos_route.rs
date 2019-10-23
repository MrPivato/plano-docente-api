use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::schema;
use crate::ciclos_letivos_model::{InsertableCicloLetivo, UpdatableCicloLetivo, CicloLetivos};
use crate::DbConn;

#[post("/ciclos_letivos", data = "<ciclos_letivos>")]
pub fn create_ciclos_letivos(conn: DbConn, ciclos_letivos: Json<InsertableCicloLetivo>) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::ciclos_letivos::table)
    .values(&ciclos_letivos.0)
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error inserting row: {:?}", err);
        "Error inserting row into database".into()
    })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/ciclos_letivos")]
pub fn read_ciclos_letivos(conn: DbConn) -> Result<Json<Vec<CicloLetivos>>, String> {
    use crate::schema::ciclos_letivos::dsl::*;

    ciclos_letivos
    .load(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying page views: {:?}", err);
        "Error querying page views from the database".into()
    })
    .map(Json)
}

#[get("/ciclos_letivos/<id>")]
pub fn read_ciclos_letivos_unique(id: i32, conn: DbConn) -> Result<Json<Vec<CicloLetivos>>, String> {
    schema::ciclos_letivos::table.find(id)
    .load(&conn.0)
    .map_err(|err| -> String {
        println!("Error querying ciclos_letivos: {:?}", err);
        "Error querying ciclos_letivos from the database".into()
    })
    .map(Json)
}


#[put("/ciclos_letivos/<id>", data = "<ciclos_letivos>")]
pub fn update_ciclos_letivos(id: i32, conn: DbConn, ciclos_letivos: Json<UpdatableCicloLetivo>) -> Result<String, String> {
     let inserted_rows = diesel::update(schema::ciclos_letivos::table.find(id))
    .set(&ciclos_letivos.0)
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error updating row: {:?}", err);
        "Error updating row into database".into()
    })?;

    Ok(format!("Updated {} row(s).", inserted_rows))

}

#[delete("/ciclos_letivos/<id>")]
pub fn delete_ciclos_letivos(id: i32, conn: DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(schema::ciclos_letivos::table.find(id))
    .execute(&conn.0)
    .map_err(|err| -> String {
        println!("Error deleting row: {:?}", err);
        "Error deleting row into database".into()
    })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}