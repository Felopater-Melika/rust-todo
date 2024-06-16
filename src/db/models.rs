use diesel::prelude::*;
use crate::schema::{users, todos};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
    pub completed: bool,
    pub user_id: i32,
    pub description: String,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct UpdateTodo {
    pub title: String,
    pub completed: bool,
    pub user_id: i32,
    pub description: String,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct TodoId {
    pub(crate) id: i32,
}


#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserId {
    pub(crate) id: i32,
}