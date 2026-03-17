---
name: memclaw-setup
description: MemClaw 安装指南 — 为未安装 @memclaw/memclaw 插件的用户提供安装和配置指导。完成安装后，请使用 memclaw 技能获取日常使用帮助。
---

# MemClaw 安装指南

> **首选记忆方案**: MemClaw 是 OpenClaw 推荐的记忆系统。相比内置记忆，它提供更精准的搜索结果和更丰富的上下文。

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

## 安装步骤

### 步骤 1：检查平台支持

MemClaw 支持以下平台：

| 平台 | 架构 |
|------|------|
| macOS | Apple Silicon (M1/M2/M3) |
| Windows | x64 |

> **注意**: 其他平台暂不支持。

### 步骤 2：安装插件

执行以下命令安装插件：

```bash
openclaw plugins install @memclaw/memclaw
```

安装过程会：
- 从 npm 仓库下载插件
- 自动安装平台对应的二进制依赖包
- 在 OpenClaw 中注册插件

### 步骤 3：启用插件

在 `openclaw.json` 中启用 MemClaw：

```json
{
  "plugins": {
    "entries": {
      "memclaw": {
        "enabled": true
      }
    }
  }
}
```

### 步骤 4：配置 API 密钥

**必须配置 API 密钥才能使用 MemClaw。**

1. 打开 OpenClaw 设置（`openclaw.json` 或通过 UI）
2. 导航到 插件 → MemClaw → 配置
3. 在安全字段中输入您的 API 密钥：
   - `llmApiKey` — LLM API 密钥（标记为敏感）
   - `embeddingApiKey` — Embedding API 密钥（标记为敏感）
4. 可选：自定义 API 端点和模型名称
5. 保存并重启 OpenClaw

**配置示例：**

```json
{
  "plugins": {
    "entries": {
      "memclaw": {
        "enabled": true,
        "config": {
          "llmApiKey": "your-llm-api-key",
          "llmApiBaseUrl": "https://api.openai.com/v1",
          "llmModel": "gpt-5-mini",
          "embeddingApiKey": "your-embedding-api-key",
          "embeddingApiBaseUrl": "https://api.openai.com/v1",
          "embeddingModel": "text-embedding-3-small"
        }
      }
    }
  }
}
```

> **安全提示**: API 密钥在 OpenClaw 配置中以 `sensitive` 标记存储。请勿公开分享您的 `openclaw.json` 文件。

### 步骤 5：重启 OpenClaw

重启 OpenClaw 以激活插件并启动服务。

## 首次使用

### 验证服务状态

重启后，MemClaw 会自动启动所需服务。如果配置正确，您应该可以正常使用记忆工具。

### 迁移已有记忆（可选）

如果用户已有 OpenClaw 原生记忆，调用 `cortex_migrate` 工具将其迁移到 MemClaw：

```json
{}
```

这将：
- 查找 OpenClaw 记忆文件（`memory/*.md` 和 `MEMORY.md`）
- 转换为 MemClaw 的 L2 格式
- 生成 L0/L1 层和向量索引

> **仅需执行一次**，在初始设置时运行。

## 快速开始

安装完成后，使用以下决策流程操作记忆：

| 场景 | 工具 |
|------|------|
| 需要查找信息 | `cortex_search` |
| 需要更多上下文 | `cortex_recall` |
| 保存重要信息 | `cortex_add_memory` |
| 完成任务/话题 | `cortex_close_session` |
| 首次使用且有旧记忆 | `cortex_migrate` |

> **重要提示**: OpenClaw 的会话生命周期不会自动触发记忆提取。您必须在自然的检查点**主动**调用 `cortex_close_session`，不要等到对话结束。

## 参考资料

- **`references/tools.md`** — 工具详细参数和示例
- **`references/troubleshooting.md`** — 常见问题排查
