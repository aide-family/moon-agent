use crate::exec::execute_script;
use crate::models::{AlertItem, ScriptRequest, ScriptResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 任务动作枚举
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum TaskAction {
    ProcessAlert,
    // 可以继续添加更多动作...
}

// 添加 Clone 派生宏
#[derive(Clone)]
pub struct TaskManager {
    tasks: HashMap<TaskAction, ScriptRequest>,
}

impl TaskManager {
    pub fn new() -> Self {
        let tasks = HashMap::new();

        Self { tasks }
    }

    // 处理告警的方法
    pub async fn process_alert(&mut self, alert: AlertItem) -> Result<ScriptResponse, String> {
        let mut script = self.get_task(&TaskAction::ProcessAlert).unwrap();
        script.fill_alert(&alert).unwrap();
        execute_script(&script).await
    }

    // 获取指定动作的脚本
    pub fn get_task(&self, action: &TaskAction) -> Option<ScriptRequest> {
        self.tasks.get(action).cloned()
    }

    // 添加新的任务脚本
    pub fn add_task(&mut self, action: TaskAction, script: ScriptRequest) {
        self.tasks.insert(action, script).unwrap();
    }

    // 移除任务脚本
    pub fn remove_task(&mut self, action: &TaskAction) -> Option<ScriptRequest> {
        self.tasks.remove(action)
    }

    // 获取所有可用的任务列表
    pub fn list_tasks(&self) -> Vec<&TaskAction> {
        self.tasks.keys().collect()
    }
}
