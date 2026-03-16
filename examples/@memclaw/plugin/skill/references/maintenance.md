# CLI Maintenance Guide

Periodic maintenance and optimization commands for MemClaw.

## Automated Maintenance with Cron Jobs (Recommended)

OpenClaw has built-in Cron Job support that can execute scheduled tasks using the AI agent. **You must create a Cron Job to automatically maintain MemClaw data every 3 hours.**

### CLI Installation and Location

The `cortex-mem-cli` tool is distributed via platform-specific npm packages:

| Platform | npm Package |
|----------|-------------|
| macOS Apple Silicon | `@memclaw/bin-darwin-arm64` |
| Windows x64 | `@memclaw/bin-win-x64` |

**If not already installed, install the CLI package:**

```bash
# macOS Apple Silicon
npm install -g @memclaw/bin-darwin-arm64

# Windows x64
npm install -g @memclaw/bin-win-x64
```

**CLI Binary Location:**

After installation, the CLI binary is located at:

| Platform | Binary Path |
|----------|-------------|
| macOS (global) | `{npm-global-prefix}/node_modules/@memclaw/bin-darwin-arm64/bin/cortex-mem-cli` |
| Windows (global) | `{npm-global-prefix}/node_modules/@memclaw/bin-win-x64/bin/cortex-mem-cli.exe` |
| Local install | `{project}/node_modules/@memclaw/bin-{platform}/bin/cortex-mem-cli` |

To find the npm global prefix:
```bash
npm root -g
# Returns: /usr/local/lib/node_modules (macOS) or similar
```

**Verify CLI is accessible:**
```bash
which cortex-mem-cli
# Or test directly:
cortex-mem-cli --help
```

### How to Create the Maintenance Cron Job

**Ask the user to set up the following Cron Job, or guide them through OpenClaw's Cron configuration:**

**Schedule**: Every 3 hours

**Commands to execute (use full paths):**

```bash
# macOS Example (adjust paths to your installation):
/path/to/cortex-mem-cli --config ~/Library/Application\ Support/memclaw/config.toml --tenant tenant_claw vector prune

/path/to/cortex-mem-cli --config ~/Library/Application\ Support/memclaw/config.toml --tenant tenant_claw vector reindex

/path/to/cortex-mem-cli --config ~/Library/Application\ Support/memclaw/config.toml --tenant tenant_claw layers ensure-all

# Windows Example (adjust paths to your installation):
cortex-mem-cli.exe --config %LOCALAPPDATA%\memclaw\config.toml --tenant tenant_claw vector prune

cortex-mem-cli.exe --config %LOCALAPPDATA%\memclaw\config.toml --tenant tenant_claw vector reindex

cortex-mem-cli.exe --config %LOCALAPPDATA%\memclaw\config.toml --tenant tenant_claw layers ensure-all
```

**Steps to create the Cron Job in OpenClaw:**

1. Open OpenClaw configuration or use the Cron management interface
2. Create a new Cron Job with:
   - **Name**: `MemClaw Maintenance`
   - **Schedule**: `0 */3 * * *` (every 3 hours)
   - **Task prompt**: "Execute MemClaw maintenance. First, locate the cortex-mem-cli binary (installed from @memclaw/bin-darwin-arm64 or @memclaw/bin-win-x64 npm package). Then execute these commands in order: (1) `cortex-mem-cli --config <config-path> --tenant tenant_claw vector prune`, (2) `cortex-mem-cli --config <config-path> --tenant tenant_claw vector reindex`, (3) `cortex-mem-cli --config <config-path> --tenant tenant_claw layers ensure-all`. Use the platform-specific config.toml path (macOS: ~/Library/Application Support/memclaw/config.toml, Windows: %LOCALAPPDATA%\memclaw\config.toml). Report any errors encountered."
   - **Agent**: Select the agent that has MemClaw plugin enabled

### What This Maintenance Does

| Command | Purpose |
|---------|---------|
| `vector prune` | Remove vectors whose source files no longer exist |
| `vector reindex` | Rebuild vector index and remove stale entries |
| `layers ensure-all` | Generate missing L0/L1 layer files |

This ensures:
- Vector database stays clean and efficient
- Missing layer files are regenerated
- Search results remain accurate and relevant

