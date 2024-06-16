use actix_web::{web, Responder, Error};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use tracing::log::info;
use crate::db::operations;
use crate::db::models::{NewTodo, NewUser, TodoId, UpdateTodo, UpdateUser, UserId};

pub async fn get_all_users(pool: web::Data<Pool<AsyncPgConnection>>) -> Result<impl Responder, Error> {
    info!("get_all_users srv called");

    let results = operations::get_all_users(pool).await;

    Ok(web::Json(results))
}

pub async fn get_user_by_id(pool: web::Data<Pool<AsyncPgConnection>>, user_id: web::Path<UserId>) -> Result<impl Responder, Error> {
    info!("get_user_by_id srv called");

    let result = operations::get_user_by_id(pool, user_id.into_inner()).await;

    Ok(web::Json(result))
}

pub async fn create_user(pool: web::Data<Pool<AsyncPgConnection>>, new_user: web::Json<NewUser>) -> Result<impl Responder, Error> {
    info!("create_user srv called");

    let result = operations::create_user(pool, new_user.into_inner()).await;

    Ok(web::Json(result))
}

pub async fn delete_user(pool: web::Data<Pool<AsyncPgConnection>>, user_id: web::Path<UserId>) -> Result<impl Responder, Error> {
    info!("delete_user srv called");

    let result = operations::delete_user(pool, user_id.into_inner()).await;

    Ok(web::Json(result))
}

pub async fn update_user(pool: web::Data<Pool<AsyncPgConnection>>, user_id: web::Path<UserId>, updated_user: web::Json<UpdateUser>) -> Result<impl Responder, Error> {
    info!("update_user srv called");

    let result = operations::update_user(pool, user_id.into_inner(), updated_user.into_inner()).await;

    Ok(web::Json(result))
}

pub async fn get_all_todos(pool: web::Data<Pool<AsyncPgConnection>>) -> Result<impl Responder, Error> {
    info!("get_all_todos service called");
    let results = operations::get_all_todos(pool).await;
    Ok(web::Json(results))
}

pub async fn get_todo_by_id(pool: web::Data<Pool<AsyncPgConnection>>, todo_id: web::Path<TodoId>) -> Result<impl Responder, Error> {
    info!("get_todo_by_id service called");
    let result = operations::get_todo_by_id(pool, todo_id.into_inner()).await;
    Ok(web::Json(result))
}

pub async fn create_todo(pool: web::Data<Pool<AsyncPgConnection>>, new_todo: web::Json<NewTodo>) -> Result<impl Responder, Error> {
    info!("create_todo service called");
    let result = operations::create_todo(pool, new_todo.into_inner()).await;
    Ok(web::Json(result))
}

pub async fn delete_todo(pool: web::Data<Pool<AsyncPgConnection>>, todo_id: web::Path<TodoId>) -> Result<impl Responder, Error> {
    info!("delete_todo service called");
    let result = operations::delete_todo(pool, todo_id.into_inner()).await;
    Ok(web::Json(result))
}

pub async fn update_todo(pool: web::Data<Pool<AsyncPgConnection>>, todo_id: web::Path<TodoId>, updated_todo: web::Json<UpdateTodo>) -> Result<impl Responder, Error> {
    info!("update_todo service called");
    let result = operations::update_todo(pool, todo_id.into_inner(), updated_todo.into_inner()).await;
    Ok(web::Json(result))
}

pub async fn get_all_todos_for_user(pool: web::Data<Pool<AsyncPgConnection>>, user_id: web::Path<UserId>) -> Result<impl Responder, Error> {
    info!("get_all_todos_for_user service called");
    let result = operations::get_all_todos_for_user(pool, user_id.into_inner()).await;
    Ok(web::Json(result))
}