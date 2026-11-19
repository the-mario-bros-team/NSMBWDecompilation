# Organization model

## Why there are two source layers

Parallel lanes translate by **address range** because that is how the binary is
partitioned without symbols. That is efficient for extraction but hostile to a
reader: `r801c0_80200.rs` tells a newcomer nothing.

So the tree has two layers:

| Layer | Location | Purpose | Reader-facing? |
|---|---|---|---|
| Extraction | `src/subsystems/r*.rs` | parallel, address-partitioned translations | **no** — staging only |
| Semantic | `src/boot/`, `src/runtime/`, `src/render/`, `src/audio/`, `src/ui/`, `src/game/` | system-organized, documented | **yes** — start here |

Range modules are allowed to be incomplete. They are promoted into semantic
modules when their role is understood.

## Promotion checklist

A function moves from `src/subsystems/r*.rs` to a semantic module when:

1. Its subsystem is identified with evidence (string, call target, data structure).
2. It has a plain-English doc comment and an `# Original` evidence block.
3. Address-only names are replaced by descriptive names **or** the address name
   is documented as unresolved.
4. The ledger row's `module` and `rust_file` are updated.
5. The semantic module's `//!` header lists it under "Start here" or a call graph.
6. The range coverage doc marks it `promoted`.

No duplicate implementations: promotion moves the function, it does not copy it.

## Directory contract

| Path | Contains | Entry point |
|---|---|---|
| `src/boot/` | entry, CRT, memcpy/memset, cache, init tables | `entry.rs` |
| `src/runtime/os.rs` | OS init/scheduler primitives | `init.rs` |
| `src/runtime/` (future) | allocator, DVD, input, C++ runtime | per-file header |
| `src/render/` | GX init/state translation | `gx.rs` |
| `src/audio/` | AX/JAS/BRSAR | `ax.rs` |
| `src/ui/` | NW4R lyt/blyt | `lyt.rs` |
| `src/game/` | actor, stage, player, enemies | `actor.rs` / `stage.rs` |
| `src/subsystems/` | extraction ranges | `docs/COVERAGE_*` |

## Definition of done

A module is readable when a person with no context can answer, from the file
alone: what system is this, where does execution enter, what data does it own,
which addresses are original, and what is still unknown.

A function is readable when its doc comment explains behavior in one sentence
before any code, and its `# Original` block lets the reader jump to the Ghidra C.

## Enforcement

```powershell
node tools/doc-lint.mjs src
```

The linter fails on: missing module header, missing function doc, missing
`# Original`, missing `# Confidence`/`# Safety`, and bare `// TODO`.

Run it after every integration pass. A lane's output is not merged while it has
doc-lint violations.
