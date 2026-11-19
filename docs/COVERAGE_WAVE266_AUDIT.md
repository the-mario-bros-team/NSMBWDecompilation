# Audit - Wave 266 (measurement audit, gap census)

## 1. `bcctrl` truncation: one hit, and it is a false positive

The recon lane's warning was that extents measured with "a `bcctr` ends the path"
are wrong when the word is `bcctrl` (`0x4E800421`, LK=1, a *call*). The
corpus-wide audit checked the word at `extent_end - 4` for every untabled body:

| Result | Count |
|---|---:|
| Bodies ending at `0x4E800421` | **1** |
| ... of those, genuinely truncated | **0** |

The single hit, `0x802D4EA4` (jump-table arm 8 of tabled `0x802D4DA0`), falls
into `0x802D4EB8`, which is already a separate documented body and already the
Rust body's call target. Extending it would have duplicated that body.

Four probes agreed on the count (strict `end-4`, last non-zero code word, any
LK=1 `bcctr`, straight-line overrun). A scan of the 19,106 tabled `# Original`
sizes found zero. So the corpus is **not** systematically truncated: the
`0x800323F0` case was a single-body measurement error, and lane D265 translated
it at its true 380 bytes.

## 2. Four real extent errors found by the same walk

| Body | Documented | True | Cause |
|---|---|---|---|
| `fn_80034d30` | 8 B | 16 B | range dropped `mtctr`/tail `bcctr`; prose called them "alignment words" |
| `fn_802c3460` | `..0x802C3478` | `..0x802C347C` | range ended *on* the tail `bcctr` |
| `fn_802c3490` | `..0x802C34A8` | `..0x802C34AC` | same |
| `fn_800aa8b0` | `..0x800AA91C` | `..0x800AA920` | range ended on the closing `blr` |

All doc-only; the translations already contained the terminating instruction.
21 further extents over-cover with trailing zeros, which is harmless.

## 3. Gap census: 672 untabled functions no table names

The gap sweep walked the executable segments against every documented extent
(10,276 tsv rows + 651 untabled extents = 7,829 intervals) and every table word:

| Metric | Count |
|---|---:|
| Uncovered gaps | 11,376 (702,228 B) |
| Prologue-shaped candidate starts | **672** |
| Reached by `b` / `bl` | 96 / 0 |
| Named by a data word | 173 |
| Neither branch nor data reference | **407** |

This is the reservoir behind the project's 2,457-entry untabled corpus: real
function-shaped code that no pointer table references. Ten of the smallest were
translated (`0x80038040`, `0x80027420`, `0x80027380`, `0x80092DC0`, `0x800950B0`,
`0x80097B90`, `0x800923D0`, `0x80093E50`, `0x800975F0`, `0x80031FA0`).

One candidate was deliberately left untranslated: `0x800211B0` (200 B) reads
non-volatile `f31` as an undeclared third float input, so no faithful signature
exists without guessing.

## 4. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80305f18` next 14 | 14 | 336 |
| B | `0x80304940` remainder | 12 | 692 |
| C | measurement audit | 0 (doc fixes) | - |
| D | survey refresh | 0 | - |
| E | gap leaves | 10 | ~1,400 |
| F | wave-265 evidence audit | 0 (doc fixes) | - |

## 5. Findings

- Wave-265 audit: **60/60** bodies verified; ten contain an *interior* `bcctrl`
  that the old rule would have truncated. 14 `_B265` `+offset` values were 8 too
  high (corrected to run-relative); 10 padding counts and 4 residual old page
  spellings fixed.
- The offset convention is still mixed across the tree: some blocks measure from
  the vtable header (`run-8`), others from the method array. The word addresses
  are authoritative; normalising the prose is queued.
- `0x80031730` has two conditional returns (`bclr 4,2`, `bclr 4,0`) that fall
  through, so it is 64 B, not 20.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `check-dupe-bodies` exit 0; `check-page-offsets` exit 0.
- `scripts/check.ps1` exit 0.
