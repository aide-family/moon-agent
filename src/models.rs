use actix_web::Error;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ScriptRequest {
    pub script_type: ScriptType,
    pub content: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScriptType {
    Python2,
    Shell,
}

#[derive(Debug, Serialize)]
pub struct ScriptResponse {
    pub execution_id: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

// 告警状态枚举
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    Firing,
    Resolved,
}

// 告警明细
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlertItem {
    // 告警状态
    pub status: AlertStatus,
    // 标签
    pub labels: HashMap<String, String>,
    // 注解
    pub annotations: HashMap<String, String>,
    // 开始时间
    #[serde(rename = "startsAt")]
    pub starts_at: String,
    // 结束时间, 空表示未结束
    #[serde(rename = "endsAt")]
    pub ends_at: String,
    // 告警生成链接
    #[serde(rename = "generatorURL")]
    pub generator_url: String,
    // 指纹
    pub fingerprint: String,
    // value
    pub value: f64,
}

impl ScriptRequest {
    // 告警内容填充到脚本中
    pub fn fill_alert(&mut self, alert: &AlertItem) -> Result<ScriptRequest, Error> {
        let alert_json = serde_json::to_string(alert).expect("序列化告警数据失败");

        // 使用模板库完成数据填充
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("script", self.content.clone())
            .unwrap();
        let filled_script = handlebars
            .render("script", &alert_json)
            .expect("填充告警数据失败");
        let mut script_request = self.clone();
        script_request.content = filled_script;
        Ok(script_request)
    }
}

impl Default for AlertItem {
    fn default() -> Self {
        let mut labels = HashMap::new();
        labels.insert("service".to_string(), "api".to_string());
        labels.insert("severity".to_string(), "critical".to_string());

        let mut annotations = HashMap::new();
        annotations.insert("summary".to_string(), "API is down".to_string());
        annotations.insert(
            "description".to_string(),
            "API is not responding".to_string(),
        );

        AlertItem {
            status: AlertStatus::Firing,
            labels,
            annotations,
            starts_at: "2024-01-01T00:00:00Z".to_string(),
            ends_at: "".to_string(),
            generator_url: "https://example.com".to_string(),
            fingerprint: "test-alert-1".to_string(),
            value: 1.11,
        }
    }
}

// 测试fill_alert
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_alert() {
        let content = "echo {{alert.labels.service}}";
        let mut script = ScriptRequest {
            script_type: ScriptType::Shell,
            content: content.to_string(),
        };

        let alert = AlertItem::default();
        let filled_script = script.fill_alert(&alert);
        println!("{:?}", filled_script);
        // 断言
        assert_eq!(filled_script.unwrap().content, "echo api");
    }
}