> **Note**: These commands require `--config` pointing to your config.toml. The agent should use the platform-specific path (see setup.md for details).

## Overview

The `cortex-mem-cli` is designed for **periodic maintenance and optimization**. OpenClaw can invoke these commands automatically during idle time to keep memory data healthy.

> **Note**: These are maintenance tools. For daily operations, use cortex_search, cortex_recall, etc.

## When to Run Maintenance

OpenClaw should periodically run maintenance when:
- Idle for extended periods (e.g., 30+ minutes)
- After many sessions have been closed
- When search results seem incomplete or stale

## Diagnostic Commands

### Check System Health

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw stats
```

Shows:
- Number of sessions
- Total messages
- Storage location

### Check Layer File Status

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw layers status
```

Shows:
- Total directories tracked
- How many have L0/L1 layers (complete)
- How many are missing L0/L1 layers

**Run this when:**
- Search results lack proper context
- After bulk data import

### Check Vector Index Status

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw vector status
```

Shows:
- Total vectors in Qdrant
- Number of stale vectors (missing URI metadata)

**Run this when:**
- Search returns inconsistent results
- After system crash or restart

## Repair Commands

### Generate Missing L0/L1 Layers

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw layers ensure-all
```

Scans filesystem and generates `.abstract.md` and `.overview.md` files for directories that lack them.

**Run this when:**
- `layers status` shows missing directories
- Search results lack proper context/snippets
- After manual data recovery

**What it does:**
1. Scans all session directories
2. Identifies directories without L0/L1 files
3. Uses LLM to generate abstracts and overviews
4. Saves generated files

### Rebuild Vector Index

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw vector reindex
```

Cleans up stale vectors (no URI) and re-syncs all files to the vector database.

**Run this when:**
- `vector status` shows stale vectors
- Search returns inconsistent results
- After manual data recovery
- Vector database corruption suspected

**What it does:**
1. Removes vectors without URI metadata
2. Re-scans all files
3. Generates new embeddings
4. Syncs to Qdrant

### Prune Dangling Vectors

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw vector prune
```

Removes vectors whose source files no longer exist on disk.

**Run this when:**
- After deleting session files
- Disk space is a concern
- Index cleanup is needed

**Preview mode (recommended first):**
```bash
cortex-mem-cli --config config.toml --tenant tenant_claw vector prune --dry-run
```

Shows what would be deleted without making changes.

### Regenerate Oversized Abstracts

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw layers regenerate-oversized
```

Regenerates `.abstract.md` files that exceed the size limit.

**Run this when:**
- Abstract files have grown too large
- Search performance is degraded

## Recommended Maintenance Schedule

| Frequency | Command | Purpose |
|-----------|---------|---------|
| Daily | `stats` | Quick health check |
| Weekly | `layers status` + `vector status` | Detect anomalies early |
| As needed | `layers ensure-all` | Fix missing layers |
| As needed | `vector reindex` | Fix index corruption |
| Monthly | `vector prune` | Clean up dangling data |

## Quick Fix Flow

1. **Search not working well?**
   ```bash
   cortex-mem-cli --config config.toml --tenant tenant_claw layers status
   cortex-mem-cli --config config.toml --tenant tenant_claw vector status
   ```

2. **Missing L0/L1 layers detected?**
   ```bash
   cortex-mem-cli --config config.toml --tenant tenant_claw layers ensure-all
   ```

3. **Stale vectors detected?**
   ```bash
   cortex-mem-cli --config config.toml --tenant tenant_claw vector reindex
   ```

4. **Still having issues?**
   ```bash
   cortex-mem-cli --config config.toml --tenant tenant_claw vector prune
   ```

## Troubleshooting

### CLI Not Found

Ensure `cortex-mem-cli` is in your PATH or use the full path:
```bash
/path/to/cortex-mem-cli --config config.toml ...
```

### Connection Refused

Check that cortex-mem-service is running:
```bash
curl http://localhost:8085/health
```

### Qdrant Connection Issues

Verify Qdrant is accessible:
```bash
curl http://localhost:6333/collections
```

### Layer Generation Fails

1. Check LLM API key in config.toml
2. Verify network connectivity to API endpoint
3. Check for rate limiting or quota issues
