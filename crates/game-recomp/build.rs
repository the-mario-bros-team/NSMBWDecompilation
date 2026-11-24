//! Without generated code (`ppc-recomp` not run yet, or not for every
//! version), writes an empty table so the port still builds; it then
//! interprets everything.

/// The versions other than USA revision 1, whose code is in `gen/<name>/`.
const VERSIONS: [&str; 7] = ["e2", "p1", "p2", "j1", "j2", "k", "w"];

fn main() {
    let gen = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("gen");
    for dir in std::iter::once(gen.clone()).chain(VERSIONS.iter().map(|v| gen.join(v))) {
        let module = dir.join("mod.rs");
        if !module.exists() {
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(
                &module,
                "// Empty: run ppc-recomp to generate recompiled code.\n#![allow(unused)]\npub use guest_cpu::recomp::FnInfo;\n\npub const SPECIAL: &[u32] = &[];\npub static FUNCTIONS: [FnInfo; 0] = [];\n",
            )
            .unwrap();
        }
    }
    println!("cargo:rerun-if-changed=gen");
}
