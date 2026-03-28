# Decisions

## 2026-03-27 - Rename Repository To `rust-practical-lab`

### Context

The repository had grown beyond a literal hello-world starter. The existing README, docs, and workspace layout show a practical Rust lab with multiple phases, interop demos, and reusable production-oriented building blocks. Nick explicitly approved a rename on 2026-03-27 to reflect that broader practical value.

### Decision

Rename the repository identity from `rust-hello-world` to `rust-practical-lab` and add a minimal repo memory stack (`AGENTS.md`, repo brief, heartbeat, decisions, dated session log) without changing the underlying workspace design.

### Why

- The old name understated the repo's actual utility
- The new name matches the current README and crate layout more closely
- A small memory stack reduces restart cost for maintainers and coding agents
- A tight identity pass avoids destabilizing working code and docs

### Impact

- README, agent guidance, repo-memory docs, and repository URLs use the new name
- Future session work has a clear read order and operational snapshot
- No crate names, workspace members, or behavior changed as part of the rename

### Revisit When

Revisit if the repository stops functioning as a practical lab workspace, or if a future packaging/release strategy needs a different public identity.
