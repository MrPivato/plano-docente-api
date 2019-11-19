use crate::schema::projetos_ensino;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct ProjetosEnsino {
    pub id: i32,
    pub id_professor: i32,
    pub titulo: String,
    pub numero_edital: i32,
    pub ano_edital: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "projetos_ensino"]
pub struct InsertableProjetoEnsino {
    pub id_professor: i32,
    pub titulo: String,
    pub numero_edital: i32,
    pub ano_edital: i32,
}

/*
    projetos_ensino (id) {
        id -> Integer,
        id_professor -> Integer,
        titulo -> Varchar,
        numero_edital -> Integer,
        ano_edital -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
