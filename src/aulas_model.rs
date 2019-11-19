use crate::schema::aulas;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Aulas {
    pub id: i32,
    pub id_professor: i32,
    pub curso_id: Option<i32>,
    pub componente_curricular_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "aulas"]
pub struct InsertableAula {
    pub id_professor: i32,
    pub curso_id: Option<i32>,
    pub componente_curricular_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct AulasCC {
    pub curso_id: Option<i32>,
    pub componente_curricular_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct AulasFull {
    pub id: i32,
    pub id_professor: i32,
    pub curso_id: Option<i32>,
    pub componente_curricular_id: Option<i32>,
    pub nome_curso: Option<String>,
    pub nivel_ensino_id: Option<i32>,
    pub nome_componente_curricular: Option<String>,
    pub ch_semanal: f32,
}
/*
    aulas (id) {
        id -> Integer,
        id_professor -> Integer,
        curso_id -> Nullable<Integer>,
        componente_curricular_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
