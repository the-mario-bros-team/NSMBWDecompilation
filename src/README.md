# Source map

Read `src/lib.rs` first; it is the table of contents.

| Module | State | Start here |
|---|---|---|
| `boot/` | active | `boot/entry.rs` |
| `runtime/` | active | `runtime/os.rs`, `runtime/init.rs` |
| `engine/` | active | `engine/startup.rs` |
| `render/` | semantic home, empty | `render/mod.rs` |
| `audio/` | semantic home, empty | `audio/mod.rs` |
| `ui/` | semantic home, empty | `ui/mod.rs` |
| `game/` | semantic home, empty | `game/mod.rs` |
| `subsystems/r*.rs` | extraction staging | `docs/COVERAGE_*` |
| `platform.rs` | memory/call primitives | read the module header |

## Rules

- New translations go into a **semantic** module, not a range module.
- Range modules shrink as functions are promoted; see `docs/ORGANIZATION.md`.
- Every public function needs a `///` doc with `# Original` and `# Confidence`.
- `node tools/doc-lint.mjs src` must pass before merging a lane.

Addresses are virtual (image base `0x80004000`), never file offsets.
