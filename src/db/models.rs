use diesel::prelude::*;
use crate::schema::{users, todos};
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub completed: bool,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}