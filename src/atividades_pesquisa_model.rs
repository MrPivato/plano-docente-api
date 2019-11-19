use crate::schema::atividades_pesquisa;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct AtividadesPesquisa {
    pub id: i32,
    pub id_professor: i32,
    pub titulo: String,
    pub numero_edital: i32,
    pub ano_edital: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "atividades_pesquisa"]
pub struct InsertableAtividadePesquisa {
    pub id_professor: i32,
    pub titulo: String,
    pub numero_edital: i32,
    pub ano_edital: i32,
}

/*
    atividades_pesquisa (id) {
        id -> Integer,
        id_professor -> Integer,
        titulo -> Varchar,
        numero_edital -> Integer,
        ano_edital -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
