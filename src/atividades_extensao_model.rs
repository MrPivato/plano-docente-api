use crate::schema::atividades_extensao;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable)]
pub struct AtividadesExtensao {
    pub id: i32,
    pub id_professor: i32,
    pub titulo: String,
    pub numero_edital: i32,
    pub ano_edital: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "atividades_extensao"]
pub struct InsertableAtividadeExtensao {
    pub id_professor: i32,
    pub titulo: String,
    pub numero_edital: i32,
    pub ano_edital: i32,
}

/*
    atividades_extensao (id) {
        id -> Integer,
        id_professor -> Integer,
        titulo -> Varchar,
        numero_edital -> Integer,
        ano_edital -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
