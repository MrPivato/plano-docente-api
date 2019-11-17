use chrono::NaiveDateTime;

use crate::schema::cursos;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Cursos {
    pub id: i32,
    pub nome: Option<String>,
    pub nivel_ensino_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "cursos"]
pub struct InsertableCurso {
    pub nome: Option<String>,
    pub nivel_ensino_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "cursos"]
pub struct UpdatableCurso {
    pub nome: Option<String>,
    pub nivel_ensino_id: Option<i32>,
}

/*
    cursos (id) {
        id -> Integer,
        nome -> Nullable<Varchar>,
        nivel_ensino_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
