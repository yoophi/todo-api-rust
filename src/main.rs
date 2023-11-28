use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct TodosResponse {
    todos: Vec<Todo>
}

#[derive(Clone, Serialize)]
struct Todo {
    id: i64,
    title: String,
    is_completed: bool,
}

struct AppState {
    todos: std::sync::RwLock<Vec<Todo>>
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

async fn todo_detail() -> String {
    "todo_detail".to_string()
}

async fn todo_update() -> String {
    "todo_update".to_string()
}

async fn todo_delete() -> String {
    "todo_delete".to_string()
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

async fn todo_create() -> String {
    "todo_create".to_string()
}
