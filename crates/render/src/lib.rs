//! What the renderers share: the interface the frontend draws `gx::Output`
//! through ([`Backend`]), its types, the GX ubershader's WGSL source, and
//! helpers for decoding GX state the same way in every backend.
//!
//! The backends: Vulkan (`vk-render`, every platform), Direct3D 12 and
//! Direct3D 11 (`d3d-render`, Windows).

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use gx::{sh, Output, Raster, Vertex};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub type Res<T> = Result<T, Box<dyn Error>>;

/// The GX ubershader (vertex `vs`, fragment `fs` and `fs_early`).
pub const UBERSHADER: &str = include_str!("shader.wgsl");
/// The EFB copy pass (`vs`, `fs`).
pub const COPY: &str = include_str!("copy.wgsl");
/// The vertex stage for vertices the GPU decodes and transforms
/// (`gx::Output::pull`), appended to [`UBERSHADER`] by renderers that ask for
/// pulled draws.
pub const PULLED: &str = include_str!("pulled.wgsl");

/// Overlay colours: white, black, green, yellow, red, dark grey, grey, blue.
pub const PALETTE: [[u8; 4]; 8] = [[255, 255, 255, 255], [0, 0, 0, 255], [80, 220, 100, 255], [250, 210, 60, 255], [240, 70, 60, 255], [24, 24, 28, 255], [120, 120, 130, 255], [90, 150, 250, 255]];

pub const EFB_W: u32 = gx::EFB_WIDTH as u32;
pub const EFB_H: u32 = gx::EFB_HEIGHT as u32;
/// Kept XFB images (the game double- or triple-buffers its XFB).
pub const MAX_XFBS: usize = 4;

/// The graphics API a renderer draws with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Api {
    Vulkan,
    D3d12,
    D3d11,
}

impl Api {
    pub fn name(self) -> &'static str {
        match self {
            Api::Vulkan => "Vulkan",
            Api::D3d12 => "Direct3D 12",
            Api::D3d11 => "Direct3D 11",
        }
    }
}

static VALIDATION_ERRORS: AtomicU32 = AtomicU32::new(0);
static VALIDATION_WARNINGS: AtomicU32 = AtomicU32::new(0);

/// Errors the API's debug layer reported (Vulkan's validation layer, the
/// Direct3D debug layer), in every renderer so far.
pub fn validation_errors() -> u32 {
    VALIDATION_ERRORS.load(Ordering::Relaxed)
}

pub fn validation_warnings() -> u32 {
    VALIDATION_WARNINGS.load(Ordering::Relaxed)
}

/// Counts and prints a debug layer's message. A message repeated every
/// draw would bury the others (and overrun a phone's log): each kind (`id`)
/// is printed 20 times, then at every power of two.
pub fn note_validation(api: &str, error: bool, id: i64, msg: &str) {
    if error {
        VALIDATION_ERRORS.fetch_add(1, Ordering::Relaxed);
    } else {
        VALIDATION_WARNINGS.fetch_add(1, Ordering::Relaxed);
    }
    static SEEN: std::sync::Mutex<Option<HashMap<i64, u64>>> = std::sync::Mutex::new(None);
    let n = {
        let mut seen = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        let n = seen.get_or_insert_with(HashMap::new).entry(id).or_insert(0);
        *n += 1;
        *n
    };
    if n <= 20 || n.is_power_of_two() {
        let times = if n > 20 { format!(" ({n} times so far)") } else { String::new() };
        eprintln!("[{api} {}] {msg}{times}", if error { "error" } else { "warning" });
    }
}

/// How the renderer is set up.
#[derive(Clone, Debug)]
pub struct Options {
    /// EFB resolution multiplier (fractions too: 0.5 renders 320x264).
    pub scale: f32,
    /// Present on vertical blank (FIFO); otherwise mailbox or immediate.
    pub vsync: bool,
    /// Where the Vulkan pipeline cache is loaded from and saved to. The
    /// draw pipelines' keys go next to it (`.keys`), for every backend; the
    /// Direct3D backends keep their compiled shaders there too.
    pub pipeline_cache: Option<PathBuf>,
    /// Read EFB-to-texture copies back for the CPU to write into RAM
    /// ([`Backend::take_writebacks`]). Off, copies stay on the GPU only
    /// (Dolphin's default), which is much cheaper.
    pub efb_copies_to_ram: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { scale: 2.0, vsync: true, pipeline_cache: None, efb_copies_to_ram: true }
    }
}

