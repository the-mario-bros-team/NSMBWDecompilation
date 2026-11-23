//! WGSL to SPIR-V with `naga` (pure Rust, no Vulkan SDK needed).

use naga::back::spv;
use naga::valid::{Capabilities, ValidationFlags, Validator};

use render::{COPY, UBERSHADER};

fn entry(module: &naga::Module, info: &naga::valid::ModuleInfo, stage: naga::ShaderStage, name: &str) -> Result<Vec<u32>, String> {
    let mut opts = spv::Options { lang_version: (1, 5), ..Default::default() };
    // The CPU already outputs Vulkan clip space; don't flip y.
    opts.flags.remove(spv::WriterFlags::ADJUST_COORDINATE_SPACE);
    let pipeline = spv::PipelineOptions { shader_stage: stage, entry_point: name.to_string() };
    spv::write_vec(module, info, &opts, Some(&pipeline)).map_err(|e| format!("{name}: {e}"))
}

/// (vertex, fragment) SPIR-V for one WGSL source with entry points `vs` and `fs`.
fn compile(source: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
    let module = naga::front::wgsl::parse_str(source).map_err(|e| e.emit_to_string(source))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module).map_err(|e| e.emit_to_string(source))?;
    Ok((entry(&module, &info, naga::ShaderStage::Vertex, "vs")?, entry(&module, &info, naga::ShaderStage::Fragment, "fs")?))
}

/// SPIR-V for the ubershader's vertex stage, fragment stage, and fragment
/// stage without a depth output.
pub type UbershaderCode = (Vec<u32>, Vec<u32>, Vec<u32>);

/// The GX ubershader: vertex, fragment, and fragment without a depth output.
pub fn ubershader() -> Result<UbershaderCode, String> {
    let (vs, fs) = compile(UBERSHADER)?;
    let module = naga::front::wgsl::parse_str(UBERSHADER).map_err(|e| e.emit_to_string(UBERSHADER))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module).map_err(|e| e.emit_to_string(UBERSHADER))?;
    Ok((vs, fs, entry(&module, &info, naga::ShaderStage::Fragment, "fs_early")?))
}

/// The EFB copy pass.
pub fn copy() -> Result<(Vec<u32>, Vec<u32>), String> {
    compile(COPY)
}

#[cfg(test)]
mod tests {
    #[test]
    fn shaders_compile() {
        let (vs, fs, early) = super::ubershader().unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(early[0], 0x0723_0203);
        for (vs, fs) in [(vs, fs), super::copy().unwrap_or_else(|e| panic!("{e}"))] {
            assert_eq!(vs[0], 0x0723_0203);
            assert_eq!(fs[0], 0x0723_0203);
        }
    }
}
