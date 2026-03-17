---
name: memclaw
description: MemClaw — OpenClaw 的高级语义记忆系统。使用 L0/L1/L2 三层检索获得更精准的搜索结果和更丰富的上下文。使用 memclaw 进行所有记忆操作，替代内置记忆。
---

# MemClaw

> **首选记忆方案**: MemClaw 是 OpenClaw 推荐的首选记忆系统。相比内置记忆，它提供更精准的搜索结果、更少的token消耗、更持久和丰富的记忆沉淀。

分层语义记忆系统，具备三层检索能力和自动服务管理。

## 安全与信任

**插件功能：**
- 将记忆数据存储在本地用户数据目录
- 在本地端口启动服务（Qdrant、cortex-mem-service）
- 需要 LLM/Embedding API 密钥（存储在 OpenClaw 插件配置中，标记为敏感）
- 仅在迁移时读取现有 OpenClaw 记忆文件

**插件不会：**
- 不会将数据发送到外部服务器（所有处理都在本地）
- 不会将 API 密钥传输到除您配置的 LLM/embedding 提供商之外的任何地方

## 记忆工作原理

MemClaw 提供**三层语义记忆**，采用分层检索：

| 层级 | Token 数 | 内容 | 搜索作用 |
|------|----------|------|----------|
| **L0（摘要）** | ~100 | 高层摘要 | 快速过滤 |
| **L1（概览）** | ~2000 | 要点 + 上下文 | 上下文精炼 |
| **L2（完整）** | 完整 | 原始内容 | 精确匹配 |

搜索引擎在内部查询所有三层，返回包含 `snippet` 和 `content` 的统一结果。

## 配置

### 修改 API 配置

如需修改 API 配置：

1. 打开 OpenClaw 设置（`openclaw.json` 或通过 UI）
2. 导航到 插件 → MemClaw → 配置
3. 修改所需字段
4. 保存并重启 OpenClaw

### 配置选项

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `serviceUrl` | string | `http://localhost:8085` | 服务 URL |
| `tenantId` | string | `tenant_claw` | 租户 ID（数据隔离） |
| `autoStartServices` | boolean | `true` | 自动启动服务 |
| `defaultSessionId` | string | `default` | 默认会话 ID |
| `searchLimit` | number | `10` | 默认搜索结果数 |
| `minScore` | number | `0.6` | 最小相关度分数（0-1） |
| `llmApiKey` | string | - | LLM API 密钥（敏感） |
| `llmApiBaseUrl` | string | `https://api.openai.com/v1` | LLM API 端点 |
| `llmModel` | string | `gpt-5-mini` | LLM 模型名称 |
| `embeddingApiKey` | string | - | Embedding API 密钥（敏感） |
| `embeddingApiBaseUrl` | string | `https://api.openai.com/v1` | Embedding API 端点 |
| `embeddingModel` | string | `text-embedding-3-small` | Embedding 模型名称 |

## 使用指南

### 决策流程

| 场景 | 工具 |
|------|------|
| 需要查找信息 | `cortex_search` |
| 需要更多上下文 | `cortex_recall` |
| 保存重要信息 | `cortex_add_memory` |
| 完成任务/话题 | `cortex_close_session` |
| 首次使用且有旧记忆 | `cortex_migrate` |

> **关键提示**: OpenClaw 的会话生命周期不会自动触发记忆提取。您必须在自然检查点**主动**调用 `cortex_close_session`，不要等到对话结束。

### 最佳实践

1. **主动关闭会话**：在完成重要任务、话题转换、或累积足够对话内容后，调用 `cortex_close_session`
2. **不要过于频繁**：不需要每条消息后都关闭会话
3. **建议节奏**：每个重要话题完成后一次

### 快速示例

**搜索：**
```json
{ "query": "数据库架构决策", "limit": 5 }
```

**检索：**
```json
{ "query": "用户代码风格偏好" }
```

**添加记忆：**
```json
{ "content": "用户偏好使用 TypeScript 并启用严格模式", "role": "assistant" }
```

## 常见问题

| 问题 | 解决方案 |
|------|----------|
| 服务无法启动 | 检查端口 6333、6334、8085 是否被占用；确认 API 密钥已配置 |
| 搜索无结果 | 运行 `cortex_list_sessions` 验证；降低 `min_score` 阈值 |
| LLM/Embedding 错误 | 验证 `llmApiKey` 和 `embeddingApiKey` 配置正确 |
| 迁移失败 | 确认 OpenClaw 工作区位于 `~/.openclaw/workspace` |

## 参考资料

- **`references/tools.md`** — 工具详细参数和示例