/// Texels of an EFB copy at native resolution (RGBA8, as the texture
/// decoder would produce them), for the frontend to encode into RAM.
pub struct Writeback {
    pub addr: u32,
    /// The destination's hash at copy time (`gx::CopyTex::ram_hash`).
    pub ram_hash: u64,
    pub tex_format: u32,
    pub width: u32,
    pub height: u32,
    /// `gx::CopyTex::stride`: its rows' spacing in RAM when it patches a
    /// wider texture (0: one after another).
    pub stride: u32,
    pub rgba: Vec<u8>,
}

/// The EFB at native resolution, for CPU peeks.
pub struct EfbSnapshot {
    /// 640 x 528 RGBA8.
    pub color: Vec<u8>,
    /// 640 x 528 24-bit depth.
    pub depth: Vec<u32>,
}

/// A solid rectangle drawn over the presented image, in window pixels.
#[derive(Clone, Copy, Debug)]
pub struct OverlayRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    /// Index into [`PALETTE`].
    pub color: u8,
}

/// The menus drawn over the picture (the settings window): egui's output for
/// the frames shown until the next one ([`Backend::set_ui`]).
pub struct UiFrame {
    pub primitives: Vec<egui::ClippedPrimitive>,
    pub pixels_per_point: f32,
}

/// Frame statistics for the window title and logs.
#[derive(Default, Clone, Copy, Debug)]
pub struct FrameStats {
    pub draws: u32,
    /// Of `draws`, those drawn with a specialized pipeline (Vulkan).
    pub specialized: u32,
    pub pipelines: usize,
    pub textures: usize,
    pub uploads: usize,
}

