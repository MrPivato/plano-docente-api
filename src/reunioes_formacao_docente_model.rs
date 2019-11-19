use crate::schema::reunioes_formacao_docente;
use chrono::{NaiveDateTime, NaiveTime};

#[derive(Serialize, Deserialize, Queryable)]
pub struct ReunioesFormacaoDocente {
    pub id: i32,
    pub id_professor: i32,
    pub dia: String,
    pub hora_inicio: NaiveTime,
    pub hora_fim: NaiveTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "reunioes_formacao_docente"]
pub struct InsertableReuniaoFormacaoDocente {
    pub id_professor: i32,
    pub dia: String,
    pub hora_inicio: NaiveTime,
    pub hora_fim: NaiveTime,
}

/*
    reunioes_formacao_docente (id) {
        id -> Integer,
        id_professor -> Integer,
        dia -> Varchar,
        hora_inicio -> Time,
        hora_fim -> Time,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
