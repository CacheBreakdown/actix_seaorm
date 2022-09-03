use actix_web::{
    get, web, Error as ActixError, HttpResponse, Responder, Result as ActixResult, Scope,
};

use crate::AppState;

#[get("/all")]
async fn get_todos(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let todos = state.todo_repository.get_todos().await;
    Ok(web::Json(todos))
}

#[get("/add")]
async fn add_todos(state: web::Data<AppState>) -> impl Responder {
    state.todo_repository.add().await;
    HttpResponse::Ok().body("add success")
}

#[get("/delete/{id}")]
async fn delete_todos(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    state.todo_repository.delete(id.into_inner()).await;
    HttpResponse::Ok().body("delete success")
}

#[get("/update/{id}")]
async fn update_todos(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    state.todo_repository.update(id.into_inner()).await;
    HttpResponse::Ok().body("update success")
}

#[get("/find/{id}")]
async fn find_todos(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> ActixResult<impl Responder, ActixError> {
    let todos = state.todo_repository.find(id.into_inner()).await;
    Ok(web::Json(todos))
}

pub fn todos_handler() -> Scope {
    web::scope("/todos")
        .service(get_todos)
        .service(add_todos)
        .service(delete_todos)
        .service(update_todos)
        .service(find_todos)
}
