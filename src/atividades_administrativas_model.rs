use chrono::NaiveDateTime;
use crate::schema::atividades_administrativas;

#[derive(Serialize, Deserialize, Queryable)]
pub struct AtividadesAdministrativas {
    pub id: i32,
    pub id_professor: i32,
    pub atividade: String,
    pub numero_portaria: i32,
    pub ano_portaria: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "atividades_administrativas"]
pub struct InsertableAtividadeAdministrativa {
    pub id_professor: i32,
    pub atividade: String,
    pub numero_portaria: i32,
    pub ano_portaria: i32,
}

/*
 atividades_administrativas (id) {
        id -> Integer,
        id_professor -> Integer,
        atividade -> Varchar,
        numero_portaria -> Integer,
        ano_portaria -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
