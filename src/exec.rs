use crate::models::{ScriptRequest, ScriptResponse, ScriptType};
use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::NamedTempFile;
use uuid::Uuid;

pub struct ScriptExecutor;

impl ScriptExecutor {
    pub fn execute(script_req: &ScriptRequest) -> Result<ScriptResponse, String> {
        let execution_id = Uuid::new_v4().to_string();

        // 创建临时文件
        let mut temp_file =
            NamedTempFile::new().map_err(|e| format!("Failed to create temp file: {}", e))?;

        // 写入脚本内容
        temp_file
            .write_all(script_req.content.as_bytes())
            .map_err(|e| format!("Failed to write script: {}", e))?;

        println!("file_path: {}", temp_file.path().display());

        // 设置执行权限
        #[cfg(unix)]
        fs::set_permissions(temp_file.path(), fs::Permissions::from_mode(0o755))
            .map_err(|e| format!("Failed to set permissions: {}", e))?;

        #[cfg(not(unix))]
        {
            let mut perms = fs::metadata(temp_file.path())
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_readonly(false);
            fs::set_permissions(temp_file.path(), perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }

        // 根据脚本类型选择执行器
        let output = match script_req.script_type {
            ScriptType::Python2 => Command::new("python2").arg(temp_file.path()).output(),
            ScriptType::Shell => Command::new("/bin/bash").arg(temp_file.path()).output(),
        }
        .map_err(|e| format!("Execution failed: {}", e))?;

        Ok(ScriptResponse {
            execution_id,
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
}

// 原有的自定义脚本执行处理函数
pub async fn execute_script(script_req: &ScriptRequest) -> Result<ScriptResponse, String> {
    ScriptExecutor::execute(&script_req)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_script() {
        let script_req = ScriptRequest {
            script_type: ScriptType::Python2,
            content: "print('Hello, World!')".to_string(),
        };

        let result = execute_script(&script_req).await;

        assert!(result.is_ok());

        let script_resp = result.unwrap();

        assert_eq!(script_resp.stdout, "Hello, World!\n");
        assert_eq!(script_resp.stderr, "");
        assert_eq!(script_resp.exit_code, 0);
    }
}