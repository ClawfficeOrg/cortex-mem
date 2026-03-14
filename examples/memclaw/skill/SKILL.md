---
name: memclaw
description: MemClaw — Layered semantic memory for OpenClaw. Use this skill to set up, configure, and use MemClaw for storing, searching, and recalling memories with L0/L1/L2 tiered retrieval.
---

# MemClaw

Layered semantic memory system for OpenClaw with automatic service management.

## How Memory Works

MemClaw provides **three-layer semantic memory** with tiered retrieval:

| Layer | Tokens | Content | Role in Search |
|-------|--------|---------|----------------|
| **L0 (Abstract)** | ~100 | High-level summary | Quick filtering |
| **L1 (Overview)** | ~2000 | Key points + context | Context refinement |
| **L2 (Full)** | Complete | Original content | Precise matching |

The search engine queries all three layers internally and returns unified results with `snippet` and `content`.

---

## Setup

### Requirements

| Requirement | Details |
|-------------|---------|
| **Platforms** | Windows x64, macOS Apple Silicon |
| **Node.js** | ≥ 22.0.0 |

### Install

```bash
openclaw plugins install memclaw
```

### Configure openclaw.json

```json
{
  "plugins": {
    "entries": {
      "memclaw": { "enabled": true }
    }
  },
  "agents": {
    "defaults": {
      "memorySearch": { "enabled": false }
    }
  }
}
```

> Set `memorySearch.enabled: false` to disable OpenClaw's built-in memory search.

### Configure LLM

On first run, MemClaw creates `config.toml`:

| Platform | Path |
|----------|------|
| Windows | `%APPDATA%\memclaw\config.toml` |
| macOS | `~/Library/Application Support/memclaw/config.toml` |

Fill in required fields:

```toml
[llm]
api_key = "xxx"  # REQUIRED

[embedding]
api_key = "xxx"  # REQUIRED (can be same as llm.api_key)
```

Then restart OpenClaw.

---

## Tools

### cortex_search

Semantic search across all memories.

```json
{ "query": "database architecture decisions", "limit": 5 }
```

### cortex_recall

Recall with more context (snippet + content).

```json
{ "query": "user preferences for code style" }
```

### cortex_add_memory

Store a message for future retrieval.

```json
{ "content": "User prefers TypeScript with strict mode", "role": "assistant" }
```

### cortex_list_sessions

List all memory sessions.

### cortex_close_session

Close session and trigger memory extraction. Takes 30-60s.

### cortex_migrate

Migrate from OpenClaw native memory. Run once during setup.

---

## Quick Decision Flow

1. **Need to find something** → `cortex_search`
2. **Need more context** → `cortex_recall`
3. **Save something important** → `cortex_add_memory`
4. **Conversation complete** → `cortex_close_session`
5. **First time with existing memory** → `cortex_migrate`

---

## Troubleshooting

### Services Won't Start

1. Check ports 6333, 6334, 8085 are available
2. Verify `api_key` fields in config.toml

### Search Returns No Results

1. Run `cortex_list_sessions` to verify sessions exist
2. Lower `min_score` threshold (default: 0.6)

### Migration Fails

Ensure OpenClaw workspace exists at `~/.openclaw/workspace`

---

## CLI Reference

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw session list
cortex-mem-cli --config config.toml --tenant tenant_claw layers ensure-all
cortex-mem-cli --config config.toml --tenant tenant_claw vector reindex
```
