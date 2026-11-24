//! WGSL to SPIR-V with `naga` (pure Rust, no Vulkan SDK needed).

use naga::back::spv;
use naga::valid::{Capabilities, ValidationFlags, Validator};

const UBERSHADER: &str = include_str!("shader.wgsl");
const COPY: &str = include_str!("copy.wgsl");

/// SPIR-V version `spirv`: 1.5 for Vulkan 1.3, 1.3 for Vulkan 1.1.
fn entry(module: &naga::Module, info: &naga::valid::ModuleInfo, stage: naga::ShaderStage, name: &str, spirv: (u8, u8)) -> Result<Vec<u32>, String> {
    let mut opts = spv::Options { lang_version: spirv, ..Default::default() };
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
fn compile(source: &str, spirv: (u8, u8)) -> Result<(Vec<u32>, Vec<u32>), String> {
    let (module, info) = parse(source)?;
    Ok((entry(&module, &info, naga::ShaderStage::Vertex, "vs", spirv)?, entry(&module, &info, naga::ShaderStage::Fragment, "fs", spirv)?))
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
pub fn ubershader(names: &[&str], spirv: (u8, u8)) -> Result<(Vec<u32>, Vec<Vec<u32>>), String> {
    let (module, info) = parse(UBERSHADER)?;
    let vs = entry(&module, &info, naga::ShaderStage::Vertex, "vs", spirv)?;
    let fs = names.iter().map(|n| entry(&module, &info, naga::ShaderStage::Fragment, n, spirv)).collect::<Result<_, _>>()?;
    Ok((vs, fs))
}

/// The ubershader's fragment entry point `name` with shading words baked in
/// (`fixed`: index, mask of the bits baked, their value): `word(i)` returns
/// those bits as constants (and reads only the rest), so the driver can
/// unroll the TEV loop and leave out what the draw doesn't use. The
/// arithmetic is the ubershader's, so the pixels are the same.
pub fn specialized(name: &str, fixed: &[(u32, u32, u32)], spirv: (u8, u8)) -> Result<Vec<u32>, String> {
    const BODY: &str = "    return shadings[sidx].w[i >> 2u][i & 3u];";
    let mut cases = String::new();
    for &(i, mask, v) in fixed {
        let v = v & mask;
        if mask == u32::MAX {
            cases += &format!("        case {i}u: {{ return {v}u; }}\n");
        } else {
            cases += &format!("        case {i}u: {{ return (shadings[sidx].w[{}u][{}u] & {}u) | {v}u; }}\n", i >> 2, i & 3, !mask);
        }
    }
    let src = UBERSHADER.replacen(BODY, &format!("    switch i {{\n{cases}        default: {{ return shadings[sidx].w[i >> 2u][i & 3u]; }}\n    }}"), 1);
    if src == UBERSHADER {
        return Err("the ubershader's word() changed".into());
    }
    let (module, info) = parse(&src)?;
    entry(&module, &info, naga::ShaderStage::Fragment, name, spirv)
}

/// The EFB copy pass.
pub fn copy(spirv: (u8, u8)) -> Result<(Vec<u32>, Vec<u32>), String> {
    compile(COPY, spirv)
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
        // SPIR-V 1.5 for Vulkan 1.3, 1.3 for Vulkan 1.1 (word 1 is the version).
        for spirv in [(1, 5), (1, 3)] {
            let version = u32::from(spirv.0) << 16 | u32::from(spirv.1) << 8;
            let (vs, fs) = super::ubershader(&all, spirv).unwrap_or_else(|e| panic!("{e}"));
            assert_eq!(vs[..2], [0x0723_0203, version]);
            for (name, code) in all.iter().zip(&fs) {
                assert_eq!(code[..2], [0x0723_0203, version]);
                // The single-output stages must run on GPUs without dual-source blending.
                assert_eq!(has_index_decoration(code), name.starts_with("fs") && !name.starts_with("fs_one"), "{name}");
            }
            let (vs, fs) = super::copy(spirv).unwrap_or_else(|e| panic!("{e}"));
            assert_eq!(vs[..2], [0x0723_0203, version]);
            assert_eq!(fs[..2], [0x0723_0203, version]);
        }
    }

    #[test]
    fn specialized_compiles() {
        // Two stages, partly masked words, every fragment entry point.
        let fixed = [(0, u32::MAX, 1 << 10), (1, u32::MAX, 0x0004_8040), (65, 0xFFFF_0000, 0x00E0_0000), (135, 0xFFF0_0000, 0)];
        for name in super::DUAL_SOURCE.iter().chain(&super::ONE).chain(&super::ONE_BLEND) {
            let code = super::specialized(name, &fixed, (1, 3)).unwrap_or_else(|e| panic!("{name}: {e}"));
            assert_eq!(code[0], 0x0723_0203);
        }
    }
}
