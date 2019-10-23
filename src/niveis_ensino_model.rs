use chrono::NaiveDateTime;

use crate::schema::niveis_ensino;

#[derive(Serialize, Deserialize, Queryable)]
pub struct NiveisEnsino {
    pub id: i32,
    pub nome: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "niveis_ensino"]
pub struct InsertableNiveisEnsino {
    pub nome: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "niveis_ensino"]
pub struct UpdatableNiveisEnsino {
    pub nome: Option<String>,
}

/*
    niveis_ensino (id) {
        id -> Integer,
        nome -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/