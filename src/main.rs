use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::web::{Json, Path};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct EntityId {
    id: i64,
}

#[derive(Serialize)]
struct TodoResponse {
    todo: Todo,
}

#[derive(Serialize)]
struct TodosResponse {
    todos: Vec<Todo>,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
}

#[derive(Clone, Serialize)]
struct Todo {
    id: i64,
    title: String,
    is_completed: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

#[derive(Deserialize, Serialize)]
struct UpdateTodo {
    title: Option<String>,
    is_completed: Option<bool>,
}

struct AppState {
    todos: std::sync::RwLock<Vec<Todo>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(
        AppState {
            todos: std::sync::RwLock::new(
                vec![
                    Todo { id: 1, title: "study rust".to_string(), is_completed: false },
                    Todo { id: 2, title: "study actix_web".to_string(), is_completed: false },
                    Todo { id: 3, title: "deploy todo service".to_string(), is_completed: false },
                ]
            )
        }
    );
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(
                web::scope("/v1")
                    .service(
                        web::resource("/todos/{id}")
                            .route(web::get().to(todo_detail))
                            .route(web::put().to(todo_update))
                            .route(web::delete().to(todo_delete))
                    )
                    .service(
                        web::resource("/todos")
                            .route(web::get().to(todo_list))
                            .route(web::post().to(todo_create))
                    )
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn todo_detail(app_data: web::Data<AppState>, params: Path<EntityId>) -> impl Responder {
    let todos = app_data.todos.read().unwrap().clone();
    let todo = todos.iter().find(|t| t.id == params.id).unwrap().clone();
    let response = TodoResponse { todo };
    let serialized = serde_json::to_string(&response).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized)
}

async fn todo_update(app_data: web::Data<AppState>, params: Path<EntityId>, todo_input: Json<UpdateTodo>) -> impl Responder {
    let mut todos = app_data.todos.write().unwrap();
    let mut update_todo = todos.iter().find(|t| t.id == params.id).unwrap().clone();

    match &todo_input.title {
        Some(title) => {
            update_todo.title = title.clone();
        }
        _ => {}
    };
    match &todo_input.is_completed {
        Some(is_completed) => {
            update_todo.is_completed = is_completed.clone();
        }
        _ => {}
    };

    let current_index = todos.iter().position(|t| { t.id == params.id }).unwrap();
    todos.remove(current_index);
    todos.push(update_todo.clone());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&SuccessResponse { success: true }).unwrap())
}

async fn todo_delete(app_data: web::Data<AppState>, params: Path<EntityId>) -> impl Responder {
    let mut todos = app_data.todos.write().unwrap();
    let current_index = todos.iter().position(|t| { t.id == params.id }).unwrap();
    todos.remove(current_index);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&SuccessResponse { success: true }).unwrap())
}

async fn todo_list(app_data: web::Data<AppState>) -> impl Responder {
    let response = TodosResponse {
        todos: app_data.todos.read().unwrap().clone()
    };
    let serialized = serde_json::to_string(&response).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serialized)
}

async fn todo_create(app_data: web::Data<AppState>, todo_input: Json<NewTodo>) -> impl Responder {
    let mut todos = app_data.todos.write().unwrap();
    let max_id = todos.iter().max_by_key(|t| { t.id }).unwrap().id;
    let new_todo = Todo {
        id: max_id + 1,
        title: todo_input.title.clone(),
        is_completed: false,
    };
    todos.push(new_todo.clone());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&TodoResponse { todo: new_todo.clone() }).unwrap())
}
