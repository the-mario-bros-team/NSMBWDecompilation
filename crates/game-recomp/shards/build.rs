//! Shared build script of the shard crates: without generated code
//! (`ppc-recomp` not run yet), writes an empty `gen/sNN.rs` so the port
//! still builds.

fn main() {
    let shard = std::env::var("CARGO_PKG_NAME").unwrap().trim_start_matches("recomp-").to_string();
    let dir = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../gen");
    let file = dir.join(format!("{shard}.rs"));
    if !file.exists() {
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&file, "// Empty: run ppc-recomp to generate recompiled code.
").unwrap();
    }
    println!("cargo:rerun-if-changed={}", file.display());
}
