mod exec;
mod handler;
mod models;
mod tasks;

use crate::handler::API;
use actix_web::{web, App, HttpServer};
use tasks::TaskManager;

// 共享状态
struct AppState {
    task_manager: TaskManager,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载环境变量
    dotenv::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    println!("Starting server at http://{}", bind_address);

    // 创建任务管理器
    let task_manager = TaskManager::new();
    let app_state = web::Data::new(AppState { task_manager });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/agent/task/execute", web::post().to(API::execute_task))
            .route("/agent/task/add", web::post().to(API::add_task))
            .route("/agent/task/list", web::get().to(API::list_tasks))
            .route("/agent/task/remove", web::post().to(API::remove_task))
            .route("/agent/task/get", web::post().to(API::get_task))
    })
    .bind(bind_address)?
    .run()
    .await
}
