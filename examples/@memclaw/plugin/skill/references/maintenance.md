# Maintenance Guide

MemClaw automatically maintains data health through scheduled tasks and provides tools for manual intervention when needed.

## Automatic Maintenance

**The plugin automatically registers a Cron Job that runs every 3 hours**, executing:

| Command | Purpose |
|---------|---------|
| `vector prune` | Remove vectors whose source files no longer exist |
| `vector reindex` | Rebuild vector index and remove stale entries |
| `layers ensure-all` | Generate missing L0/L1 layer files |

No manual setup required. The job is registered when the plugin starts.

## Manual Maintenance Tool

Use `cortex_maintenance` tool to run maintenance on demand:

```json
// Full maintenance
{ "dryRun": false }

// Preview changes without executing
{ "dryRun": true }

// Run specific commands only
{ "commands": ["prune", "reindex"] }
```

**When to run manually:**
- Search results seem incomplete or stale
- After recovering from a crash or data corruption
- When disk space cleanup is needed

## Diagnostic Commands

### Check System Health

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw stats
```

### Check Layer Status

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw layers status
```

Shows how many directories have L0/L1 layers vs missing.

### Check Vector Index Status

```bash
cortex-mem-cli --config config.toml --tenant tenant_claw vector status
```

Shows total vectors and stale entries.

## Quick Fix Flow

1. **Search not working well?** → Check `layers status` and `vector status`
2. **Missing L0/L1 layers?** → Run `layers ensure-all`
3. **Stale vectors detected?** → Run `vector reindex`
4. **Still having issues?** → Run `vector prune`

## Troubleshooting

| Issue | Solution |
|-------|----------|
| CLI not found | Ensure `@memclaw/bin-{platform}` is installed |
| Connection refused | Check cortex-mem-service at `localhost:8085` |
| Qdrant issues | Verify Qdrant at `localhost:6333` |
| Layer generation fails | Check LLM API key in config.toml |