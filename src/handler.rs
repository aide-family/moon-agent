use crate::models::{AlertItem, ScriptRequest, ScriptType};
use crate::tasks::TaskAction;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TaskActionRequest {
    pub action: TaskAction,
    pub alert: AlertItem,
    pub biz_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTaskRequest {
    pub biz_id: String,
    pub script_type: ScriptType,
    pub content: String,
}

impl AddTaskRequest {
    fn to_script_request(&self) -> ScriptRequest {
        ScriptRequest {
            script_type: self.script_type.clone(),
            content: self.content.clone(),
        }
    }
}

pub struct API;

impl API {
    // 执行任务脚本
    pub async fn execute_task(
        data: web::Data<AppState>,
        req: web::Json<TaskActionRequest>,
    ) -> impl Responder {
        let mut task_manager = data.task_manager.clone();

        // 根据action执行不同的任务
        let result = match req.action {
            TaskAction::ProcessAlert => {
                task_manager
                    .process_alert(&req.biz_id, req.alert.clone())
                    .await
            }
        };

        match result {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(e) => HttpResponse::InternalServerError().body(e),
        }
    }

    pub async fn list_tasks(data: web::Data<AppState>) -> impl Responder {
        let task_manager = data.task_manager.clone();

        let tasks = task_manager.list_tasks();

        HttpResponse::Ok().json(tasks)
    }

    pub async fn add_task(
        data: web::Data<AppState>,
        req: web::Json<AddTaskRequest>,
    ) -> impl Responder {
        let mut task_manager = data.task_manager.clone();

        let script_req = req.to_script_request();
        task_manager.add_task(req.biz_id.clone(), script_req.clone());
        HttpResponse::Ok().json(script_req)
    }

    pub async fn remove_task(data: web::Data<AppState>, req: web::Json<String>) -> impl Responder {
        let mut task_manager = data.task_manager.clone();

        let result = task_manager.remove_task(&req.0);

        match result {
            Some(script_req) => HttpResponse::Ok().json(script_req),
            None => HttpResponse::NotFound().body("Task not found"),
        }
    }

    pub async fn get_task(data: web::Data<AppState>, req: web::Json<String>) -> impl Responder {
        let task_manager = data.task_manager.clone();

        let result = task_manager.get_task(&req.0);

        match result {
            Some(script_req) => HttpResponse::Ok().json(script_req),
            None => HttpResponse::NotFound().body("Task not found"),
        }
    }
}
