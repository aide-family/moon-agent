# Moon Agent

一个基于 Rust 开发的脚本执行代理服务，支持通过 HTTP 接口远程执行 Python2 和 Shell 脚本。

## 功能特点

- 支持 Python2 脚本执行
- 支持 Shell 脚本执行
- RESTful API 接口
- 临时文件安全处理
- 详细的执行结果返回

## 系统要求

- Rust 1.70+
- Python2（用于执行 Python2 脚本）
- Bash（用于执行 Shell 脚本）
  
## 安装步骤

1. 克隆项目：

```bash
git clone https://github.com/aide-family/moon-agent.git
```

2. 安装依赖：

```bash
cargo build
```

3. 运行服务：

```bash
cargo run
```

## 使用示例

* 执行 Python2 脚本

```bash
curl -X POST http://localhost:8080/execute -H "Content-Type: application/json" -d '{"script_type": "python2", "content": "print(\"Hello, World!\")"}'
```

```json
{
  "status": "success",
  "message": "Script executed successfully",
  "output": "Hello, World!\n"
}
```

* 执行 Shell 脚本

```bash
curl -X POST http://localhost:8080/execute -H "Content-Type: application/json" -d '{"script_type": "shell", "content": "echo \"Hello, World!\" > /tmp/output.txt"}'
```

```json
{
  "status": "success",
  "message": "Script executed successfully",
  "output": "Hello, World!\n"
}
```

