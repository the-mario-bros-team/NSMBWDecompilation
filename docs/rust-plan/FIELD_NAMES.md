# Where field names come from

Layout offsets, sizes and types come from the binary (constructors and the functions that use each field). Names need a source too, and only these count:

1. **A CC0 decomp of the same class.** Nintendo's framework classes are shared between games: `fBase_c`'s names come from the Skyward Sword decomp (zeldaret/ss, CC0, `include/f/f_base.h`), whose offsets match this game's.
2. **The game's own symbols:** function, static and class names from the symbol maps, e.g. `setKind` stores `kind`, and `preExecute` tests a field against the static `dActor_c::mExecStop`, so that field is `exec_stop_mask`.
3. **What the code does with it,** written in the layout's evidence and in the function's doc comment.

If none of those prove a name yet, the field is `unk_<offset>`. It gets a name when the function that uses it is typed and gated.

**Never** take field names, layouts or comments from the community NSMBW-Decomp (no license) or from any source whose license is unknown, even where they would fit. On 2026-11-27, 96 of the actor layouts' field names that had come from there were replaced this way. Git history still shows the old names, but the current tree doesn't use them.
