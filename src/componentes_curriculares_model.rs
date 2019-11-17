use chrono::NaiveDateTime;

use crate::schema::componentes_curriculares;

#[derive(Serialize, Deserialize, Queryable)]
pub struct ComponentesCurriculares {
    pub id: i32,
    pub nome: Option<String>,
    pub curso_id: Option<i32>,
    pub ch_semanal: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "componentes_curriculares"]
pub struct InsertableComponenteCurricular {
    pub nome: Option<String>,
    pub curso_id: Option<i32>,
    pub ch_semanal: f32,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "componentes_curriculares"]
pub struct UpdatableComponenteCurricular {
    pub nome: Option<String>,
    pub curso_id: Option<i32>,
    pub ch_semanal: f32,
}

/*
    componentes_curriculares (id) {
        id -> Integer,
        nome -> Nullable<Varchar>,
        curso_id -> Nullable<Integer>,
        ch_semanal -> Float,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
