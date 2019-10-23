use chrono::NaiveDateTime;

use crate::schema::ciclos_letivos;

#[derive(Serialize, Deserialize, Queryable)]
pub struct CicloLetivos {
    pub id: i32,
    pub ano: Option<i32>,
    pub semestre: Option<i32>,
    pub nivel_ensino_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "ciclos_letivos"]
pub struct InsertableCicloLetivo {
    pub ano: Option<i32>,
    pub semestre: Option<i32>,
    pub nivel_ensino_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "ciclos_letivos"]
pub struct UpdatableCicloLetivo {
    pub ano: Option<i32>,
    pub semestre: Option<i32>,
    pub nivel_ensino_id: Option<i32>,
}

/*
    ciclos_letivos (id) {
        id -> Integer,
        ano -> Nullable<Integer>,
        semestre -> Nullable<Integer>,
        nivel_ensino_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
