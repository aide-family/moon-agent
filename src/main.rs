use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::NamedTempFile;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ScriptRequest {
    script_type: ScriptType,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ScriptType {
    Python2,
    Shell,
}

#[derive(Debug, Serialize)]
struct ScriptResponse {
    execution_id: String,
    stdout: String,
    stderr: String,
    exit_code: i32,
}

async fn execute_script(script_req: web::Json<ScriptRequest>) -> impl Responder {
    let execution_id = Uuid::new_v4().to_string();

    // 创建临时文件
    let mut temp_file = match NamedTempFile::new() {
        Ok(file) => file,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(format!("Failed to create temp file: {}", e))
        }
    };

    // 写入脚本内容
    if let Err(e) = temp_file.write_all(script_req.content.as_bytes()) {
        return HttpResponse::InternalServerError().json(format!("Failed to write script: {}", e));
    }

    // 设置执行权限
    #[cfg(unix)]
    if let Err(e) = fs::set_permissions(temp_file.path(), fs::Permissions::from_mode(0o755)) {
        return HttpResponse::InternalServerError()
            .json(format!("Failed to set permissions: {}", e));
    }

    #[cfg(not(unix))]
    {
        let mut perms = match fs::metadata(temp_file.path()) {
            Ok(metadata) => metadata.permissions(),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(format!("Failed to get file metadata: {}", e))
            }
        };
        perms.set_readonly(false);
        if let Err(e) = fs::set_permissions(temp_file.path(), perms) {
            return HttpResponse::InternalServerError()
                .json(format!("Failed to set permissions: {}", e));
        }
    }

    // 根据脚本类型选择执行器
    let output = match script_req.script_type {
        ScriptType::Python2 => Command::new("python2").arg(temp_file.path()).output(),
        ScriptType::Shell => Command::new("/bin/bash").arg(temp_file.path()).output(),
    };

    match output {
        Ok(output) => {
            let response = ScriptResponse {
                execution_id,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(-1),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Execution failed: {}", e)),
    }
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

    HttpServer::new(|| App::new().route("/execute", web::post().to(execute_script)))
        .bind(bind_address)?
        .run()
        .await
}
