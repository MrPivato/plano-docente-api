use diesel::{self, prelude::*};

use rocket_contrib::json::Json;

use crate::componentes_curriculares_model::{ComponentesCurriculares, InsertableComponenteCurricular, UpdatableComponenteCurricular};
use crate::schema;
use crate::DbConn;

#[post("/componentes_curriculares", data = "<componentes_curriculares>")]
pub fn create_componentes_curriculares(
    conn: DbConn,
    componentes_curriculares: Json<InsertableComponenteCurricular>,
) -> Result<String, String> {
    let inserted_rows = diesel::insert_into(schema::componentes_curriculares::table)
        .values(&componentes_curriculares.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error inserting row: {:?}", err);
            "Error inserting row into database".into()
        })?;

    Ok(format!("Inserted {} row(s).", inserted_rows))
}

#[get("/componentes_curriculares")]
pub fn read_componentes_curriculares(conn: DbConn) -> Result<Json<Vec<ComponentesCurriculares>>, String> {
    use crate::schema::componentes_curriculares::dsl::*;

    componentes_curriculares
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying page views: {:?}", err);
            "Error querying page views from the database".into()
        })
        .map(Json)
}

#[get("/componentes_curriculares/<id>")]
pub fn read_componentes_curriculares_unique(
    id: i32,
    conn: DbConn,
) -> Result<Json<Vec<ComponentesCurriculares>>, String> {
    schema::componentes_curriculares::table
        .find(id)
        .load(&conn.0)
        .map_err(|err| -> String {
            println!("Error querying componentes_curriculares: {:?}", err);
            "Error querying componentes_curriculares from the database".into()
        })
        .map(Json)
}

#[put("/componentes_curriculares/<id>", data = "<componentes_curriculares>")]
pub fn update_componentes_curriculares(
    id: i32,
    conn: DbConn,
    componentes_curriculares: Json<UpdatableComponenteCurricular>,
) -> Result<String, String> {
    let inserted_rows = diesel::update(schema::componentes_curriculares::table.find(id))
        .set(&componentes_curriculares.0)
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error updating row: {:?}", err);
            "Error updating row into database".into()
        })?;

    Ok(format!("Updated {} row(s).", inserted_rows))
}

#[delete("/componentes_curriculares/<id>")]
pub fn delete_componentes_curriculares(id: i32, conn: DbConn) -> Result<String, String> {
    let deleted_rows = diesel::delete(schema::componentes_curriculares::table.find(id))
        .execute(&conn.0)
        .map_err(|err| -> String {
            println!("Error deleting row: {:?}", err);
            "Error deleting row into database".into()
        })?;

    Ok(format!("Deleted {} row(s).", deleted_rows))
}
