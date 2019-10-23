use chrono::NaiveDateTime;

use crate::schema::professors;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Professors {
    pub id: i32,
    pub user_id: i32,
    pub nome: Option<String>,
    pub sexo: Option<String>,
    pub email: Option<String>,
    pub area_conhecimento: Option<String>,
    pub categoria: Option<String>,
    pub regime_trabalho: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "professors"]
pub struct InsertableProfessor {
    pub user_id: i32,
    pub nome: Option<String>,
    pub sexo: Option<String>,
    pub email: Option<String>,
    pub area_conhecimento: Option<String>,
    pub categoria: Option<String>,
    pub regime_trabalho: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "professors"]
pub struct UpdatableProfessor {
    pub user_id: i32,
    pub nome: Option<String>,
    pub sexo: Option<String>,
    pub email: Option<String>,
    pub area_conhecimento: Option<String>,
    pub categoria: Option<String>,
    pub regime_trabalho: Option<String>,
}

/*
    professors (id) {
        id -> Integer,
        user_id -> Integer,
        nome -> Nullable<Varchar>,
        sexo -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        area_conhecimento -> Nullable<Varchar>,
        categoria -> Nullable<Varchar>,
        regime_trabalho -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