/// A renderer: executes `gx::Output` into an EFB, keeps each XFB copy, and
/// presents the one the video interface scans out (with the overlay and the
/// menus over it). A player's window draws on a thread of its own, hence
/// `Send`.
pub trait Backend: Send {
    fn api(&self) -> Api;
    /// The GPU's name.
    fn device_name(&self) -> &str;
    /// Whether the API's debug layer is on.
    fn validation(&self) -> bool;
    /// The EFB resolution multiplier.
    fn scale(&self) -> f32;
    /// The picture is `stretch` times as wide as the game's 16:9.
    fn set_stretch(&mut self, stretch: f32);
    /// Display aspect ratio (width / height) the XFB is letterboxed to.
    fn set_aspect(&mut self, aspect: f32);
    /// Save the next presented XFB as a PNG here.
    fn set_screenshot(&mut self, path: Option<PathBuf>);
    /// Drawn over the presented image.
    fn set_overlay(&mut self, rects: Vec<OverlayRect>);
    fn stats(&self) -> FrameStats;
    /// How long the GPU took for the last frame it finished; `None` where
    /// the renderer doesn't time it.
    fn gpu_time(&self) -> Option<std::time::Duration> {
        None
    }
    /// Switches V-Sync from the next present.
    fn set_vsync(&mut self, on: bool);
    /// The window changed size.
    fn resized(&mut self);
    /// What the menus draw over the frames shown from now on, with egui's
    /// texture changes.
    fn set_ui(&mut self, frame: Option<UiFrame>, textures: &egui::TexturesDelta) -> Res<()>;
    /// Startup pipelines still being built.
    fn pipelines_compiling(&mut self) -> u32;
    /// Executes `out` with `vertices` and presents the XFB at `xfb_addr` (or
    /// the latest) to a `width` x `height` window (zero: nothing). `fresh`:
    /// the first render of `out` (uploads its textures, reads its EFB
    /// copies back). Returns whether a frame was presented.
    fn render(&mut self, out: &Output, vertices: &[Vertex], fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool>;
    /// [`Backend::render`] with `fill` writing the `count` vertices into the
    /// GPU's buffer.
    #[allow(clippy::too_many_arguments)]
    fn render_with(&mut self, out: &Output, count: usize, fill: &mut dyn FnMut(&mut [Vertex]), fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool>;
    /// The key of the field on the GPU for in-between frames, if any.
    fn prepared(&self) -> Option<(u64, u64)>;
    /// Keeps `out` on the GPU for [`Backend::render_prepared`].
    fn prepare(&mut self, key: (u64, u64), out: &Output, moves: Option<&gx::interp::Moves>) -> Res<()>;
    /// [`Backend::prepare`] with `vertices` in place of the field's own.
    fn prepare_with(&mut self, key: (u64, u64), out: &Output, vertices: &[Vertex], moves: Option<&gx::interp::Moves>) -> Res<()>;
    /// Renders the prepared field at `t` between the field before and it.
    fn render_prepared(&mut self, out: &Output, t: f32, xfb_addr: u32, width: u32, height: u32) -> Res<bool>;
    /// Drops textures the GX texture cache evicted.
    fn evict(&mut self, ids: &[u32]);
    /// EFB copies read back so far.
    fn take_writebacks(&mut self) -> Vec<Writeback>;
    /// Waits for the GPU and collects every outstanding writeback.
    fn finish(&mut self) -> Res<()>;
    /// The EFB at native resolution (waits for the GPU).
    fn efb_snapshot(&mut self) -> Res<EfbSnapshot>;
    /// A renderer of the same kind presenting to another window on this
    /// one's GPU (a player's window), which may draw on another thread.
    fn new_sharing(&self, display: RawDisplayHandle, window: RawWindowHandle, opts: Options) -> Res<Box<dyn Backend>>;
}

/// `n` EFB pixels at scale `s`: rounded, and at least one.
pub fn at_scale(n: u32, s: f32) -> u32 {
    ((n as f32 * s).round() as u32).max(1)
}

/// Pipeline key: the BP blend-mode bits that pipelines bake in, plus whether
/// the EFB has alpha.
pub fn blend_key(r: &Raster) -> u32 {
    (r.blend & 0xFFFB) | u32::from(r.efb_alpha) << 16
}

/// Pipeline key bit: the fragment shader leaves depth to the rasteriser.
pub const EARLY_DEPTH: u32 = 1 << 17;

/// Draw pipelines New Super Mario Bros. Wii uses, built at startup before
/// any saved list exists: the keys `cache/pipelines.keys` held after runs
/// through boot, the title, the World 1 map and 1-1, alone and with four
/// players (2026-11-22). The ubershader does the rest, so there are only
/// these few.
pub const STARTUP_KEYS: &[u32] = &[
    0x5018, 0x2_0128, 0x2_0129, 0x3_0010, 0x3_0069, 0x3_0429, 0x3_04A9, 0x3_3118, 0x3_3209, 0x3_3429, 0x3_3439, 0x3_34A8, 0x3_34A9, 0x3_34B8, 0x3_5131, 0x3_5931, 0x3_F118, 0x3_F119, 0x3_F4A9, 0x3_F4B9,
];

/// Whether a draw's depth is the rasterised one: no z-freeze plane and no
/// Z texture.
pub fn rasterised_depth(s: &[u32]) -> bool {
    s[sh::FLAGS] & gx::FLAG_ZFREEZE == 0 && (s[sh::ZTEX + 1] >> 2) & 3 == 0
}

/// A pipeline key's blending, decoded (`blend_key`; GX precedence:
/// subtract, then blending, then the logic op).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blend {
    /// Written as it is.
    Off,
    /// dst - src for colour and alpha.
    Subtract,
    /// src * `src` + dst * `dst` (GX factors 0..7, [`BlendFactor`]).
    Add { src: BlendFactor, dst: BlendFactor },
    /// One of the sixteen logic ops, in GX's (and Vulkan's) order.
    Logic(u32),
}

/// A blend factor. `Src1Alpha` is the second fragment output's alpha
/// (dual-source), which always carries the TEV's alpha.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    Src1Alpha,
    OneMinusSrc1Alpha,
    DstAlpha,
    OneMinusDstAlpha,
}

