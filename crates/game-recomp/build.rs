//! Without generated code (`ppc-recomp` not run yet), writes an empty table
//! so the port still builds; it then interprets everything.

fn main() {
    let dir = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("gen");
    let module = dir.join("mod.rs");
    if !module.exists() {
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            &module,
            "// Empty: run ppc-recomp to generate recompiled code.\n#![allow(unused)]\npub use guest_cpu::recomp::FnInfo;\n\npub const SPECIAL: &[u32] = &[];\npub static FUNCTIONS: [FnInfo; 0] = [];\n",
        )
        .unwrap();
    }
    println!("cargo:rerun-if-changed=gen");
}
