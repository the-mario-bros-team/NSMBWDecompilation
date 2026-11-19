# Toolchain

## Host

- Windows (win32 x64), no admin rights in this session
- Node 24.22.2 (portable, `%LOCALAPPDATA%\nodejs-24.22.2`, first in user PATH)
- System Node 24.18.0 remains in machine PATH; OpenCode's `rea` server uses a
  scoped `environment.PATH` override to get 24.22.2
- Java 17 installed (`C:\Program Files\...\jdk-17`), not sufficient for Ghidra 12.x
- Python absent, no 7z/wit; tooling is Node-based

## REA

- `rea-agents@6.8.0` via npx; MCP registration in
  `%USERPROFILE%\.config\opencode\opencode.json`
- REA rejects Wii DOL/U8 directly (`unsupported binary format`)
- REA accepts the converted `main.elf`: `format: elf`, `architecture: powerpc`
- Before Ghidra: `analysis_provider_binding: null` (no native provider)

## Ghidra (installed)

- **Ghidra 12.1.4** at `%LOCALAPPDATA%\tools\ghidra_12.1.4_PUBLIC`
  (sha256 `ddac49f903da9d5bac833e5cc79395098b9c33cfd3279be5f31bd00387d2d4db`)
- **JDK 21.0.13+9** at `%LOCALAPPDATA%\tools\jdk-21.0.13+9`
  (sha256 `a4dbaa05fca4bd89d84584e58c459bd732efebb1722af80ab13c6516832004f9`)
- REA sees Ghidra (`doctor` ghidra check true: version 12.1.4, JDK 21 accepted)
- REA still cannot use it for PowerPC: Windows Ghidra P0 accepts native PE and
  raw 6502 only. Standalone headless is the analysis path.
- Imported `main.elf` as `PowerPC:BE:32` (forced `default`; Broadway is 750CL,
  not `e500`), image base `0x80004000`, 15 memory blocks

## Ghidra headless

```powershell
$env:JAVA_HOME="$env:LOCALAPPDATA\tools\jdk-21.0.13+9"
& "$env:LOCALAPPDATA\tools\ghidra_12.1.4_PUBLIC\support\analyzeHeadless.bat" `
  work\ghidra\<projdir> nsmbw -import work\dol\main.elf `
  -processor PowerPC:BE:32:Gekko_Broadway -analysisTimeoutPerFile 7200 `
  -scriptPath tools\ghidra -postScript ExportDecomp.java <outdir>
```

## Gekko/Broadway language

- Installed **Ghidra-GameCube-Loader 1.3.1** (Cuyler36) into
  `Ghidra\Extensions\GameCubeLoader`; language `PowerPC:BE:32:Gekko_Broadway`.
- Stock Ghidra PPC (default/e500) does not decode paired-single: 549/9,518
  functions (5.8%) contained `halt_baddata`.
- Gekko_Broadway pass: **10,276 functions, 0 bad-instruction files**
  (`work/ghidra/main-gb/export`).
- Caveat: the extension's **DOL/REL loader** opens a Swing symbol-map chooser
  even under `analyzeHeadless` (blocks headless import). Use the ELF import +
  `-processor` path; do not use its loader headless.
- Caveat: small-data register `r13` is not established by the ELF import, so
  some globals decompile as `unaff_r13`. Documented, not silently corrected.
- The extension's CodeWarrior demangler analyzer throws a null-demangler NPE in
  headless; analysis still completes.



## Rust

- rustup **1.99.1** (`x86_64-pc-windows-msvc`, minimal profile), user-scope:
  `%USERPROFILE%\.cargo\bin` (added to user PATH)
- `cargo check` works without a linker; producing Windows binaries later needs
  Visual Studio Build Tools (`link.exe`) or a GNU target
- Crate: `Cargo.toml` at workspace root, `#![no_std]` lib in `src/lib.rs`

## Local tools

| Tool | Purpose |
|---|---|
| `tools/lz11.mjs` | Nintendo LZ11 decompression (RELs verified) |
| `tools/u8.mjs` | U8 list/index/extract |
| `tools/index-arcs.mjs` | Bulk U8 inventory |
| `tools/dol2elf.mjs` | DOL → ELF32 BE EM_PPC (BSS split for NSMBW) |
| `tools/rel2elf.mjs` | REL → ELF (pending, Stage 1) |

## Commands

```powershell
$env:PATH="$env:LOCALAPPDATA\nodejs-24.22.2;"+$env:PATH
node tools/lz11.mjs <in.LZ> <out>
node tools/u8.mjs <a.arc> [--json] [--extract <dir>]
node tools/index-arcs.mjs <files-root> <out.json>
node tools/dol2elf.mjs <main.dol> <out.elf>
```
