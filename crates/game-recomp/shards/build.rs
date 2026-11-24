//! Shared build script of the shard crates: without generated code
//! (`ppc-recomp` not run yet, or not for every version), writes an empty
//! `gen/sNN.rs` and `gen/<version>/sNN.rs` so the port still builds.

/// The versions other than USA revision 1, whose code is in `gen/<name>/`.
const VERSIONS: [&str; 7] = ["e2", "p1", "p2", "j1", "j2", "k", "w"];

fn main() {
    let shard = std::env::var("CARGO_PKG_NAME").unwrap().trim_start_matches("recomp-").to_string();
    let gen = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../gen");
    for dir in std::iter::once(gen.clone()).chain(VERSIONS.iter().map(|v| gen.join(v))) {
        let file = dir.join(format!("{shard}.rs"));
        if !file.exists() {
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(&file, "// Empty: run ppc-recomp to generate recompiled code.\n").unwrap();
        }
        println!("cargo:rerun-if-changed={}", file.display());
    }
}
