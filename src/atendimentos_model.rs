use chrono::{ NaiveDateTime, NaiveTime };
use crate::schema::atendimentos;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Atendimentos {
    pub id: i32,
    pub id_professor: i32,
    pub dia: String,
    pub hora_inicio: NaiveTime,
    pub hora_fim: NaiveTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "atendimentos"]
pub struct InsertableAtendimento {
    pub id_professor: i32,
    pub dia: String,
    pub hora_inicio: NaiveTime,
    pub hora_fim: NaiveTime,
}

/*
    atendimentos (id) {
        id -> Integer,
        id_professor -> Integer,
        dia -> Varchar,
        hora_inicio -> Time,
        hora_fim -> Time,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
