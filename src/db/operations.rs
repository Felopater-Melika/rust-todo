use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use diesel_async::pooled_connection::deadpool::Pool;
use tracing::log::info;
use crate::db::models::{NewUser, User, UpdateUser, UserId, Todo, TodoId, UpdateTodo, NewTodo};
use crate::schema::todos::dsl::{todos, id as todo_id, user_id as todo_user_id};
use crate::schema::users::dsl::{users, id as user_id};


pub async fn get_all_users(pool: web::Data<Pool<AsyncPgConnection>>) -> Vec<User> {
    info!("get_all_users ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = users.load::<User>(&mut conn).await.expect("Failed to get all users");
    info!("Result: {:?}", result);
    result
}

pub async fn get_user_by_id(pool: web::Data<Pool<AsyncPgConnection>>, user_id_arg: UserId) -> User {
    info!("get_user_by_id ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = users.filter(user_id.eq(user_id_arg.id)).get_result::<User>(&mut conn).await.expect("Failed to get user by id");
    info!("Result: {:?}", result);
    result
}

pub async fn create_user(pool: web::Data<Pool<AsyncPgConnection>>, new_user: NewUser) -> User {
    info!("create_user ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = diesel::insert_into(users).values(new_user).get_result::<User>(&mut conn).await.expect("Failed to create user");
    info!("Result: {:?}", result);
    result
}

pub async fn delete_user(pool: web::Data<Pool<AsyncPgConnection>>, user_id_arg: UserId) -> usize {
    info!("delete_user ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = diesel::delete(users.find(user_id_arg.id)).execute(&mut conn).await.expect("Failed to delete user");
    info!("Result: {:?}", result);
    result
}

pub async fn update_user(pool: web::Data<Pool<AsyncPgConnection>>, user_id_arg: UserId, updated_user: UpdateUser) -> User {
    info!("update_user ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = diesel::update(users.find(user_id_arg.id)).set(updated_user).get_result::<User>(&mut conn).await.expect("Failed to update user");
    info!("Result: {:?}", result);
    result
}


pub async fn get_all_todos(pool: web::Data<Pool<AsyncPgConnection>>) -> Vec<Todo> {
    info!("get_all_todos ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = todos.load::<Todo>(&mut conn).await.expect("Failed to get all todos");
    info!("Result: {:?}", result);
    result
}

pub async fn get_todo_by_id(pool: web::Data<Pool<AsyncPgConnection>>, todo_id_arg: TodoId) -> Todo {
    info!("get_todo_by_id ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = todos.filter(todo_id.eq(todo_id_arg.id)).get_result::<Todo>(&mut conn).await.expect("Failed to get todo by id");
    info!("Result: {:?}", result);
    result
}

pub async fn create_todo(pool: web::Data<Pool<AsyncPgConnection>>, new_todo: NewTodo) -> Todo {
    info!("create_todo ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = diesel::insert_into(todos).values(new_todo).get_result::<Todo>(&mut conn).await.expect("Failed to create todo");
    info!("Result: {:?}", result);
    result
}

pub async fn delete_todo(pool: web::Data<Pool<AsyncPgConnection>>, todo_id_arg: TodoId) -> usize {
    info!("delete_todo ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = diesel::delete(todos.find(todo_id_arg.id)).execute(&mut conn).await.expect("Failed to delete todo");
    info!("Result: {:?}", result);
    result
}

pub async fn update_todo(pool: web::Data<Pool<AsyncPgConnection>>, todo_id_arg: TodoId, updated_todo: UpdateTodo) -> Todo {
    info!("update_todo ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = diesel::update(todos.find(todo_id_arg.id)).set(updated_todo).get_result::<Todo>(&mut conn).await.expect("Failed to update todo");
    info!("Result: {:?}", result);
    result
}

pub async fn get_all_todos_for_user(pool: web::Data<Pool<AsyncPgConnection>>, user_id_arg: UserId) -> Vec<Todo> {
    info!("get_all_todos_for_user ops called");
    let mut conn = pool.get().await.expect("Failed to get DB connection from pool");
    let result = todos.filter(todo_user_id.eq(user_id_arg.id)).load::<Todo>(&mut conn).await.expect("Failed to get all todos for user");
    info!("Result: {:?}", result);
    result
}