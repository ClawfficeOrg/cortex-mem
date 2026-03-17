# 故障排查指南

MemClaw 常见问题及解决方案。

## 安装问题

### 平台不支持

**症状**：显示 "Platform not supported" 错误

**解决方案**：
- 确认您使用的是 macOS Apple Silicon (M1/M2/M3) 或 Windows x64
- 其他平台暂不支持

### 插件安装失败

**症状**：`openclaw plugins install @memclaw/memclaw` 失败

**解决方案**：
1. 检查网络连接
2. 确认 npm 仓库可访问
3. 尝试使用代理或镜像源

## 配置问题

### API 密钥无效

**症状**：搜索或记忆操作返回 API 错误

**解决方案**：
1. 验证 `llmApiKey` 和 `embeddingApiKey` 在 OpenClaw 插件设置中已正确配置
2. 确认 API 密钥有效且有足够额度
3. 确认 `llmApiBaseUrl` 和 `embeddingApiBaseUrl` 对于您的提供商是正确的
4. 验证到 API 端点的网络连接

### 配置未生效

**症状**：修改配置后服务行为未改变

**解决方案**：
1. 确保保存了配置文件
2. 重启 OpenClaw 以应用更改
3. 检查配置文件语法是否正确（JSON 格式）

## 服务问题

### 服务无法启动

**症状**：插件加载时服务启动失败

**解决方案**：
1. 检查端口 6333、6334、8085 是否被其他应用占用
2. 确认 API 密钥已在 OpenClaw 插件设置中配置
3. 查看 OpenClaw 日志获取详细错误信息

### 服务不可达

**症状**：工具调用返回连接错误

**解决方案**：
1. 确认 OpenClaw 已重启且插件已加载
2. 检查 `autoStartServices` 配置项是否为 `true`（默认）
3. 验证防火墙允许这些端口的本地连接

## 使用问题

### 搜索无结果

**症状**：`cortex_search` 返回空结果

**解决方案**：
1. 运行 `cortex_list_sessions` 验证会话是否存在
2. 降低 `min_score` 阈值（例如从 0.6 降到 0.4）
3. 尝试不同的查询词或同义词
4. 确认之前已调用 `cortex_add_memory` 或 `cortex_close_session` 存储记忆

### 记忆提取失败

**症状**：`cortex_close_session` 执行失败或结果不完整

**解决方案**：
1. 验证 LLM API 配置正确
2. 检查 API 额度是否充足
3. 查看 OpenClaw 日志获取详细错误信息

### 迁移失败

**症状**：`cortex_migrate` 执行失败

**解决方案**：
1. 确认 OpenClaw 工作区位于 `~/.openclaw/workspace`
2. 确认记忆文件存在于 `~/.openclaw/workspace/memory/`
3. 验证文件权限正确

## 数据问题

### 数据位置

MemClaw 数据存储位置：

| 平台 | 路径 |
|------|------|
| macOS | `~/Library/Application Support/memclaw` |
| Windows | `%LOCALAPPDATA%\memclaw` |
| Linux | `~/.local/share/memclaw` |

### 数据安全

- **备份**：迁移前，现有 OpenClaw 记忆文件会被保留
- **本地存储**：所有记忆数据存储在本地
- **无云同步**：数据保留在本地机器

## 错误信息参考

| 错误信息 | 可能原因 | 解决方案 |
|----------|----------|----------|
| `Service not running` | 服务未启动 | 重启 OpenClaw 或启用 `autoStartServices` |
| `API error: 401` | API 密钥无效 | 检查 API 密钥配置 |
| `API error: 429` | 请求频率超限 | 等待后重试或升级 API 套餐 |
| `Connection refused` | 服务不可达 | 检查端口占用和服务状态 |
| `No sessions found` | 无记忆数据 | 使用 `cortex_add_memory` 添加记忆 |

## 获取帮助

如果以上解决方案未能解决问题：

1. 查看 OpenClaw 日志获取详细错误信息
2. 在 [GitHub Issues](https://github.com/sopaco/cortex-mem/issues) 提交问题报告
3. 提供以下信息：
   - 操作系统和版本
   - OpenClaw 版本
   - MemClaw 插件版本
   - 相关日志片段
   - 重现步骤