fn blend_factor(f: u32, src: bool, efb_alpha: bool) -> BlendFactor {
    use BlendFactor as B;
    match f {
        0 => B::Zero,
        1 => B::One,
        2 if src => B::DstColor,
        2 => B::SrcColor,
        3 if src => B::OneMinusDstColor,
        3 => B::OneMinusSrcColor,
        4 => B::Src1Alpha,
        5 => B::OneMinusSrc1Alpha,
        6 if efb_alpha => B::DstAlpha,
        6 => B::One,
        7 if efb_alpha => B::OneMinusDstAlpha,
        _ => B::Zero,
    }
}

/// The blending of pipeline key `key`, and its write mask (bit 0 colour,
/// bit 1 alpha).
pub fn blend_of(key: u32) -> (Blend, u32) {
    let efb_alpha = key & (1 << 16) != 0;
    let (blend_on, logic_on, subtract) = (key & 1 != 0, key & 2 != 0, key & (1 << 11) != 0);
    let mask = u32::from(key & 8 != 0) | u32::from(key & 0x10 != 0) << 1;
    let blend = if subtract {
        Blend::Subtract
    } else if blend_on {
        Blend::Add { src: blend_factor((key >> 8) & 7, true, efb_alpha), dst: blend_factor((key >> 5) & 7, false, efb_alpha) }
    } else if logic_on {
        Blend::Logic((key >> 12) & 0xF)
    } else {
        Blend::Off
    };
    (blend, mask)
}

/// A logic op done with blending, for a GPU without logic ops (Direct3D
/// has them on integer formats only, and not on every GPU). Exact for clear
/// and no-op (as src * a + dst * b); `None` for the rest, which are drawn as
/// copy. New Super Mario Bros. Wii draws with none (none of its pipeline
/// keys has a logic op).
pub fn logic_as_blend(op: u32) -> Option<(BlendFactor, BlendFactor)> {
    use BlendFactor as B;
    match op {
        0 => Some((B::Zero, B::Zero)), // clear
        5 => Some((B::Zero, B::One)),  // no-op
        _ => None,
    }
}

/// A GX sampler key decoded (`gx::sh::SAMPLER` words).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerDesc {
    /// 0 clamp, 1 repeat, 2 mirrored repeat.
    pub wrap_u: u32,
    pub wrap_v: u32,
    pub mag_linear: bool,
    pub min_linear: bool,
    pub mip_linear: bool,
    /// `None`: no limit (a texture pack's image with its own mips).
    pub max_lod: Option<f32>,
    pub lod_bias: f32,
}

pub fn sampler_desc(key: u32) -> SamplerDesc {
    let wrap = |w: u32| match w & 3 {
        0 => 0,
        1 => 1,
        _ => 2,
    };
    let min = (key >> 5) & 7;
    // A texture pack's image (bit 24) has its own mip levels, however
    // many: a linear-filtered texture without mips blends between them.
    let replaced = key & (1 << 24) != 0;
    SamplerDesc {
        wrap_u: wrap(key),
        wrap_v: wrap(key >> 2),
        mag_linear: (key >> 4) & 1 != 0,
        min_linear: min >= 4,
        mip_linear: min & 3 == 2 || (replaced && min == 4),
        max_lod: if replaced { None } else if min & 3 != 0 { Some(((key >> 20) & 0xF) as f32) } else { Some(0.0) },
        // LOD bias: s2.5 fixed point in bits 9..16.
        lod_bias: ((((key >> 9) & 0xFF) as u8) as i8) as f32 / 32.0,
    }
}

/// The copy pass's parameters (`copy.wgsl`'s `Params`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CopyParams {
    pub src: [f32; 4],
    pub dst: [f32; 2],
    pub format: u32,
    pub flags: u32,
}

