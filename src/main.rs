use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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

async fn todo_list() -> String {
    "todo_list".to_string()
}

async fn todo_create() -> String {
    "todo_create".to_string()
}
