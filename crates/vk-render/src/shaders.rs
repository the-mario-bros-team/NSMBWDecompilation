//! WGSL to SPIR-V with `naga` (pure Rust, no Vulkan SDK needed).

use naga::back::spv;
use naga::valid::{Capabilities, ValidationFlags, Validator};

const UBERSHADER: &str = include_str!("shader.wgsl");
const COPY: &str = include_str!("copy.wgsl");

fn entry(module: &naga::Module, info: &naga::valid::ModuleInfo, stage: naga::ShaderStage, name: &str) -> Result<Vec<u32>, String> {
    let mut opts = spv::Options { lang_version: (1, 5), ..Default::default() };
    // The CPU already outputs Vulkan clip space; don't flip y.
    opts.flags.remove(spv::WriterFlags::ADJUST_COORDINATE_SPACE);
    let pipeline = spv::PipelineOptions { shader_stage: stage, entry_point: name.to_string() };
    spv::write_vec(module, info, &opts, Some(&pipeline)).map_err(|e| format!("{name}: {e}"))
}

fn parse(source: &str) -> Result<(naga::Module, naga::valid::ModuleInfo), String> {
    let module = naga::front::wgsl::parse_str(source).map_err(|e| e.emit_to_string(source))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module).map_err(|e| e.emit_to_string(source))?;
    Ok((module, info))
}

/// (vertex, fragment) SPIR-V for one WGSL source with entry points `vs` and `fs`.
fn compile(source: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
    let (module, info) = parse(source)?;
    Ok((entry(&module, &info, naga::ShaderStage::Vertex, "vs")?, entry(&module, &info, naga::ShaderStage::Fragment, "fs")?))
}

/// The ubershader's fragment entry points: with dual-source blending
/// (`fs`, `fs_early`), and the single-output ones for GPUs without it.
pub const DUAL_SOURCE: [&str; 2] = ["fs", "fs_early"];
/// One output, the EFB's colour: [with depth, early depth].
pub const ONE: [&str; 2] = ["fs_one", "fs_one_early"];
/// One output, the blending alpha in place of the EFB's.
pub const ONE_BLEND: [&str; 2] = ["fs_one_blend", "fs_one_early_blend"];

/// SPIR-V for the ubershader's vertex stage and the fragment entry points
/// `names` (each a module of its own).
pub fn ubershader(names: &[&str]) -> Result<(Vec<u32>, Vec<Vec<u32>>), String> {
    let (module, info) = parse(UBERSHADER)?;
    let vs = entry(&module, &info, naga::ShaderStage::Vertex, "vs")?;
    let fs = names.iter().map(|n| entry(&module, &info, naga::ShaderStage::Fragment, n)).collect::<Result<_, _>>()?;
    Ok((vs, fs))
}

/// The EFB copy pass.
pub fn copy() -> Result<(Vec<u32>, Vec<u32>), String> {
    compile(COPY)
}

#[cfg(test)]
mod tests {
    /// Whether SPIR-V `code` decorates anything with `Index` (dual-source
    /// blending's second output).
    fn has_index_decoration(code: &[u32]) -> bool {
        const OP_DECORATE: u32 = 71;
        const OP_MEMBER_DECORATE: u32 = 72;
        const INDEX: u32 = 32;
        let mut i = 5;
        while i < code.len() {
            let (count, op) = ((code[i] >> 16) as usize, code[i] & 0xFFFF);
            if (op == OP_DECORATE && code.get(i + 2) == Some(&INDEX)) || (op == OP_MEMBER_DECORATE && code.get(i + 3) == Some(&INDEX)) {
                return true;
            }
            i += count.max(1);
        }
        false
    }

    #[test]
    fn shaders_compile() {
        let all: Vec<&str> = super::DUAL_SOURCE.iter().chain(&super::ONE).chain(&super::ONE_BLEND).copied().collect();
        let (vs, fs) = super::ubershader(&all).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(vs[0], 0x0723_0203);
        for (name, code) in all.iter().zip(&fs) {
            assert_eq!(code[0], 0x0723_0203);
            // The single-output stages must run on GPUs without dual-source blending.
            assert_eq!(has_index_decoration(code), name.starts_with("fs") && !name.starts_with("fs_one"), "{name}");
        }
        let (vs, fs) = super::copy().unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(vs[0], 0x0723_0203);
        assert_eq!(fs[0], 0x0723_0203);
    }
}
