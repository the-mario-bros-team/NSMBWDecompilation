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

/// (vertex, fragment) SPIR-V for one WGSL source with entry points `vs` and `fs`.
fn compile(source: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
    let module = naga::front::wgsl::parse_str(source).map_err(|e| e.emit_to_string(source))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module).map_err(|e| e.emit_to_string(source))?;
    Ok((entry(&module, &info, naga::ShaderStage::Vertex, "vs")?, entry(&module, &info, naga::ShaderStage::Fragment, "fs")?))
}

/// SPIR-V for the ubershader's vertex stage, fragment stage, and fragment
/// stage without a depth output.
pub type UbershaderCode = (Vec<u32>, Vec<u32>, Vec<u32>);

/// The ubershader's one texture fetch, with the coordinate's screen
/// derivatives (for mip levels and minification), and the same at the base
/// level without them.
const SAMPLE_GRAD: &str = "return textureSampleGrad(textures[slot], samplers[word(SAMPLER + m)], uv, dx, dy);";
const SAMPLE_BASE: &str = "return textureSampleLevel(textures[slot], samplers[word(SAMPLER + m)], uv, 0.0);";

/// The ubershader's source for this platform.
///
/// On Android it samples at the base level, which leaves the derivatives
/// (`dpdx`, `dpdy`) unused, for the GPU's compiler to drop: with them in
/// use, an Adreno 840 loses the device about 250 fields into the title
/// demo's snow course, in the draws of its lit 3D models
/// (docs/PORT_PLAN.md, 16). Why is not known: the fault stays with every
/// texture slot, sampler, coordinate and gradient forced valid, and goes
/// when nothing uses the derivatives. Mipmaps are not used there until it
/// is.
fn ubershader_source(base_level_only: bool) -> std::borrow::Cow<'static, str> {
    if base_level_only {
        UBERSHADER.replace(SAMPLE_GRAD, SAMPLE_BASE).into()
    } else {
        UBERSHADER.into()
    }
}

/// The GX ubershader: vertex, fragment, and fragment without a depth output.
pub fn ubershader() -> Result<UbershaderCode, String> {
    let source = ubershader_source(cfg!(target_os = "android"));
    let source: &str = &source;
    let (vs, fs) = compile(source)?;
    let module = naga::front::wgsl::parse_str(source).map_err(|e| e.emit_to_string(source))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module).map_err(|e| e.emit_to_string(source))?;
    Ok((vs, fs, entry(&module, &info, naga::ShaderStage::Fragment, "fs_early")?))
}

/// The EFB copy pass.
pub fn copy() -> Result<(Vec<u32>, Vec<u32>), String> {
    compile(COPY)
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_level_variant_compiles() {
        // The fetch the Android variant replaces is still in the source, and
        // the variant compiles.
        assert_eq!(super::UBERSHADER.matches(super::SAMPLE_GRAD).count(), 1);
        let source = super::ubershader_source(true);
        assert!(!source.contains("textureSampleGrad"));
        super::compile(&source).unwrap_or_else(|e| panic!("{e}"));
    }

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
