# Architecture Decisions

> Record significant architectural decisions that are hard to discover from code alone.

## ADR-001: Three-Tier Memory Hierarchy

**Status**: Accepted

**Context**: Need to balance retrieval precision with token consumption for LLM context windows.

**Decision**: Adopt L0/L1/L2 three-tier architecture:
- **L0 Abstract** (~100 tokens): Quick relevance filtering
- **L1 Overview** (~2000 tokens): Context understanding
- **L2 Detail** (full): Complete original content

**Consequences**:
- Token efficiency: ~11× reduction vs naive retrieval
- Requires maintaining layer synchronization
- Search uses weighted scoring: `0.2×L0 + 0.3×L1 + 0.5×L2`

---

## ADR-002: Virtual Filesystem with cortex:// URI

**Status**: Accepted

**Context**: Need a unified, human-readable way to address memory resources across different storage dimensions.

**Decision**: Use `cortex://` URI scheme mapping to markdown files on disk.

**Consequences**:
- Human-readable, debuggable storage
- Easy backup and version control
- URI parsing overhead (mitigated by caching)
- Physical files in `{data_dir}/tenants/{tenant_id}/`

---

## ADR-003: Qdrant for Vector Storage

**Status**: Accepted

**Context**: Need efficient semantic search for memory retrieval.

**Decision**: Use Qdrant as the vector database.

**Consequences**:
- gRPC + HTTP API support
- Tenant isolation via collection naming: `{collection}_{tenant_id}`
- Requires Qdrant server dependency

---

## ADR-004: Multi-Tenancy via Directory Isolation

**Status**: Accepted

**Context**: Support multiple isolated memory spaces for different users/applications.

**Decision**: Tenant isolation at filesystem level:
```
{data_dir}/tenants/{tenant_id}/
├── session/
├── user/
├── agent/
└── resources/
```

**Consequences**:
- Simple backup/migration per tenant
- No cross-tenant data leakage
- Each tenant has separate Qdrant collection

---

## ADR-005: Incremental Memory Updates

**Status**: Accepted

**Context**: Full memory re-extraction on every file change is expensive.

**Decision**: Event-driven incremental updates:
- `IncrementalMemoryUpdater` handles delta changes
- `CascadeLayerUpdater` propagates changes to L0/L1
- `LlmResultCache` deduplicates LLM calls

**Consequences**:
- ~50-75% reduction in LLM API calls
- Complexity in event coordination
- Eventual consistency for layers

---

## Template for New Decisions

```markdown
## ADR-XXX: Title

**Status**: Proposed | Accepted | Deprecated | Superseded

**Context**: What is the issue we're addressing?

**Decision**: What is the change we're proposing/have made?

**Consequences**: What are the trade-offs and impacts?
```