/// The XFB letterboxed into a `tw` x `th` target at `aspect`: x, y, w, h.
pub fn letterbox(tw: u32, th: u32, aspect: f32) -> (u32, u32, u32, u32) {
    let (tw, th) = (tw as f32, th as f32);
    let (w, h) = if tw / th > aspect { (th * aspect, th) } else { (tw, tw / aspect) };
    (((tw - w) / 2.0) as u32, ((th - h) / 2.0) as u32, (w as u32).max(1), (h as u32).max(1))
}

/// Overlay rectangles clipped to a `tw` x `th` target: (x0, y0, x1, y1, colour).
pub fn overlay_rects(rects: &[OverlayRect], tw: u32, th: u32) -> Vec<(i32, i32, i32, i32, [u8; 4])> {
    let (tw, th) = (tw as i32, th as i32);
    rects
        .iter()
        .filter_map(|r| {
            let (x0, y0) = (r.x.clamp(0, tw), r.y.clamp(0, th));
            let (x1, y1) = ((r.x + r.w).clamp(0, tw), (r.y + r.h).clamp(0, th));
            (x1 > x0 && y1 > y0).then(|| (x0, y0, x1, y1, PALETTE[(r.color % PALETTE.len() as u8) as usize]))
        })
        .collect()
}

/// Writes `rgba` (opaque: the EFB keeps no alpha the display uses) as a PNG.
pub fn save_png(path: &std::path::Path, w: u32, h: u32, rgba: &[u8]) -> Res<()> {
    let file = std::io::BufWriter::new(std::fs::File::create(path)?);
    let mut enc = png::Encoder::new(file, w, h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header()?;
    let opaque: Vec<u8> = rgba.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2], 255]).collect();
    writer.write_image_data(&opaque)?;
    Ok(())
}

/// Reads a PNG as RGBA8: (width, height, pixels).
pub fn load_png(path: &std::path::Path) -> Res<(u32, u32, Vec<u8>)> {
    let mut dec = png::Decoder::new(std::io::BufReader::new(std::fs::File::open(path)?));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = dec.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size().ok_or("PNG too large")?];
    let info = reader.next_frame(&mut buf)?;
    let px = &buf[..info.buffer_size()];
    let rgba = match info.color_type {
        png::ColorType::Rgba => px.to_vec(),
        png::ColorType::Rgb => px.chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect(),
        png::ColorType::GrayscaleAlpha => px.chunks_exact(2).flat_map(|p| [p[0], p[0], p[0], p[1]]).collect(),
        _ => px.iter().flat_map(|&g| [g, g, g, 255]).collect(),
    };
    Ok((info.width, info.height, rgba))
}

/// Plain data as bytes.
pub fn as_bytes<T: Copy>(v: &[T]) -> &[u8] {
    // SAFETY: T is plain old data (`gx::Vertex` is repr(C) f32s; u32).
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast::<u8>(), std::mem::size_of_val(v)) }
}

/// The pipeline keys to build at startup: [`STARTUP_KEYS`] and those saved
/// next to `cache` by an earlier run.
pub fn startup_keys(cache: Option<&std::path::Path>) -> Vec<u32> {
    let mut keys: Vec<u32> = STARTUP_KEYS.to_vec();
    if let Some(text) = cache.and_then(|p| std::fs::read_to_string(p.with_extension("keys")).ok()) {
        keys.extend(text.lines().filter_map(|l| u32::from_str_radix(l.trim(), 16).ok()));
    }
    keys.sort_unstable();
    keys.dedup();
    keys
}

/// Saves the pipeline keys this run had next to `cache`, built first next time.
pub fn save_keys(cache: &std::path::Path, keys: impl Iterator<Item = u32>) {
    let mut keys: Vec<u32> = keys.collect();
    keys.sort_unstable();
    let text: String = keys.iter().map(|k| format!("{k:x}\n")).collect();
    let _ = std::fs::write(cache.with_extension("keys"), text);
}
