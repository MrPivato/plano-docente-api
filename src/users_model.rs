use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Users {
    pub id: i32,
    pub login: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub login: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct UpdatableUser {
    pub login: String,
    pub password_hash: String,
    pub role: String,
}

/*
    users (id) {
        id -> Integer,
        login -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
*/
