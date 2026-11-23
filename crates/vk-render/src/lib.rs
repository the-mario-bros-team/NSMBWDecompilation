//! Vulkan renderer (docs/PORT_PLAN.md, section 2.5): executes `gx::Output`
//! into an EFB render target, keeps each XFB copy as an image, and presents
//! the one the video interface scans out.
//!
//! - One ubershader pair (`shader.wgsl`, compiled to SPIR-V by `naga` at
//!   startup); the TEV state of each draw sits in a storage buffer indexed
//!   by the draw's instance index. Destination alpha uses dual-source
//!   blending; logic ops use Vulkan's.
//! - Textures and samplers are bound as arrays; each frame writes the slots
//!   it uses into its own descriptor set.
//! - Pipelines are keyed by blend state; cull mode, depth state and scissor
//!   are dynamic (Vulkan 1.3). The pipeline cache can be kept on disk.
//! - EFB copies to textures go through a copy pass (`copy.wgsl`) that
//!   converts to the copy format; a second, native-resolution pass is read
//!   back so the frontend can write the texels to guest RAM, and serves CPU
//!   EFB peeks.
//! - `Renderer::headless` renders without a window (tests, FIFO-log replay).
//!
//! Debug builds enable `VK_LAYER_KHRONOS_validation` when it is installed;
//! so does any build with `NSMBW_VALIDATION` set in the environment.

mod shaders;

use std::collections::HashMap;
use std::error::Error;
use std::ffi::{c_char, c_void, CStr};
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use ash::ext::debug_utils;
use ash::khr::{surface, swapchain};
use ash::{vk, Device, Entry, Instance};
use gpu_allocator::vulkan::{Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc};
use gpu_allocator::MemoryLocation;
use gx::{sh, Cmd, Output, Raster, Rect, Vertex, NO_TEXTURE, SHADING_WORDS};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub type Res<T> = Result<T, Box<dyn Error>>;

const FRAMES_IN_FLIGHT: usize = 2;
const MAX_TEXTURES: u32 = 4096;
const MAX_SAMPLERS: u32 = 256;
const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const DEPTH_FORMAT: vk::Format = vk::Format::D32_SFLOAT;
const VERTEX_SIZE: u32 = std::mem::size_of::<Vertex>() as u32;
const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
/// Kept XFB images (the game double- or triple-buffers its XFB).
const MAX_XFBS: usize = 4;
const EFB_W: u32 = gx::EFB_WIDTH as u32;
const EFB_H: u32 = gx::EFB_HEIGHT as u32;

/// Overlay colours: white, black, green, yellow, red, dark grey, grey, blue.
pub const PALETTE: [[u8; 4]; 8] = [[255, 255, 255, 255], [0, 0, 0, 255], [80, 220, 100, 255], [250, 210, 60, 255], [240, 70, 60, 255], [24, 24, 28, 255], [120, 120, 130, 255], [90, 150, 250, 255]];

static VALIDATION_ERRORS: AtomicU32 = AtomicU32::new(0);
static VALIDATION_WARNINGS: AtomicU32 = AtomicU32::new(0);

pub fn validation_errors() -> u32 {
    VALIDATION_ERRORS.load(Ordering::Relaxed)
}

pub fn validation_warnings() -> u32 {
    VALIDATION_WARNINGS.load(Ordering::Relaxed)
}

unsafe extern "system" fn on_validation_message(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user: *mut c_void,
) -> vk::Bool32 {
    let msg = if data.is_null() || (*data).p_message.is_null() {
        "(no message)".into()
    } else {
        CStr::from_ptr((*data).p_message).to_string_lossy()
    };
    let error = severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR);
    if error {
        VALIDATION_ERRORS.fetch_add(1, Ordering::Relaxed);
    } else {
        VALIDATION_WARNINGS.fetch_add(1, Ordering::Relaxed);
    }
    // A message repeated every draw would bury the others (and overrun a
    // phone's log): each kind is printed 20 times, then at every power of two.
    static SEEN: std::sync::Mutex<Option<HashMap<i32, u64>>> = std::sync::Mutex::new(None);
    let id = if data.is_null() { 0 } else { (*data).message_id_number };
    let n = {
        let mut seen = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        let n = seen.get_or_insert_with(HashMap::new).entry(id).or_insert(0);
        *n += 1;
        *n
    };
    if n <= 20 || n.is_power_of_two() {
        let times = if n > 20 { format!(" ({n} times so far)") } else { String::new() };
        eprintln!("[vulkan {}] {msg}{times}", if error { "error" } else { "warning" });
    }
    vk::FALSE
}

/// How the renderer is set up.
#[derive(Clone, Debug)]
pub struct Options {
    /// EFB resolution multiplier (fractions too: 0.5 renders 320x264).
    pub scale: f32,
    /// Present on vertical blank (FIFO); otherwise mailbox or immediate.
    pub vsync: bool,
    /// Where the Vulkan pipeline cache is loaded from and saved to.
    pub pipeline_cache: Option<PathBuf>,
    /// Read EFB-to-texture copies back for the CPU to write into RAM
    /// ([`Renderer::take_writebacks`]). Off, copies stay on the GPU only
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

struct Image {
    image: vk::Image,
    view: vk::ImageView,
    alloc: Option<Allocation>,
    w: u32,
    h: u32,
    levels: u32,
    aspect: vk::ImageAspectFlags,
    layout: vk::ImageLayout,
}

struct Buffer {
    buffer: vk::Buffer,
    alloc: Option<Allocation>,
    size: u64,
}

/// A readback the frame owes: copy texels at `offset` of its buffer.
struct PendingWriteback {
    addr: u32,
    ram_hash: u64,
    tex_format: u32,
    w: u32,
    h: u32,
    stride: u32,
    offset: usize,
}

struct Frame {
    cmd: vk::CommandBuffer,
    fence: vk::Fence,
    acquired: vk::Semaphore,
    set: vk::DescriptorSet,
    vertices: Option<Buffer>,
    shadings: Option<Buffer>,
    staging: Option<Buffer>,
    readback: Option<Buffer>,
    writebacks: Vec<PendingWriteback>,
    /// What this frame's descriptor set and draw states were last written
    /// for by [`Renderer::render_prepared`]: (prepare serial, texture
    /// epoch); `None` after any other render.
    described: Option<(u64, u64)>,
    /// Resources retired before this frame's last submission: free once its
    /// fence signals (submissions complete in order on the one queue).
    retired_images: Vec<Image>,
    retired_buffers: Vec<Buffer>,
    /// Prepared-field buffers retired the same way, reused afterwards.
    retired_spare: Vec<Buffer>,
}

/// A field kept on the GPU for the frames shown between it and the field
/// before ([`Renderer::prepare`]): one buffer holding its vertices, its
/// indices, and frame interpolation's data (`gx::interp::Moves`).
struct Prepared {
    key: (u64, u64),
    /// Counts prepares, so descriptors written for one are never reused.
    serial: u64,
    buffer: Buffer,
    index_offset: u64,
    /// (offset, size) of the per-vertex model positions, matrix pairs and
    /// draw pairs; `None` when nothing moves.
    moves: Option<[(u64, u64); 3]>,
}

/// The menus drawn over the picture (the settings window): egui's output for
/// the frames shown until the next one ([`Renderer::set_ui`]).
pub struct UiFrame {
    pub primitives: Vec<egui::ClippedPrimitive>,
    pub pixels_per_point: f32,
}

/// egui's Vulkan renderer, made the first time a menu is drawn, for the
/// swapchain's format.
struct UiLayer {
    renderer: egui_ash_renderer::Renderer<egui_ash_renderer::allocator::DefaultAllocator>,
    format: vk::Format,
}

/// The vertex stage's push constant: fraction between the two fields
/// (f32 bits), and whether to move vertices at all.
type Push = [u32; 2];

/// The copy pass: pipeline and the descriptor set binding the EFB.
#[derive(Clone, Copy)]
struct CopyPass {
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    set: vk::DescriptorSet,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CopyParams {
    src: [f32; 4],
    dst: [f32; 2],
    format: u32,
    flags: u32,
}

/// Frame statistics for the window title and logs.
#[derive(Default, Clone, Copy, Debug)]
pub struct FrameStats {
    pub draws: u32,
    pub pipelines: usize,
    pub textures: usize,
    pub uploads: usize,
}

pub struct Renderer {
    pub device_name: String,
    pub validation: bool,
    /// Internal resolution multiplier for the EFB.
    scale: f32,
    /// The EFB's width over its 16:9 one: the picture is wider (or
    /// narrower) than the game's 16:9 ([`Renderer::set_stretch`]).
    stretch: f32,
    /// The EFB images are for another stretch: made again before the next frame.
    efb_stale: bool,
    vsync: bool,
    copies_to_ram: bool,
    _entry: Entry,
    instance: Instance,
    debug: Option<(debug_utils::Instance, vk::DebugUtilsMessengerEXT)>,
    surface: Option<(surface::Instance, vk::SurfaceKHR)>,
    physical: vk::PhysicalDevice,
    /// The queue family drawn and presented with.
    family: u32,
    device: Device,
    queue: vk::Queue,
    alloc: ManuallyDrop<Allocator>,
    swapchain_fn: Option<swapchain::Device>,
    swapchain: vk::SwapchainKHR,
    swap_images: Vec<vk::Image>,
    /// Views of the swapchain images, for drawing the menus into them.
    swap_views: Vec<vk::ImageView>,
    swap_format: vk::Format,
    ui: Option<UiLayer>,
    ui_frame: Option<UiFrame>,
    swap_extent: vk::Extent2D,
    /// Per swapchain image: signalled when its frame is done, waited on by present.
    rendered: Vec<vk::Semaphore>,
    needs_rebuild: bool,
    /// The swapchain is kept upright on a turned display (see `rebuild`).
    upright: bool,
    pool: vk::CommandPool,
    frames: Vec<Frame>,
    frame: usize,
    set_layout: vk::DescriptorSetLayout,
    copy_set_layout: vk::DescriptorSetLayout,
    desc_pool: vk::DescriptorPool,
    pipeline_layout: vk::PipelineLayout,
    pipeline_cache: vk::PipelineCache,
    pipeline_cache_path: Option<PathBuf>,
    vs: vk::ShaderModule,
    fs: vk::ShaderModule,
    /// `fs` without a depth output (early depth testing).
    fs_early: vk::ShaderModule,
    copy_modules: [vk::ShaderModule; 2],
    copy: CopyPass,
    pipelines: HashMap<u32, vk::Pipeline>,
    efb_color: Image,
    efb_depth: Image,
    /// Native-size scratch target for writebacks and peeks.
    native: Image,
    palette: Image,
    efb_ready: bool,
    /// Where this frame's indices start in its vertex buffer.
    index_offset: u64,
    /// The vertex stage's push constant for the frame being recorded.
    push: Push,
    prepared: Option<Prepared>,
    /// Prepared-field buffers no frame uses any more, kept for the next
    /// fields (they are megabytes, and come at 60 a second); and those
    /// retired since the last submission.
    spare: Vec<Buffer>,
    trash_spare: Vec<Buffer>,
    textures: HashMap<u32, Image>,
    /// Changes whenever `textures` does (descriptors written for one epoch
    /// are stale in the next).
    textures_epoch: u64,
    samplers: HashMap<u32, vk::Sampler>,
    /// XFB images by guest address, with the serial of their last copy.
    xfbs: HashMap<u32, (Image, u64)>,
    xfb_serial: u64,
    writebacks: Vec<Writeback>,
    /// Retired resources, destroyed once every frame in flight is done.
    trash_images: Vec<Image>,
    trash_buffers: Vec<Buffer>,
    /// Display aspect ratio (width / height) the XFB is letterboxed to.
    pub aspect: f32,
    /// Save the next presented XFB as a PNG here.
    pub screenshot: Option<PathBuf>,
    /// Drawn over the presented image.
    pub overlay: Vec<OverlayRect>,
    pub stats: FrameStats,
}

fn range(aspect: vk::ImageAspectFlags, levels: u32) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange { aspect_mask: aspect, base_mip_level: 0, level_count: levels, base_array_layer: 0, layer_count: 1 }
}

/// Moves `img` to `layout` with a full barrier (simple, and fast enough here).
unsafe fn transition(device: &Device, cmd: vk::CommandBuffer, img: &mut Image, layout: vk::ImageLayout) {
    if img.layout == layout {
        return;
    }
    let b = vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
        .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE)
        .old_layout(img.layout)
        .new_layout(layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(img.image)
        .subresource_range(range(img.aspect, img.levels));
    device.cmd_pipeline_barrier(cmd, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[b]);
    img.layout = layout;
}

fn sub_layers(aspect: vk::ImageAspectFlags, level: u32) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers { aspect_mask: aspect, mip_level: level, base_array_layer: 0, layer_count: 1 }
}

fn offsets(x: u32, y: u32, w: u32, h: u32) -> [vk::Offset3D; 2] {
    [vk::Offset3D { x: x as i32, y: y as i32, z: 0 }, vk::Offset3D { x: (x + w) as i32, y: (y + h) as i32, z: 1 }]
}

/// `n` EFB pixels at scale `s`: rounded, and at least one.
fn at_scale(n: u32, s: f32) -> u32 {
    ((n as f32 * s).round() as u32).max(1)
}

fn rect2d(x: u32, y: u32, w: u32, h: u32) -> vk::Rect2D {
    vk::Rect2D { offset: vk::Offset2D { x: x as i32, y: y as i32 }, extent: vk::Extent2D { width: w, height: h } }
}

/// GX blend factor `f` for the source (`src`) or destination side. The
/// source alpha comes from the second fragment output (dual-source), which
/// always carries the TEV's alpha.
fn blend_factor(f: u32, src: bool, efb_alpha: bool) -> vk::BlendFactor {
    use vk::BlendFactor as B;
    match f {
        0 => B::ZERO,
        1 => B::ONE,
        2 if src => B::DST_COLOR,
        2 => B::SRC_COLOR,
        3 if src => B::ONE_MINUS_DST_COLOR,
        3 => B::ONE_MINUS_SRC_COLOR,
        4 => B::SRC1_ALPHA,
        5 => B::ONE_MINUS_SRC1_ALPHA,
        6 if efb_alpha => B::DST_ALPHA,
        6 => B::ONE,
        7 if efb_alpha => B::ONE_MINUS_DST_ALPHA,
        _ => B::ZERO,
    }
}

/// Pipeline key: the BP blend-mode bits that pipelines bake in, plus whether
/// the EFB has alpha.
fn blend_key(r: &Raster) -> u32 {
    (r.blend & 0xFFFB) | u32::from(r.efb_alpha) << 16
}

/// Pipeline key bit: the fragment shader leaves depth to the rasteriser.
const EARLY_DEPTH: u32 = 1 << 17;

/// Whether a draw's depth is the rasterised one: no z-freeze plane and no
/// Z texture.
fn rasterised_depth(s: &[u32]) -> bool {
    s[sh::FLAGS] & gx::FLAG_ZFREEZE == 0 && (s[sh::ZTEX + 1] >> 2) & 3 == 0
}

impl Renderer {
    /// A renderer presenting to a window.
    pub fn new(display: RawDisplayHandle, window: RawWindowHandle, opts: Options) -> Res<Self> {
        Self::create(Some((display, window)), opts)
    }

    /// A renderer without a window: `render` with a zero window size never
    /// presents; results come from [`Renderer::efb_snapshot`], screenshots
    /// and writebacks.
    pub fn headless(opts: Options) -> Res<Self> {
        Self::create(None, opts)
    }

    fn create(window: Option<(RawDisplayHandle, RawWindowHandle)>, opts: Options) -> Res<Self> {
        let scale = opts.scale.max(0.25);
        unsafe {
            let entry = Entry::load()?;
            let layers_present = entry.enumerate_instance_layer_properties()?;
            let validation = (cfg!(debug_assertions) || std::env::var_os("NSMBW_VALIDATION").is_some()) && layers_present.iter().any(|l| l.layer_name_as_c_str().is_ok_and(|n| n == VALIDATION_LAYER));
            let mut extensions: Vec<*const c_char> = match window {
                Some((display, _)) => ash_window::enumerate_required_extensions(display)?.to_vec(),
                None => Vec::new(),
            };
            let mut layers: Vec<*const c_char> = Vec::new();
            if validation {
                extensions.push(debug_utils::NAME.as_ptr());
                layers.push(VALIDATION_LAYER.as_ptr());
            }
            let app = vk::ApplicationInfo::default().application_name(c"nsmbw-port").engine_name(c"nsmbw-port").api_version(vk::API_VERSION_1_3);
            let instance = entry.create_instance(&vk::InstanceCreateInfo::default().application_info(&app).enabled_extension_names(&extensions).enabled_layer_names(&layers), None)?;
            let debug = if validation {
                let f = debug_utils::Instance::new(&entry, &instance);
                let info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                    .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
                    .message_type(vk::DebugUtilsMessageTypeFlagsEXT::GENERAL | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE)
                    .pfn_user_callback(Some(on_validation_message));
                let messenger = f.create_debug_utils_messenger(&info, None)?;
                Some((f, messenger))
            } else {
                None
            };
            let surface = match window {
                Some((display, handle)) => Some((surface::Instance::new(&entry, &instance), ash_window::create_surface(&entry, &instance, display, handle, None)?)),
                None => None,
            };

            // Prefer a discrete GPU with Vulkan 1.3; take the first queue family that can draw (and present).
            let mut best: Option<(u32, vk::PhysicalDevice, u32, vk::PhysicalDeviceProperties)> = None;
            for pd in instance.enumerate_physical_devices()? {
                let props = instance.get_physical_device_properties(pd);
                let features = instance.get_physical_device_features(pd);
                if props.api_version < vk::API_VERSION_1_3 || features.dual_src_blend == 0 || features.logic_op == 0 {
                    continue;
                }
                for (i, fam) in instance.get_physical_device_queue_family_properties(pd).iter().enumerate() {
                    let i = i as u32;
                    let presents = match &surface {
                        Some((f, s)) => f.get_physical_device_surface_support(pd, i, *s)?,
                        None => true,
                    };
                    if fam.queue_flags.contains(vk::QueueFlags::GRAPHICS) && presents {
                        let score = if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU { 2 } else { 1 };
                        if best.as_ref().is_none_or(|b| score > b.0) {
                            best = Some((score, pd, i, props));
                        }
                        break;
                    }
                }
            }
            let (_, physical, family, props) = best.ok_or("no Vulkan 1.3 device with dual-source blending and logic ops can draw to this window")?;
            let device_name = props.device_name_as_c_str().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();

            let priorities = [1.0f32];
            let queues = [vk::DeviceQueueCreateInfo::default().queue_family_index(family).queue_priorities(&priorities)];
            let device_extensions: Vec<*const c_char> = if surface.is_some() { vec![swapchain::NAME.as_ptr()] } else { Vec::new() };
            let features = vk::PhysicalDeviceFeatures::default().dual_src_blend(true).logic_op(true);
            let mut f12 = vk::PhysicalDeviceVulkan12Features::default()
                .descriptor_indexing(true)
                .shader_sampled_image_array_non_uniform_indexing(true)
                .runtime_descriptor_array(true)
                .descriptor_binding_partially_bound(true);
            let mut f13 = vk::PhysicalDeviceVulkan13Features::default().dynamic_rendering(true);
            let device = instance.create_device(
                physical,
                &vk::DeviceCreateInfo::default().queue_create_infos(&queues).enabled_extension_names(&device_extensions).enabled_features(&features).push_next(&mut f12).push_next(&mut f13),
                None,
            )?;
            let queue = device.get_device_queue(family, 0);
            let swapchain_fn = surface.is_some().then(|| swapchain::Device::new(&instance, &device));
            let mut alloc = Allocator::new(&AllocatorCreateDesc {
                instance: instance.clone(),
                device: device.clone(),
                physical_device: physical,
                debug_settings: Default::default(),
                buffer_device_address: false,
                allocation_sizes: Default::default(),
            })?;

            let pool = device.create_command_pool(&vk::CommandPoolCreateInfo::default().queue_family_index(family).flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER), None)?;
            let cmds = device.allocate_command_buffers(&vk::CommandBufferAllocateInfo::default().command_pool(pool).level(vk::CommandBufferLevel::PRIMARY).command_buffer_count(FRAMES_IN_FLIGHT as u32))?;

            // Descriptors: draw states, texture array, sampler array; and the copy pass's EFB views.
            let bindings = [
                vk::DescriptorSetLayoutBinding::default().binding(0).descriptor_type(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                vk::DescriptorSetLayoutBinding::default().binding(1).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(MAX_TEXTURES).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                vk::DescriptorSetLayoutBinding::default().binding(2).descriptor_type(vk::DescriptorType::SAMPLER).descriptor_count(MAX_SAMPLERS).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                // Frame interpolation (`Prepared`): model positions, matrix pairs, draw pairs.
                vk::DescriptorSetLayoutBinding::default().binding(3).descriptor_type(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1).stage_flags(vk::ShaderStageFlags::VERTEX),
                vk::DescriptorSetLayoutBinding::default().binding(4).descriptor_type(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1).stage_flags(vk::ShaderStageFlags::VERTEX),
                vk::DescriptorSetLayoutBinding::default().binding(5).descriptor_type(vk::DescriptorType::STORAGE_BUFFER).descriptor_count(1).stage_flags(vk::ShaderStageFlags::VERTEX),
            ];
            let partial = vk::DescriptorBindingFlags::PARTIALLY_BOUND;
            let flags = [vk::DescriptorBindingFlags::empty(), partial, partial, partial, partial, partial];
            let mut flag_info = vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
            let set_layout = device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings).push_next(&mut flag_info), None)?;
            let copy_bindings = [
                vk::DescriptorSetLayoutBinding::default().binding(0).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                vk::DescriptorSetLayoutBinding::default().binding(1).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
            ];
            let copy_set_layout = device.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&copy_bindings), None)?;
            let n = FRAMES_IN_FLIGHT as u32;
            let sizes = [
                vk::DescriptorPoolSize { ty: vk::DescriptorType::STORAGE_BUFFER, descriptor_count: 4 * n },
                vk::DescriptorPoolSize { ty: vk::DescriptorType::SAMPLED_IMAGE, descriptor_count: n * MAX_TEXTURES + 2 },
                vk::DescriptorPoolSize { ty: vk::DescriptorType::SAMPLER, descriptor_count: n * MAX_SAMPLERS },
            ];
            let desc_pool = device.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(n + 1).pool_sizes(&sizes), None)?;
            let layouts = [set_layout; FRAMES_IN_FLIGHT];
            let sets = device.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(desc_pool).set_layouts(&layouts))?;
            let copy_layouts = [copy_set_layout];
            let copy_set = device.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(desc_pool).set_layouts(&copy_layouts))?[0];
            let set_layouts = [set_layout];
            let vertex_push = [vk::PushConstantRange { stage_flags: vk::ShaderStageFlags::VERTEX, offset: 0, size: std::mem::size_of::<Push>() as u32 }];
            let pipeline_layout = device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts).push_constant_ranges(&vertex_push), None)?;
            let push = [vk::PushConstantRange { stage_flags: vk::ShaderStageFlags::FRAGMENT, offset: 0, size: std::mem::size_of::<CopyParams>() as u32 }];
            let copy_pipeline_layout = device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&copy_layouts).push_constant_ranges(&push), None)?;

            let cache_data = opts.pipeline_cache.as_ref().and_then(|p| std::fs::read(p).ok()).unwrap_or_default();
            let pipeline_cache = match device.create_pipeline_cache(&vk::PipelineCacheCreateInfo::default().initial_data(&cache_data), None) {
                Ok(c) => c,
                // A cache from another driver or GPU: start empty.
                Err(_) => device.create_pipeline_cache(&vk::PipelineCacheCreateInfo::default(), None)?,
            };

            let (vs_code, fs_code, early_code) = shaders::ubershader()?;
            let vs = device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&vs_code), None)?;
            let fs = device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&fs_code), None)?;
            let fs_early = device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&early_code), None)?;
            let (cvs_code, cfs_code) = shaders::copy()?;
            let copy_modules = [
                device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&cvs_code), None)?,
                device.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&cfs_code), None)?,
            ];
            let copy_pipeline = Self::create_copy_pipeline(&device, pipeline_cache, copy_pipeline_layout, copy_modules)?;

            let mut frames = Vec::new();
            for (i, cmd) in cmds.into_iter().enumerate() {
                frames.push(Frame {
                    cmd,
                    fence: device.create_fence(&vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED), None)?,
                    acquired: device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?,
                    set: sets[i],
                    vertices: None,
                    shadings: None,
                    staging: None,
                    readback: None,
                    writebacks: Vec::new(),
                    described: None,
                    retired_images: Vec::new(),
                    retired_buffers: Vec::new(),
                    retired_spare: Vec::new(),
                });
            }

            let (ew, eh) = (at_scale(EFB_W, scale), at_scale(EFB_H, scale));
            let usage = vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
            let efb_color = Self::create_image(&device, &mut alloc, ew, eh, 1, COLOR_FORMAT, usage, vk::ImageAspectFlags::COLOR)?;
            let usage = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
            let efb_depth = Self::create_image(&device, &mut alloc, ew, eh, 1, DEPTH_FORMAT, usage, vk::ImageAspectFlags::DEPTH)?;
            let usage = vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC;
            let native = Self::create_image(&device, &mut alloc, EFB_W, EFB_H, 1, COLOR_FORMAT, usage, vk::ImageAspectFlags::COLOR)?;
            let usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
            let palette = Self::create_image(&device, &mut alloc, PALETTE.len() as u32, 1, 1, COLOR_FORMAT, usage, vk::ImageAspectFlags::COLOR)?;

            Self::bind_efb(&device, copy_set, &efb_color, &efb_depth);

            let mut r = Self {
                device_name,
                validation,
                scale,
                stretch: 1.0,
                efb_stale: false,
                vsync: opts.vsync,
                copies_to_ram: opts.efb_copies_to_ram,
                _entry: entry,
                instance,
                debug,
                surface,
                physical,
                family,
                device,
                queue,
                alloc: ManuallyDrop::new(alloc),
                swapchain_fn,
                swapchain: vk::SwapchainKHR::null(),
                swap_images: Vec::new(),
                swap_views: Vec::new(),
                swap_format: vk::Format::UNDEFINED,
                ui: None,
                ui_frame: None,
                swap_extent: vk::Extent2D::default(),
                rendered: Vec::new(),
                needs_rebuild: true,
                upright: false,
                pool,
                frames,
                frame: 0,
                set_layout,
                copy_set_layout,
                desc_pool,
                pipeline_layout,
                pipeline_cache,
                pipeline_cache_path: opts.pipeline_cache,
                vs,
                fs,
                fs_early,
                copy_modules,
                copy: CopyPass { pipeline: copy_pipeline, layout: copy_pipeline_layout, set: copy_set },
                pipelines: HashMap::new(),
                efb_color,
                efb_depth,
                native,
                palette,
                efb_ready: false,
                index_offset: 0,
                push: [0; 2],
                prepared: None,
                spare: Vec::new(),
                trash_spare: Vec::new(),
                textures: HashMap::new(),
                textures_epoch: 0,
                samplers: HashMap::new(),
                xfbs: HashMap::new(),
                xfb_serial: 0,
                writebacks: Vec::new(),
                trash_images: Vec::new(),
                trash_buffers: Vec::new(),
                aspect: 16.0 / 9.0,
                screenshot: None,
                overlay: Vec::new(),
                stats: FrameStats::default(),
            };
            r.upload_palette()?;
            Ok(r)
        }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// EFB pixels to EFB-image pixels, across and down.
    fn sx(&self) -> f32 {
        self.scale * self.stretch
    }
    fn sy(&self) -> f32 {
        self.scale
    }

    /// The picture is `stretch` times as wide as the game's 16:9 (Ultrawide,
    /// 16:10): the EFB keeps 640 x 528 EFB pixels, each that much wider, so
    /// the picture keeps the resolution's pixels across. Its images are made
    /// again before the next frame.
    pub fn set_stretch(&mut self, stretch: f32) {
        if self.stretch != stretch {
            self.stretch = stretch;
            self.efb_stale = true;
        }
    }

    /// The copy pass reads `color` and `depth`.
    fn bind_efb(device: &Device, set: vk::DescriptorSet, color: &Image, depth: &Image) {
        let views = [
            vk::DescriptorImageInfo { sampler: vk::Sampler::null(), image_view: color.view, image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL },
            vk::DescriptorImageInfo { sampler: vk::Sampler::null(), image_view: depth.view, image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL },
        ];
        unsafe {
            device.update_descriptor_sets(
                &[
                    vk::WriteDescriptorSet::default().dst_set(set).dst_binding(0).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&views[..1]),
                    vk::WriteDescriptorSet::default().dst_set(set).dst_binding(1).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&views[1..]),
                ],
                &[],
            );
        }
    }

    /// New EFB images for the current stretch, once the GPU is done with
    /// the old ones; they start cleared, as at the start.
    fn remake_efb(&mut self) -> Res<()> {
        unsafe { self.device.device_wait_idle()? };
        let (ew, eh) = (at_scale(EFB_W, self.sx()), at_scale(EFB_H, self.sy()));
        let usage = vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
        let color = Self::create_image(&self.device, &mut self.alloc, ew, eh, 1, COLOR_FORMAT, usage, vk::ImageAspectFlags::COLOR)?;
        let usage = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
        let depth = Self::create_image(&self.device, &mut self.alloc, ew, eh, 1, DEPTH_FORMAT, usage, vk::ImageAspectFlags::DEPTH)?;
        Self::bind_efb(&self.device, self.copy.set, &color, &depth);
        let old = [std::mem::replace(&mut self.efb_color, color), std::mem::replace(&mut self.efb_depth, depth)];
        for img in old {
            self.destroy_image(img);
        }
        self.efb_ready = false;
        self.efb_stale = false;
        Ok(())
    }

    /// Textures held, and GPU memory allocated and reserved (bytes), for
    /// diagnostics.
    pub fn memory(&self) -> (usize, u64, u64) {
        let r = self.alloc.generate_report();
        (self.textures.len(), r.total_allocated_bytes, r.total_capacity_bytes)
    }

    /// Switches V-Sync; the swapchain is rebuilt before the next present.
    pub fn set_vsync(&mut self, on: bool) {
        if self.vsync != on {
            self.vsync = on;
            self.needs_rebuild = true;
        }
    }

    unsafe fn create_copy_pipeline(device: &Device, cache: vk::PipelineCache, layout: vk::PipelineLayout, modules: [vk::ShaderModule; 2]) -> Res<vk::Pipeline> {
        let stages = [
            vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(modules[0]).name(c"vs"),
            vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(modules[1]).name(c"fs"),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL).line_width(1.0);
        let ms = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let atts = [vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA)];
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&atts);
        let dynamic = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dyn_info = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic);
        let formats = [COLOR_FORMAT];
        let mut rendering = vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&assembly)
            .viewport_state(&viewport)
            .rasterization_state(&raster)
            .multisample_state(&ms)
            .color_blend_state(&blend)
            .dynamic_state(&dyn_info)
            .layout(layout)
            .push_next(&mut rendering);
        Ok(device.create_graphics_pipelines(cache, &[info], None).map_err(|(_, e)| e)?[0])
    }

    #[allow(clippy::too_many_arguments)]
    fn create_image(device: &Device, alloc: &mut Allocator, w: u32, h: u32, levels: u32, format: vk::Format, usage: vk::ImageUsageFlags, aspect: vk::ImageAspectFlags) -> Res<Image> {
        unsafe {
            let image = device.create_image(
                &vk::ImageCreateInfo::default()
                    .image_type(vk::ImageType::TYPE_2D)
                    .format(format)
                    .extent(vk::Extent3D { width: w, height: h, depth: 1 })
                    .mip_levels(levels)
                    .array_layers(1)
                    .samples(vk::SampleCountFlags::TYPE_1)
                    .tiling(vk::ImageTiling::OPTIMAL)
                    .usage(usage)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .initial_layout(vk::ImageLayout::UNDEFINED),
                None,
            )?;
            let req = device.get_image_memory_requirements(image);
            let a = alloc.allocate(&AllocationCreateDesc { name: "image", requirements: req, location: MemoryLocation::GpuOnly, linear: false, allocation_scheme: AllocationScheme::GpuAllocatorManaged })?;
            device.bind_image_memory(image, a.memory(), a.offset())?;
            // Images only ever copied or blitted (XFBs, the palette) get no
            // view: one is not allowed without a usage it could serve.
            let viewed = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;
            let view = if usage.intersects(viewed) {
                device.create_image_view(
                    &vk::ImageViewCreateInfo::default().image(image).view_type(vk::ImageViewType::TYPE_2D).format(format).subresource_range(range(aspect, levels)),
                    None,
                )?
            } else {
                vk::ImageView::null()
            };
            Ok(Image { image, view, alloc: Some(a), w, h, levels, aspect, layout: vk::ImageLayout::UNDEFINED })
        }
    }

    fn create_buffer(&mut self, size: u64, usage: vk::BufferUsageFlags, location: MemoryLocation) -> Res<Buffer> {
        unsafe {
            let buffer = self.device.create_buffer(&vk::BufferCreateInfo::default().size(size).usage(usage).sharing_mode(vk::SharingMode::EXCLUSIVE), None)?;
            let req = self.device.get_buffer_memory_requirements(buffer);
            let a = self.alloc.allocate(&AllocationCreateDesc { name: "buffer", requirements: req, location, linear: true, allocation_scheme: AllocationScheme::GpuAllocatorManaged })?;
            self.device.bind_buffer_memory(buffer, a.memory(), a.offset())?;
            Ok(Buffer { buffer, alloc: Some(a), size })
        }
    }

    fn destroy_image(&mut self, mut img: Image) {
        unsafe {
            self.device.destroy_image_view(img.view, None);
            self.device.destroy_image(img.image, None);
        }
        if let Some(a) = img.alloc.take() {
            let _ = self.alloc.free(a);
        }
    }

    fn destroy_buffer(&mut self, mut b: Buffer) {
        unsafe { self.device.destroy_buffer(b.buffer, None) };
        if let Some(a) = b.alloc.take() {
            let _ = self.alloc.free(a);
        }
    }

    fn slot(&mut self, f: usize, which: u8) -> &mut Option<Buffer> {
        match which {
            0 => &mut self.frames[f].vertices,
            1 => &mut self.frames[f].shadings,
            2 => &mut self.frames[f].staging,
            _ => &mut self.frames[f].readback,
        }
    }

    /// A host-visible buffer of at least `size` bytes in `slot` of frame `f`
    /// (0 vertices, 1 draw states, 2 upload staging, 3 readback), reused
    /// while it is big enough.
    fn frame_buffer(&mut self, f: usize, which: u8, size: u64, usage: vk::BufferUsageFlags) -> Res<vk::Buffer> {
        if let Some(b) = self.slot(f, which).as_ref().filter(|b| b.size >= size) {
            return Ok(b.buffer);
        }
        if let Some(old) = self.slot(f, which).take() {
            self.trash_buffers.push(old);
        }
        let location = if which == 3 { MemoryLocation::GpuToCpu } else { MemoryLocation::CpuToGpu };
        let b = self.create_buffer(size.next_power_of_two().max(4096), usage, location)?;
        let handle = b.buffer;
        *self.slot(f, which) = Some(b);
        Ok(handle)
    }

    fn frame_bytes(&mut self, f: usize, which: u8) -> &mut [u8] {
        self.slot(f, which).as_mut().and_then(|b| b.alloc.as_mut()).and_then(|a| a.mapped_slice_mut()).expect("host-visible buffer")
    }

    fn sampler(&mut self, key: u32) -> Res<vk::Sampler> {
        if let Some(&s) = self.samplers.get(&key) {
            return Ok(s);
        }
        let wrap = |w: u32| match w & 3 {
            0 => vk::SamplerAddressMode::CLAMP_TO_EDGE,
            1 => vk::SamplerAddressMode::REPEAT,
            _ => vk::SamplerAddressMode::MIRRORED_REPEAT,
        };
        let mag = if (key >> 4) & 1 != 0 { vk::Filter::LINEAR } else { vk::Filter::NEAREST };
        let min = (key >> 5) & 7;
        let min_filter = if min >= 4 { vk::Filter::LINEAR } else { vk::Filter::NEAREST };
        // A texture pack's image (bit 24) has its own mip levels, however
        // many: a linear-filtered texture without mips blends between them.
        let replaced = key & (1 << 24) != 0;
        let mip = if min & 3 == 2 || (replaced && min == 4) { vk::SamplerMipmapMode::LINEAR } else { vk::SamplerMipmapMode::NEAREST };
        let max_lod = if replaced { vk::LOD_CLAMP_NONE } else if min & 3 != 0 { ((key >> 20) & 0xF) as f32 } else { 0.0 };
        // LOD bias: s2.5 fixed point in bits 9..16.
        let bias = ((((key >> 9) & 0xFF) as u8) as i8) as f32 / 32.0;
        let s = unsafe {
            self.device.create_sampler(
                &vk::SamplerCreateInfo::default()
                    .mag_filter(mag)
                    .min_filter(min_filter)
                    .mipmap_mode(mip)
                    .address_mode_u(wrap(key))
                    .address_mode_v(wrap(key >> 2))
                    .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .mip_lod_bias(bias)
                    .min_lod(0.0)
                    .max_lod(max_lod),
                None,
            )?
        };
        self.samplers.insert(key, s);
        Ok(s)
    }

    fn pipeline(&mut self, key: u32) -> Res<vk::Pipeline> {
        if let Some(&p) = self.pipelines.get(&key) {
            return Ok(p);
        }
        let efb_alpha = key & (1 << 16) != 0;
        let (blend_on, logic_on, subtract) = (key & 1 != 0, key & 2 != 0, key & (1 << 11) != 0);
        let mut mask = vk::ColorComponentFlags::empty();
        if key & 8 != 0 {
            mask |= vk::ColorComponentFlags::R | vk::ColorComponentFlags::G | vk::ColorComponentFlags::B;
        }
        if key & 0x10 != 0 {
            mask |= vk::ColorComponentFlags::A;
        }
        let mut att = vk::PipelineColorBlendAttachmentState::default().color_write_mask(mask);
        // GX precedence: subtract, then blending, then the logic op.
        let mut logic = None;
        if subtract {
            att = att
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::ONE)
                .dst_color_blend_factor(vk::BlendFactor::ONE)
                .color_blend_op(vk::BlendOp::REVERSE_SUBTRACT)
                .src_alpha_blend_factor(vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(vk::BlendFactor::ONE)
                .alpha_blend_op(vk::BlendOp::REVERSE_SUBTRACT);
        } else if blend_on {
            let src = blend_factor((key >> 8) & 7, true, efb_alpha);
            let dst = blend_factor((key >> 5) & 7, false, efb_alpha);
            att = att
                .blend_enable(true)
                .src_color_blend_factor(src)
                .dst_color_blend_factor(dst)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(src)
                .dst_alpha_blend_factor(dst)
                .alpha_blend_op(vk::BlendOp::ADD);
        } else if logic_on {
            // GX and Vulkan list the sixteen logic ops in the same order.
            logic = Some(vk::LogicOp::from_raw(((key >> 12) & 0xF) as i32));
        }
        let atts = [att];

        let binding = [vk::VertexInputBindingDescription { binding: 0, stride: VERTEX_SIZE, input_rate: vk::VertexInputRate::VERTEX }];
        let mut attrs = vec![
            vk::VertexInputAttributeDescription { location: 0, binding: 0, format: vk::Format::R32G32B32A32_SFLOAT, offset: 0 },
            vk::VertexInputAttributeDescription { location: 1, binding: 0, format: vk::Format::R32G32B32A32_SFLOAT, offset: 16 },
            vk::VertexInputAttributeDescription { location: 2, binding: 0, format: vk::Format::R32G32B32A32_SFLOAT, offset: 32 },
        ];
        for i in 0..8 {
            attrs.push(vk::VertexInputAttributeDescription { location: 3 + i, binding: 0, format: vk::Format::R32G32B32_SFLOAT, offset: 48 + 12 * i });
        }
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default().vertex_binding_descriptions(&binding).vertex_attribute_descriptions(&attrs);
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL).line_width(1.0);
        let ms = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let ds = vk::PipelineDepthStencilStateCreateInfo::default();
        let mut blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&atts);
        if let Some(op) = logic {
            blend = blend.logic_op_enable(true).logic_op(op);
        }
        let dynamic = [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::CULL_MODE,
            vk::DynamicState::FRONT_FACE,
            vk::DynamicState::DEPTH_TEST_ENABLE,
            vk::DynamicState::DEPTH_WRITE_ENABLE,
            vk::DynamicState::DEPTH_COMPARE_OP,
        ];
        let dyn_info = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic);
        let formats = [COLOR_FORMAT];
        let mut rendering = vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats).depth_attachment_format(DEPTH_FORMAT);
        let stages = [
            vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(self.vs).name(c"vs"),
            if key & EARLY_DEPTH != 0 {
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(self.fs_early).name(c"fs_early")
            } else {
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(self.fs).name(c"fs")
            },
        ];
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&assembly)
            .viewport_state(&viewport)
            .rasterization_state(&raster)
            .multisample_state(&ms)
            .depth_stencil_state(&ds)
            .color_blend_state(&blend)
            .dynamic_state(&dyn_info)
            .layout(self.pipeline_layout)
            .push_next(&mut rendering);
        let p = unsafe { self.device.create_graphics_pipelines(self.pipeline_cache, &[info], None).map_err(|(_, e)| e)?[0] };
        self.pipelines.insert(key, p);
        Ok(p)
    }

    /// What the menus draw over the frames shown from now on (`None`: no
    /// menu), with egui's texture changes: new and changed textures are
    /// uploaded now; freed ones go once the GPU is done with them.
    pub fn set_ui(&mut self, frame: Option<UiFrame>, textures: &egui::TexturesDelta) -> Res<()> {
        if self.ui.as_ref().is_none_or(|u| u.format != self.swap_format) && self.swap_format != vk::Format::UNDEFINED && !self.swap_views.is_empty() {
            unsafe { self.device.device_wait_idle()? };
            self.ui = None;
            let mode = egui_ash_renderer::RenderMode::DynamicRendering(egui_ash_renderer::DynamicRendering { color_attachment_format: self.swap_format, depth_attachment_format: None, stencil_attachment_format: None });
            // The swapchain is UNORM: egui's shader outputs sRGB itself.
            let options = egui_ash_renderer::Options { in_flight_frames: FRAMES_IN_FLIGHT, enable_depth_test: false, enable_depth_write: false, srgb_framebuffer: false };
            let renderer = egui_ash_renderer::Renderer::with_default_allocator(&self.instance, self.physical, self.device.clone(), mode, options).map_err(|e| format!("menus: {e}"))?;
            self.ui = Some(UiLayer { renderer, format: self.swap_format });
        }
        if let Some(ui) = &mut self.ui {
            for (id, deltas) in &textures.set {
                for delta in deltas {
                    ui.renderer.set_texture(self.queue, self.pool, *id, delta).map_err(|e| format!("menus: {e}"))?;
                }
            }
            if !textures.free.is_empty() {
                unsafe { self.device.device_wait_idle()? };
                for id in &textures.free {
                    ui.renderer.free_texture(*id).map_err(|e| format!("menus: {e}"))?;
                }
            }
        }
        self.ui_frame = frame;
        Ok(())
    }

    pub fn resized(&mut self) {
        self.needs_rebuild = true;
    }

    /// Whether frames can be presented: made with a window that is attached.
    fn has_window(&self) -> bool {
        self.surface.as_ref().is_some_and(|(_, s)| *s != vk::SurfaceKHR::null())
    }

    /// Lets go of the window (Android destroys an app's surface when it
    /// leaves the screen): its swapchain and surface are destroyed, and
    /// frames are rendered without being presented until
    /// [`Renderer::attach_window`]. Textures and every other GPU resource stay.
    pub fn detach_window(&mut self) {
        let Some((surface_fn, surface)) = &mut self.surface else { return };
        unsafe {
            let _ = self.device.device_wait_idle();
            if let Some(f) = &self.swapchain_fn {
                if self.swapchain != vk::SwapchainKHR::null() {
                    f.destroy_swapchain(self.swapchain, None);
                }
            }
            self.swapchain = vk::SwapchainKHR::null();
            self.swap_images.clear();
            if *surface != vk::SurfaceKHR::null() {
                surface_fn.destroy_surface(*surface, None);
                *surface = vk::SurfaceKHR::null();
            }
        }
    }

    /// Presents to `window` from now on (after [`Renderer::detach_window`],
    /// or in place of the window it was made with). Only for a renderer made
    /// with a window: a headless one has no surface or swapchain support.
    pub fn attach_window(&mut self, display: RawDisplayHandle, window: RawWindowHandle) -> Res<()> {
        self.detach_window();
        let Some((surface_fn, surface)) = &mut self.surface else { return Err("a headless renderer cannot present to a window".into()) };
        unsafe {
            let s = ash_window::create_surface(&self._entry, &self.instance, display, window, None)?;
            if !surface_fn.get_physical_device_surface_support(self.physical, self.family, s)? {
                surface_fn.destroy_surface(s, None);
                return Err("the GPU's queue cannot present to this window".into());
            }
            *surface = s;
        }
        self.needs_rebuild = true;
        Ok(())
    }

    fn rebuild(&mut self, width: u32, height: u32) -> Res<()> {
        let (Some((surface_fn, surface)), Some(swapchain_fn)) = (&self.surface, &self.swapchain_fn) else {
            return Ok(());
        };
        if *surface == vk::SurfaceKHR::null() {
            return Ok(());
        }
        unsafe {
            self.device.device_wait_idle()?;
            let caps = surface_fn.get_physical_device_surface_capabilities(self.physical, *surface)?;
            if !caps.supported_usage_flags.contains(vk::ImageUsageFlags::TRANSFER_DST) {
                return Err("swapchain images cannot be written by transfers (no TRANSFER_DST usage)".into());
            }
            // The menus are drawn into the images too, where the surface allows it.
            let usage = vk::ImageUsageFlags::TRANSFER_DST | (caps.supported_usage_flags & vk::ImageUsageFlags::COLOR_ATTACHMENT);
            // A display turned from its natural orientation (Android: a
            // landscape game on a phone whose panel is portrait) reports the
            // turn as the current transform, with the extent already the
            // window's own (landscape). The swapchain stays upright
            // (identity) at that extent and the compositor turns it, instead
            // of the frames being drawn turned.
            let upright = caps.current_transform != vk::SurfaceTransformFlagsKHR::IDENTITY && caps.supported_transforms.contains(vk::SurfaceTransformFlagsKHR::IDENTITY);
            let extent = if caps.current_extent.width != u32::MAX {
                caps.current_extent
            } else {
                vk::Extent2D {
                    width: width.clamp(caps.min_image_extent.width, caps.max_image_extent.width),
                    height: height.clamp(caps.min_image_extent.height, caps.max_image_extent.height),
                }
            };
            let formats = surface_fn.get_physical_device_surface_formats(self.physical, *surface)?;
            let format = formats
                .iter()
                .find(|f| f.format == vk::Format::B8G8R8A8_UNORM && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
                .or(formats.first())
                .copied()
                .ok_or("the surface reports no formats")?;
            let alpha = [vk::CompositeAlphaFlagsKHR::OPAQUE, vk::CompositeAlphaFlagsKHR::INHERIT, vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED, vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED]
                .into_iter()
                .find(|a| caps.supported_composite_alpha.contains(*a))
                .ok_or("the surface supports no composite alpha mode")?;
            // V-Sync off: mailbox (no tearing) where offered, else immediate.
            let modes = surface_fn.get_physical_device_surface_present_modes(self.physical, *surface)?;
            let mode = if self.vsync {
                vk::PresentModeKHR::FIFO
            } else {
                [vk::PresentModeKHR::MAILBOX, vk::PresentModeKHR::IMMEDIATE].into_iter().find(|m| modes.contains(m)).unwrap_or(vk::PresentModeKHR::FIFO)
            };
            let mut count = caps.min_image_count + 1;
            if caps.max_image_count > 0 {
                count = count.min(caps.max_image_count);
            }
            let old = self.swapchain;
            self.swapchain = swapchain_fn.create_swapchain(
                &vk::SwapchainCreateInfoKHR::default()
                    .surface(*surface)
                    .min_image_count(count)
                    .image_format(format.format)
                    .image_color_space(format.color_space)
                    .image_extent(extent)
                    .image_array_layers(1)
                    .image_usage(usage)
                    .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .pre_transform(if upright { vk::SurfaceTransformFlagsKHR::IDENTITY } else { caps.current_transform })
                    .composite_alpha(alpha)
                    .present_mode(mode)
                    .clipped(true)
                    .old_swapchain(old),
                None,
            )?;
            if old != vk::SwapchainKHR::null() {
                swapchain_fn.destroy_swapchain(old, None);
            }
            for v in self.swap_views.drain(..) {
                self.device.destroy_image_view(v, None);
            }
            self.swap_images = swapchain_fn.get_swapchain_images(self.swapchain)?;
            self.swap_extent = extent;
            self.swap_format = format.format;
            if usage.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT) {
                for &image in &self.swap_images {
                    let info = vk::ImageViewCreateInfo::default().image(image).view_type(vk::ImageViewType::TYPE_2D).format(format.format).subresource_range(range(vk::ImageAspectFlags::COLOR, 1));
                    self.swap_views.push(self.device.create_image_view(&info, None)?);
                }
            }
            self.upright = upright;
            for s in self.rendered.drain(..) {
                self.device.destroy_semaphore(s, None);
            }
            for _ in 0..self.swap_images.len() {
                self.rendered.push(self.device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?);
            }
        }
        self.needs_rebuild = false;
        Ok(())
    }

    /// Records commands into a temporary command buffer, submits and waits.
    fn one_shot(&mut self, record: impl FnOnce(&mut Self, vk::CommandBuffer) -> Res<()>) -> Res<()> {
        unsafe {
            let cmd = self.device.allocate_command_buffers(&vk::CommandBufferAllocateInfo::default().command_pool(self.pool).level(vk::CommandBufferLevel::PRIMARY).command_buffer_count(1))?[0];
            self.device.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT))?;
            record(self, cmd)?;
            self.device.end_command_buffer(cmd)?;
            let cmds = [cmd];
            self.device.queue_submit(self.queue, &[vk::SubmitInfo::default().command_buffers(&cmds)], vk::Fence::null())?;
            self.device.queue_wait_idle(self.queue)?;
            self.device.free_command_buffers(self.pool, &cmds);
        }
        Ok(())
    }

    fn upload_palette(&mut self) -> Res<()> {
        let mut staging = self.create_buffer(64, vk::BufferUsageFlags::TRANSFER_SRC, MemoryLocation::CpuToGpu)?;
        let bytes: Vec<u8> = PALETTE.iter().flatten().copied().collect();
        staging.alloc.as_mut().and_then(|a| a.mapped_slice_mut()).ok_or("staging not mapped")?[..bytes.len()].copy_from_slice(&bytes);
        let buffer = staging.buffer;
        self.one_shot(|r, cmd| unsafe {
            transition(&r.device, cmd, &mut r.palette, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
            let region = vk::BufferImageCopy::default().image_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0)).image_extent(vk::Extent3D { width: PALETTE.len() as u32, height: 1, depth: 1 });
            r.device.cmd_copy_buffer_to_image(cmd, buffer, r.palette.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[region]);
            transition(&r.device, cmd, &mut r.palette, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
            Ok(())
        })?;
        self.destroy_buffer(staging);
        Ok(())
    }

    /// Executes `out` into the EFB with `vertices` (normally
    /// `out.vertices`; frame interpolation passes moved ones), then
    /// presents the XFB at `xfb_addr` (or the latest one) to a `width` x
    /// `height` window; a zero size presents nothing. `fresh` is true the
    /// first time this output is rendered: only then are its textures
    /// uploaded and EFB copies read back for RAM. Evictions are separate
    /// ([`Renderer::evict`]).
    /// Returns false when nothing was presented.
    pub fn render(&mut self, out: &Output, vertices: &[Vertex], fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        self.render_with(out, vertices.len(), &mut |dst| dst.copy_from_slice(vertices), fresh, xfb_addr, width, height)
    }

    /// [`Renderer::render`] with `fill` writing the `count` vertices straight
    /// into the frame's GPU buffer (frame interpolation builds them there,
    /// saving a copy of every vertex). `fill` should only write.
    #[allow(clippy::too_many_arguments)]
    pub fn render_with(&mut self, out: &Output, count: usize, fill: &mut dyn FnMut(&mut [Vertex]), fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let f = self.frame;
        let dev = self.device.clone();
        unsafe {
            dev.wait_for_fences(&[self.frames[f].fence], true, u64::MAX)?;
        }
        if self.efb_stale {
            self.remake_efb()?;
        }
        self.collect_writebacks(f);
        // What was retired before this frame's previous submission is no
        // longer in use by any frame.
        for img in std::mem::take(&mut self.frames[f].retired_images) {
            self.destroy_image(img);
        }
        for b in std::mem::take(&mut self.frames[f].retired_buffers) {
            self.destroy_buffer(b);
        }
        self.reuse_spares(f);
        let cmd = self.frames[f].cmd;
        unsafe {
            dev.reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())?;
            dev.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT))?;
        }
        if fresh {
            self.upload_textures(f, cmd, out)?;
            // Images that EFB copies will fill.
            for c in &out.cmds {
                if let Cmd::CopyTex(c) = c {
                    let (w, h) = (at_scale(c.w, self.sx()), at_scale(c.h, self.sy()));
                    if self.textures.get(&c.id).is_some_and(|i| i.w == w && i.h == h) {
                        continue;
                    }
                    let usage = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC;
                    let img = Self::create_image(&dev, &mut self.alloc, w, h, 1, COLOR_FORMAT, usage, vk::ImageAspectFlags::COLOR)?;
                    self.textures_epoch += 1;
                    if let Some(old) = self.textures.insert(c.id, img) {
                        self.trash_images.push(old);
                    }
                }
            }
        } else {
            self.stats.uploads = 0;
        }
        self.write_descriptors(f, out)?;
        // Vertices, then the triangle-list indices, in one buffer.
        let vlen = count as u64 * u64::from(VERTEX_SIZE);
        self.index_offset = vlen.next_multiple_of(4);
        let total = (self.index_offset + out.indices.len() as u64 * 4).max(VERTEX_SIZE as u64);
        let vb = self.frame_buffer(f, 0, total, vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER)?;
        {
            let at = self.index_offset as usize;
            let dst = self.frame_bytes(f, 0);
            let bytes = &mut dst[..count * std::mem::size_of::<Vertex>()];
            assert!(bytes.as_ptr().align_offset(std::mem::align_of::<Vertex>()) == 0, "vertex buffer misaligned");
            // SAFETY: the mapped buffer is aligned for `Vertex` (checked), large
            // enough (sized above), and `Vertex` is plain floats.
            let verts = unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast::<Vertex>(), count) };
            fill(verts);
            let idx = as_bytes(&out.indices);
            dst[at..at + idx.len()].copy_from_slice(idx);
        }
        self.record_efb(f, cmd, vb, out, fresh)?;
        let presented = self.present(f, cmd, xfb_addr, width, height)?;
        self.frame = (f + 1) % FRAMES_IN_FLIGHT;
        Ok(presented)
    }

    /// Frame `f` is done: its retired prepared-field buffers can be reused.
    /// A few are kept (the largest); the rest are freed.
    fn reuse_spares(&mut self, f: usize) {
        const KEEP: usize = 3;
        let back = std::mem::take(&mut self.frames[f].retired_spare);
        self.spare.extend(back);
        if self.spare.len() > KEEP {
            self.spare.sort_by_key(|b| std::cmp::Reverse(b.size));
            for b in self.spare.split_off(KEEP) {
                self.destroy_buffer(b);
            }
        }
    }

    /// The key of the field on the GPU for in-between frames, if any.
    pub fn prepared(&self) -> Option<(u64, u64)> {
        self.prepared.as_ref().map(|p| p.key)
    }

    /// Keeps `out` on the GPU for [`Renderer::render_prepared`], under `key`
    /// (the two fields' indices), with `moves` placing its paired draws in
    /// between (`gx::interp::moves`; `None`: shown as it is). Its textures
    /// must have been uploaded (a `fresh` render of it).
    pub fn prepare(&mut self, key: (u64, u64), out: &Output, moves: Option<&gx::interp::Moves>) -> Res<()> {
        self.prepare_with(key, out, &out.vertices, moves)
    }

    /// [`Renderer::prepare`] with `vertices` in place of the field's own
    /// (laid out like them: a player's window on it).
    pub fn prepare_with(&mut self, key: (u64, u64), out: &Output, vertices: &[Vertex], moves: Option<&gx::interp::Moves>) -> Res<()> {
        // Storage buffer offsets: 256 bytes is the largest alignment Vulkan allows.
        const ALIGN: u64 = 256;
        let vlen = (vertices.len() * VERTEX_SIZE as usize) as u64;
        let index_offset = vlen.next_multiple_of(ALIGN);
        let mut at = index_offset + (out.indices.len() as u64 * 4);
        let mut ranges = [(0u64, 0u64); 3];
        let parts: Option<[&[u8]; 3]> = moves.filter(|m| !m.mats.is_empty()).map(|m| [as_bytes(&m.model), as_bytes(&m.mats), as_bytes(&m.recs)]);
        if let Some(parts) = &parts {
            for (r, part) in ranges.iter_mut().zip(parts) {
                at = at.next_multiple_of(ALIGN);
                *r = (at, (part.len() as u64).max(16));
                at += r.1;
            }
        }
        let need = at.max(16);
        let mut buffer = match self.spare.iter().position(|b| b.size >= need) {
            Some(i) => self.spare.swap_remove(i),
            None => {
                let usage = vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER;
                // Rounded up, so the next fields fit in it too.
                self.create_buffer(need.next_power_of_two(), usage, MemoryLocation::CpuToGpu)?
            }
        };
        {
            let dst = buffer.alloc.as_mut().and_then(|a| a.mapped_slice_mut()).ok_or("prepared buffer not mapped")?;
            dst[..vlen as usize].copy_from_slice(as_bytes(vertices));
            let idx = as_bytes(&out.indices);
            dst[index_offset as usize..index_offset as usize + idx.len()].copy_from_slice(idx);
            if let Some(parts) = &parts {
                for ((offset, _), part) in ranges.iter().zip(parts) {
                    dst[*offset as usize..*offset as usize + part.len()].copy_from_slice(part);
                }
            }
        }
        let serial = self.prepared.as_ref().map_or(0, |p| p.serial + 1);
        if let Some(old) = self.prepared.replace(Prepared { key, serial, buffer, index_offset, moves: parts.map(|_| ranges) }) {
            // Frames in flight may still read it: reused once they are done.
            self.trash_spare.push(old.buffer);
        }
        Ok(())
    }

    /// Renders the prepared field ([`Renderer::prepare`]; `out` is that
    /// field) at fraction `t` between the field before and it, and presents
    /// it like [`Renderer::render`]. Per frame shown, the CPU only records
    /// the draws: the vertices are already on the GPU, the vertex shader
    /// moves them, and the draw states and descriptors are written again only
    /// when the field or the textures change.
    pub fn render_prepared(&mut self, out: &Output, t: f32, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let f = self.frame;
        let dev = self.device.clone();
        unsafe {
            dev.wait_for_fences(&[self.frames[f].fence], true, u64::MAX)?;
        }
        if self.efb_stale {
            self.remake_efb()?;
        }
        self.collect_writebacks(f);
        for img in std::mem::take(&mut self.frames[f].retired_images) {
            self.destroy_image(img);
        }
        for b in std::mem::take(&mut self.frames[f].retired_buffers) {
            self.destroy_buffer(b);
        }
        self.reuse_spares(f);
        let cmd = self.frames[f].cmd;
        unsafe {
            dev.reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())?;
            dev.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT))?;
        }
        self.stats.uploads = 0;
        let p = self.prepared.as_ref().ok_or("render_prepared: nothing prepared")?;
        let (serial, vb, index_offset, moves) = (p.serial, p.buffer.buffer, p.index_offset, p.moves);
        let described = (serial, self.textures_epoch);
        if self.frames[f].described != Some(described) {
            self.write_draw_states(f, out, moves.map(|m| (vb, m)))?;
            self.frames[f].described = Some(described);
        }
        self.index_offset = index_offset;
        self.push = [t.to_bits(), u32::from(moves.is_some())];
        let recorded = self.record_efb(f, cmd, vb, out, false);
        self.push = [0; 2];
        recorded?;
        let presented = self.present(f, cmd, xfb_addr, width, height)?;
        self.frame = (f + 1) % FRAMES_IN_FLIGHT;
        Ok(presented)
    }

    /// Drops textures the GX texture cache evicted (`Output::evicted`). Call
    /// it once no output still to be rendered refers to them.
    pub fn evict(&mut self, ids: &[u32]) {
        for id in ids {
            if let Some(img) = self.textures.remove(id) {
                self.textures_epoch += 1;
                self.trash_images.push(img);
            }
        }
    }

    /// EFB copies read back so far, for the frontend to write to RAM.
    pub fn take_writebacks(&mut self) -> Vec<Writeback> {
        std::mem::take(&mut self.writebacks)
    }

    /// Waits for the GPU and collects every outstanding writeback.
    pub fn finish(&mut self) -> Res<()> {
        unsafe { self.device.device_wait_idle()? };
        for f in 0..FRAMES_IN_FLIGHT {
            self.collect_writebacks(f);
        }
        Ok(())
    }

    /// Frame `f`'s readbacks, now that its fence has signalled.
    fn collect_writebacks(&mut self, f: usize) {
        let pending = std::mem::take(&mut self.frames[f].writebacks);
        if pending.is_empty() {
            return;
        }
        let bytes = self.frame_bytes(f, 3).to_vec();
        for p in pending {
            let len = (p.w * p.h * 4) as usize;
            self.writebacks.push(Writeback { addr: p.addr, ram_hash: p.ram_hash, tex_format: p.tex_format, width: p.w, height: p.h, stride: p.stride, rgba: bytes[p.offset..p.offset + len].to_vec() });
        }
    }

    /// The EFB at native resolution (waits for the GPU).
    pub fn efb_snapshot(&mut self) -> Res<EfbSnapshot> {
        unsafe { self.device.device_wait_idle()? };
        let bytes = u64::from(EFB_W * EFB_H * 4);
        let color_buf = self.create_buffer(bytes, vk::BufferUsageFlags::TRANSFER_DST, MemoryLocation::GpuToCpu)?;
        let depth_buf = self.create_buffer(bytes, vk::BufferUsageFlags::TRANSFER_DST, MemoryLocation::GpuToCpu)?;
        let (cb, db) = (color_buf.buffer, depth_buf.buffer);
        let full = Rect { x: 0, y: 0, w: EFB_W, h: EFB_H };
        self.one_shot(|r, cmd| {
            let src = r.scaled_f(full);
            for (flags, buf) in [(4u32, cb), (2 | 4, db)] {
                let params = CopyParams { src, dst: [EFB_W as f32, EFB_H as f32], format: 6, flags };
                r.copy_to_native(cmd, params, buf, 0);
            }
            Ok(())
        })?;
        let read = |b: &mut Buffer| b.alloc.as_mut().and_then(|a| a.mapped_slice_mut()).map(|s| s[..bytes as usize].to_vec()).unwrap_or_default();
        let (mut color_buf, mut depth_buf) = (color_buf, depth_buf);
        let color = read(&mut color_buf);
        let d = read(&mut depth_buf);
        self.destroy_buffer(color_buf);
        self.destroy_buffer(depth_buf);
        let depth = d.chunks_exact(4).map(|p| u32::from(p[0]) << 16 | u32::from(p[1]) << 8 | u32::from(p[2])).collect();
        Ok(EfbSnapshot { color, depth })
    }

    fn upload_textures(&mut self, f: usize, cmd: vk::CommandBuffer, out: &Output) -> Res<()> {
        self.stats.uploads = out.new_textures.len();
        // A texture pack's image instead of the decoded texture, at its own size.
        fn source(t: &gx::texture::NewTexture) -> (u32, u32, &[Vec<u8>]) {
            match &t.replacement {
                Some(r) => (r.width, r.height, r.levels.as_slice()),
                None => (t.width, t.height, t.levels.as_slice()),
            }
        }
        let total: usize = out.new_textures.iter().flat_map(|t| source(t).2).map(Vec::len).sum();
        if total == 0 {
            return Ok(());
        }
        let staging = self.frame_buffer(f, 2, total as u64, vk::BufferUsageFlags::TRANSFER_SRC)?;
        let mut copies = Vec::new();
        {
            let bytes = self.frame_bytes(f, 2);
            let mut at = 0;
            for t in &out.new_textures {
                let mut regions = Vec::new();
                let (width, height, levels) = source(t);
                let (mut w, mut h) = (width, height);
                for (level, data) in levels.iter().enumerate() {
                    bytes[at..at + data.len()].copy_from_slice(data);
                    regions.push(
                        vk::BufferImageCopy::default()
                            .buffer_offset(at as u64)
                            .image_subresource(sub_layers(vk::ImageAspectFlags::COLOR, level as u32))
                            .image_extent(vk::Extent3D { width: w, height: h, depth: 1 }),
                    );
                    at += data.len();
                    w = (w / 2).max(1);
                    h = (h / 2).max(1);
                }
                copies.push((t.id, width, height, levels.len() as u32, regions));
            }
        }
        let dev = self.device.clone();
        for (id, w, h, levels, regions) in copies {
            let usage = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
            let mut img = Self::create_image(&dev, &mut self.alloc, w, h, levels, COLOR_FORMAT, usage, vk::ImageAspectFlags::COLOR)?;
            unsafe {
                transition(&dev, cmd, &mut img, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
                dev.cmd_copy_buffer_to_image(cmd, staging, img.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &regions);
                transition(&dev, cmd, &mut img, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            }
            self.textures_epoch += 1;
            if let Some(old) = self.textures.insert(id, img) {
                self.trash_images.push(old);
            }
        }
        // A texture pack's images at a course's start can make a staging
        // buffer of hundreds of MB: not kept for the small uploads after.
        if total > 32 << 20 {
            if let Some(b) = self.slot(f, 2).take() {
                self.trash_buffers.push(b);
            }
        }
        Ok(())
    }

    /// Fills frame `f`'s draw-state buffer (texture ids and sampler keys
    /// replaced by slots, the EFB scale filled in) and its descriptor set.
    fn write_descriptors(&mut self, f: usize, out: &Output) -> Res<()> {
        self.frames[f].described = None;
        self.write_draw_states(f, out, None)
    }

    /// [`Renderer::write_descriptors`], plus bindings 3..5 at `moves` (in
    /// `buffer`) when given.
    fn write_draw_states(&mut self, f: usize, out: &Output, moves: Option<(vk::Buffer, [(u64, u64); 3])>) -> Res<()> {
        let mut tex_slots: HashMap<u32, u32> = HashMap::new();
        let mut views = Vec::new();
        let mut sampler_slots: HashMap<u32, u32> = HashMap::new();
        let mut samplers = Vec::new();
        let mut words: Vec<u32> = Vec::with_capacity(out.shadings.len().max(1) * SHADING_WORDS);
        let (scale_x, scale_y) = (self.sx().to_bits(), self.sy().to_bits());
        for s in &out.shadings {
            let mut w = s.0;
            w[sh::SCALE] = scale_x;
            w[sh::SCALE_Y] = scale_y;
            for m in 0..8 {
                let id = w[sh::TEX + m];
                if id == NO_TEXTURE {
                    continue;
                }
                let slot = match (tex_slots.get(&id), self.textures.get(&id)) {
                    (Some(&slot), _) => slot,
                    (None, Some(img)) if (views.len() as u32) < MAX_TEXTURES => {
                        views.push(vk::DescriptorImageInfo { sampler: vk::Sampler::null(), image_view: img.view, image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL });
                        tex_slots.insert(id, views.len() as u32 - 1);
                        views.len() as u32 - 1
                    }
                    _ => NO_TEXTURE,
                };
                w[sh::TEX + m] = slot;
                let key = w[sh::SAMPLER + m];
                let sslot = match sampler_slots.get(&key) {
                    Some(&s) => s,
                    None if (samplers.len() as u32) < MAX_SAMPLERS => {
                        let smp = self.sampler(key)?;
                        samplers.push(vk::DescriptorImageInfo { sampler: smp, image_view: vk::ImageView::null(), image_layout: vk::ImageLayout::UNDEFINED });
                        sampler_slots.insert(key, samplers.len() as u32 - 1);
                        samplers.len() as u32 - 1
                    }
                    None => 0,
                };
                w[sh::SAMPLER + m] = sslot;
            }
            words.extend_from_slice(&w);
        }
        if words.is_empty() {
            words.resize(SHADING_WORDS, 0);
        }
        self.stats.textures = views.len();
        let bytes = words.len() as u64 * 4;
        let sb = self.frame_buffer(f, 1, bytes, vk::BufferUsageFlags::STORAGE_BUFFER)?;
        self.frame_bytes(f, 1)[..bytes as usize].copy_from_slice(as_bytes(&words));
        let set = self.frames[f].set;
        let buf_info = [vk::DescriptorBufferInfo { buffer: sb, offset: 0, range: vk::WHOLE_SIZE }];
        let mut writes = vec![vk::WriteDescriptorSet::default().dst_set(set).dst_binding(0).descriptor_type(vk::DescriptorType::STORAGE_BUFFER).buffer_info(&buf_info)];
        if !views.is_empty() {
            writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(1).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&views));
        }
        if !samplers.is_empty() {
            writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(2).descriptor_type(vk::DescriptorType::SAMPLER).image_info(&samplers));
        }
        let move_infos = moves.map(|(buffer, ranges)| ranges.map(|(offset, range)| [vk::DescriptorBufferInfo { buffer, offset, range }]));
        if let Some(infos) = &move_infos {
            for (i, info) in infos.iter().enumerate() {
                writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(3 + i as u32).descriptor_type(vk::DescriptorType::STORAGE_BUFFER).buffer_info(info));
            }
        }
        unsafe { self.device.update_descriptor_sets(&writes, &[]) };
        Ok(())
    }

    /// An EFB rectangle in EFB-image pixels: each edge rounded (so
    /// neighbouring rectangles still meet), at least one pixel, inside the
    /// image.
    fn scaled(&self, r: Rect) -> vk::Rect2D {
        let edge = |v: u32, s: f32, size: u32| ((v as f32 * s).round() as u32).min(size);
        let (iw, ih) = (self.efb_color.w, self.efb_color.h);
        let (x0, y0) = (edge(r.x, self.sx(), iw - 1), edge(r.y, self.sy(), ih - 1));
        let (x1, y1) = (edge(r.x + r.w, self.sx(), iw).max(x0 + 1), edge(r.y + r.h, self.sy(), ih).max(y0 + 1));
        rect2d(x0, y0, x1 - x0, y1 - y0)
    }

    fn scaled_f(&self, r: Rect) -> [f32; 4] {
        let (sx, sy) = (self.sx(), self.sy());
        [r.x as f32 * sx, r.y as f32 * sy, r.w as f32 * sx, r.h as f32 * sy]
    }

    fn begin_efb(&mut self, cmd: vk::CommandBuffer, vb: vk::Buffer, set: vk::DescriptorSet) {
        let dev = &self.device;
        unsafe {
            transition(dev, cmd, &mut self.efb_color, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
            transition(dev, cmd, &mut self.efb_depth, vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL);
            let color = [vk::RenderingAttachmentInfo::default()
                .image_view(self.efb_color.view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::LOAD)
                .store_op(vk::AttachmentStoreOp::STORE)];
            let depth = vk::RenderingAttachmentInfo::default()
                .image_view(self.efb_depth.view)
                .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::LOAD)
                .store_op(vk::AttachmentStoreOp::STORE);
            let area = rect2d(0, 0, self.efb_color.w, self.efb_color.h);
            dev.cmd_begin_rendering(cmd, &vk::RenderingInfo::default().render_area(area).layer_count(1).color_attachments(&color).depth_attachment(&depth));
            dev.cmd_set_viewport(cmd, 0, &[vk::Viewport { x: 0.0, y: 0.0, width: self.efb_color.w as f32, height: self.efb_color.h as f32, min_depth: 0.0, max_depth: 1.0 }]);
            dev.cmd_set_front_face(cmd, vk::FrontFace::CLOCKWISE);
            dev.cmd_bind_vertex_buffers(cmd, 0, &[vb], &[0]);
            dev.cmd_bind_index_buffer(cmd, vb, self.index_offset, vk::IndexType::UINT32);
            dev.cmd_bind_descriptor_sets(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, &[set], &[]);
            dev.cmd_push_constants(cmd, self.pipeline_layout, vk::ShaderStageFlags::VERTEX, 0, as_bytes(&self.push));
        }
    }

    /// Runs the copy pass from the EFB into `target` (its top-left
    /// `params.dst` texels).
    fn copy_pass(dev: &Device, cmd: vk::CommandBuffer, pass: CopyPass, efb: (&mut Image, &mut Image), target: &mut Image, params: CopyParams) {
        unsafe {
            transition(dev, cmd, efb.0, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            transition(dev, cmd, efb.1, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            transition(dev, cmd, target, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
            let (w, h) = (params.dst[0] as u32, params.dst[1] as u32);
            let color = [vk::RenderingAttachmentInfo::default()
                .image_view(target.view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::DONT_CARE)
                .store_op(vk::AttachmentStoreOp::STORE)];
            dev.cmd_begin_rendering(cmd, &vk::RenderingInfo::default().render_area(rect2d(0, 0, w, h)).layer_count(1).color_attachments(&color));
            dev.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, pass.pipeline);
            dev.cmd_set_viewport(cmd, 0, &[vk::Viewport { x: 0.0, y: 0.0, width: w as f32, height: h as f32, min_depth: 0.0, max_depth: 1.0 }]);
            dev.cmd_set_scissor(cmd, 0, &[rect2d(0, 0, w, h)]);
            dev.cmd_bind_descriptor_sets(cmd, vk::PipelineBindPoint::GRAPHICS, pass.layout, 0, &[pass.set], &[]);
            let bytes = std::slice::from_raw_parts((&params as *const CopyParams).cast::<u8>(), std::mem::size_of::<CopyParams>());
            dev.cmd_push_constants(cmd, pass.layout, vk::ShaderStageFlags::FRAGMENT, 0, bytes);
            dev.cmd_draw(cmd, 3, 1, 0, 0);
            dev.cmd_end_rendering(cmd);
        }
    }

    /// The copy pass at native size into the scratch image, then into
    /// `buffer` at `offset`.
    fn copy_to_native(&mut self, cmd: vk::CommandBuffer, params: CopyParams, buffer: vk::Buffer, offset: u64) {
        let dev = self.device.clone();
        Self::copy_pass(&dev, cmd, self.copy, (&mut self.efb_color, &mut self.efb_depth), &mut self.native, params);
        unsafe {
            transition(&dev, cmd, &mut self.native, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
            let region = vk::BufferImageCopy::default()
                .buffer_offset(offset)
                .image_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                .image_extent(vk::Extent3D { width: params.dst[0] as u32, height: params.dst[1] as u32, depth: 1 });
            dev.cmd_copy_image_to_buffer(cmd, self.native.image, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, buffer, &[region]);
        }
    }

    fn record_efb(&mut self, f: usize, cmd: vk::CommandBuffer, vb: vk::Buffer, out: &Output, fresh: bool) -> Res<()> {
        let dev = self.device.clone();
        let set = self.frames[f].set;
        if !self.efb_ready {
            unsafe {
                transition(&dev, cmd, &mut self.efb_color, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
                transition(&dev, cmd, &mut self.efb_depth, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
                dev.cmd_clear_color_image(cmd, self.efb_color.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] }, &[range(vk::ImageAspectFlags::COLOR, 1)]);
                dev.cmd_clear_depth_stencil_image(cmd, self.efb_depth.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 }, &[range(vk::ImageAspectFlags::DEPTH, 1)]);
            }
            self.efb_ready = true;
        }
        // Readback space for this frame's EFB copies.
        let readback_bytes: u64 = if fresh && self.copies_to_ram { out.cmds.iter().filter_map(|c| if let Cmd::CopyTex(c) = c { Some(u64::from(c.w * c.h * 4)) } else { None }).sum() } else { 0 };
        let readback = if readback_bytes > 0 { Some(self.frame_buffer(f, 3, readback_bytes, vk::BufferUsageFlags::TRANSFER_DST)?) } else { None };
        let mut readback_at = 0usize;
        let mut rendering = false;
        let mut bound = None;
        // Dynamic state as last set inside this rendering pass (scissor, cull
        // mode, depth mode): neighbouring draws mostly share it, and the
        // world map has over 10,000 draws a frame.
        let mut state: Option<(vk::Rect2D, u32, u32)> = None;
        let mut draws = 0;
        for c in &out.cmds {
            match *c {
                Cmd::Draw(d) => {
                    if d.scissor.w == 0 || d.scissor.h == 0 || d.raster.cull == 3 {
                        continue;
                    }
                    if !rendering {
                        self.begin_efb(cmd, vb, set);
                        rendering = true;
                        bound = None;
                        state = None;
                    }
                    let early = out.shadings.get(d.shading as usize).is_some_and(|s| rasterised_depth(&s.0));
                    let p = self.pipeline(blend_key(&d.raster) | if early { EARLY_DEPTH } else { 0 })?;
                    let z = d.raster.zmode;
                    unsafe {
                        if bound != Some(p) {
                            dev.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, p);
                            bound = Some(p);
                        }
                        let scissor = self.scaled(d.scissor);
                        let last = state.unwrap_or((rect2d(0, 0, 0, 0), u32::MAX, u32::MAX));
                        if last.0 != scissor {
                            dev.cmd_set_scissor(cmd, 0, &[scissor]);
                        }
                        if last.1 != d.raster.cull {
                            dev.cmd_set_cull_mode(cmd, match d.raster.cull {
                                1 => vk::CullModeFlags::BACK,
                                2 => vk::CullModeFlags::FRONT,
                                _ => vk::CullModeFlags::NONE,
                            });
                        }
                        if last.2 != z {
                            dev.cmd_set_depth_test_enable(cmd, z & 1 != 0);
                            dev.cmd_set_depth_write_enable(cmd, z & 1 != 0 && z & 0x10 != 0);
                            dev.cmd_set_depth_compare_op(cmd, vk::CompareOp::from_raw(((z >> 1) & 7) as i32));
                        }
                        state = Some((scissor, d.raster.cull, z));
                        dev.cmd_draw_indexed(cmd, d.count, 1, d.first, 0, d.shading);
                    }
                    draws += 1;
                }
                Cmd::Clear(c) => {
                    if !rendering {
                        self.begin_efb(cmd, vb, set);
                        rendering = true;
                        bound = None;
                        // The pass restarts after a copy, whose pipeline left
                        // scissor, cull and depth state of its own.
                        state = None;
                    }
                    let rect = [vk::ClearRect { rect: self.scaled(c.rect), base_array_layer: 0, layer_count: 1 }];
                    let mut atts = Vec::new();
                    if c.color_update || c.alpha_update {
                        let col = c.color.map(|v| f32::from(v) / 255.0);
                        atts.push(vk::ClearAttachment { aspect_mask: vk::ImageAspectFlags::COLOR, color_attachment: 0, clear_value: vk::ClearValue { color: vk::ClearColorValue { float32: col } } });
                    }
                    if c.z_update {
                        let depth = c.z as f32 / 16_777_216.0;
                        atts.push(vk::ClearAttachment { aspect_mask: vk::ImageAspectFlags::DEPTH, color_attachment: 0, clear_value: vk::ClearValue { depth_stencil: vk::ClearDepthStencilValue { depth, stencil: 0 } } });
                    }
                    if !atts.is_empty() {
                        unsafe { dev.cmd_clear_attachments(cmd, &atts, &rect) };
                    }
                }
                Cmd::CopyXfb { src, addr, height } => {
                    if rendering {
                        unsafe { dev.cmd_end_rendering(cmd) };
                        rendering = false;
                    }
                    let (w, h) = (at_scale(src.w, self.sx()), at_scale(height, self.sy()));
                    let stale = self.xfbs.get(&addr).is_some_and(|(img, _)| img.w != w || img.h != h);
                    if stale {
                        let (img, _) = self.xfbs.remove(&addr).unwrap();
                        self.trash_images.push(img);
                    }
                    if !self.xfbs.contains_key(&addr) {
                        if self.xfbs.len() >= MAX_XFBS {
                            let oldest = *self.xfbs.iter().min_by_key(|(_, (_, s))| *s).unwrap().0;
                            let (img, _) = self.xfbs.remove(&oldest).unwrap();
                            self.trash_images.push(img);
                        }
                        let usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
                        let img = Self::create_image(&dev, &mut self.alloc, w, h, 1, COLOR_FORMAT, usage, vk::ImageAspectFlags::COLOR)?;
                        self.xfbs.insert(addr, (img, 0));
                    }
                    self.xfb_serial += 1;
                    let serial = self.xfb_serial;
                    let src_rect = self.scaled(src);
                    let (dst, s) = self.xfbs.get_mut(&addr).unwrap();
                    *s = serial;
                    Self::blit_from_efb(&dev, cmd, &mut self.efb_color, dst, src_rect);
                    unsafe { transition(&dev, cmd, dst, vk::ImageLayout::TRANSFER_SRC_OPTIMAL) };
                }
                Cmd::CopyTex(c) => {
                    if rendering {
                        unsafe { dev.cmd_end_rendering(cmd) };
                        rendering = false;
                    }
                    let flags = u32::from(c.intensity) | u32::from(c.depth) << 1 | u32::from(c.efb_alpha) << 2;
                    let src = self.scaled_f(c.src);
                    if let Some(dst) = self.textures.get_mut(&c.id) {
                        let params = CopyParams { src, dst: [dst.w as f32, dst.h as f32], format: c.format, flags };
                        Self::copy_pass(&dev, cmd, self.copy, (&mut self.efb_color, &mut self.efb_depth), dst, params);
                        unsafe { transition(&dev, cmd, dst, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) };
                    }
                    if let Some(buffer) = readback {
                        let params = CopyParams { src, dst: [c.w as f32, c.h as f32], format: c.format, flags };
                        self.copy_to_native(cmd, params, buffer, readback_at as u64);
                        self.frames[f].writebacks.push(PendingWriteback { addr: c.addr, ram_hash: c.ram_hash, tex_format: c.tex_format, w: c.w, h: c.h, stride: c.stride, offset: readback_at });
                        readback_at += (c.w * c.h * 4) as usize;
                    }
                }
                Cmd::Patch(p) => {
                    if rendering {
                        unsafe { dev.cmd_end_rendering(cmd) };
                        rendering = false;
                    }
                    if p.copy != p.target {
                        if let [Some(src), Some(dst)] = self.textures.get_disjoint_mut([&p.copy, &p.target]) {
                            Self::patch(&dev, cmd, src, dst, p);
                        }
                    }
                }
            }
        }
        if rendering {
            unsafe { dev.cmd_end_rendering(cmd) };
        }
        self.stats.draws = draws;
        self.stats.pipelines = self.pipelines.len();
        Ok(())
    }

    /// Draws EFB copy `src` (at the EFB's scale) into `dst` (a texture at
    /// its own size) at the patch's place.
    fn patch(dev: &Device, cmd: vk::CommandBuffer, src: &mut Image, dst: &mut Image, p: gx::Patch) {
        if p.x + p.w > dst.w || p.y + p.h > dst.h {
            return;
        }
        unsafe {
            transition(dev, cmd, src, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
            transition(dev, cmd, dst, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
            let region = vk::ImageBlit::default()
                .src_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                .src_offsets(offsets(0, 0, src.w, src.h))
                .dst_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                .dst_offsets(offsets(p.x, p.y, p.w, p.h));
            dev.cmd_blit_image(cmd, src.image, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, dst.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[region], vk::Filter::LINEAR);
            transition(dev, cmd, src, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            transition(dev, cmd, dst, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        }
    }

    /// Scales the EFB rectangle `src` onto all of `dst`.
    fn blit_from_efb(dev: &Device, cmd: vk::CommandBuffer, efb: &mut Image, dst: &mut Image, src: vk::Rect2D) {
        unsafe {
            transition(dev, cmd, efb, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
            transition(dev, cmd, dst, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
            let region = vk::ImageBlit::default()
                .src_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                .src_offsets(offsets(src.offset.x as u32, src.offset.y as u32, src.extent.width, src.extent.height))
                .dst_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                .dst_offsets(offsets(0, 0, dst.w, dst.h));
            dev.cmd_blit_image(cmd, efb.image, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, dst.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[region], vk::Filter::LINEAR);
        }
    }

    fn present(&mut self, f: usize, cmd: vk::CommandBuffer, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let dev = self.device.clone();
        let xfb = self.xfbs.get(&xfb_addr).map(|x| (xfb_addr, x.1)).or_else(|| self.xfbs.iter().max_by_key(|(_, (_, s))| *s).map(|(a, (_, s))| (*a, *s))).map(|x| x.0);
        let mut readback = None;
        if let Some(addr) = xfb.filter(|_| self.screenshot.is_some()) {
            let path = self.screenshot.take().unwrap();
            let (w, h) = (self.xfbs[&addr].0.w, self.xfbs[&addr].0.h);
            let buf = self.create_buffer(u64::from(w * h * 4), vk::BufferUsageFlags::TRANSFER_DST, MemoryLocation::GpuToCpu)?;
            let region = vk::BufferImageCopy::default().image_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0)).image_extent(vk::Extent3D { width: w, height: h, depth: 1 });
            unsafe { dev.cmd_copy_image_to_buffer(cmd, self.xfbs[&addr].0.image, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, buf.buffer, &[region]) };
            readback = Some((path, buf, w, h));
        }

        let mut index = None;
        if width > 0 && height > 0 && self.has_window() {
            if self.needs_rebuild {
                self.rebuild(width, height)?;
            }
            let swapchain_fn = self.swapchain_fn.as_ref().unwrap();
            match unsafe { swapchain_fn.acquire_next_image(self.swapchain, u64::MAX, self.frames[f].acquired, vk::Fence::null()) } {
                Ok((i, _)) => index = Some(i),
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => self.needs_rebuild = true,
                Err(e) => return Err(e.into()),
            }
        }
        if let Some(i) = index {
            let mut target = Image {
                image: self.swap_images[i as usize],
                view: vk::ImageView::null(),
                alloc: None,
                w: self.swap_extent.width,
                h: self.swap_extent.height,
                levels: 1,
                aspect: vk::ImageAspectFlags::COLOR,
                layout: vk::ImageLayout::UNDEFINED,
            };
            unsafe {
                transition(&dev, cmd, &mut target, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
                dev.cmd_clear_color_image(cmd, target.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] }, &[range(vk::ImageAspectFlags::COLOR, 1)]);
                if let Some(addr) = xfb {
                    let src = &self.xfbs[&addr].0;
                    // Letterbox to the display aspect ratio.
                    let (tw, th) = (target.w as f32, target.h as f32);
                    let (w, h) = if tw / th > self.aspect { (th * self.aspect, th) } else { (tw, tw / self.aspect) };
                    let (x, y) = (((tw - w) / 2.0) as u32, ((th - h) / 2.0) as u32);
                    let region = vk::ImageBlit::default()
                        .src_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                        .src_offsets(offsets(0, 0, src.w, src.h))
                        .dst_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                        .dst_offsets(offsets(x, y, (w as u32).max(1), (h as u32).max(1)));
                    dev.cmd_blit_image(cmd, src.image, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, target.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[region], vk::Filter::LINEAR);
                }
                // The overlay: one nearest-filtered blit of a palette texel per rectangle.
                let (tw, th) = (target.w as i32, target.h as i32);
                let regions: Vec<vk::ImageBlit> = self
                    .overlay
                    .iter()
                    .filter_map(|r| {
                        let (x0, y0) = (r.x.clamp(0, tw), r.y.clamp(0, th));
                        let (x1, y1) = ((r.x + r.w).clamp(0, tw), (r.y + r.h).clamp(0, th));
                        (x1 > x0 && y1 > y0).then(|| {
                            vk::ImageBlit::default()
                                .src_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                                .src_offsets(offsets(u32::from(r.color % PALETTE.len() as u8), 0, 1, 1))
                                .dst_subresource(sub_layers(vk::ImageAspectFlags::COLOR, 0))
                                .dst_offsets([vk::Offset3D { x: x0, y: y0, z: 0 }, vk::Offset3D { x: x1, y: y1, z: 1 }])
                        })
                    })
                    .collect();
                if !regions.is_empty() {
                    dev.cmd_blit_image(cmd, self.palette.image, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, target.image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &regions, vk::Filter::NEAREST);
                }
                if let (Some(frame), Some(ui), Some(&view)) = (&self.ui_frame, &mut self.ui, self.swap_views.get(i as usize)) {
                    if !frame.primitives.is_empty() && ui.format == self.swap_format {
                        transition(&dev, cmd, &mut target, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
                        let color = [vk::RenderingAttachmentInfo::default()
                            .image_view(view)
                            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                            .load_op(vk::AttachmentLoadOp::LOAD)
                            .store_op(vk::AttachmentStoreOp::STORE)];
                        let area = rect2d(0, 0, target.w, target.h);
                        dev.cmd_begin_rendering(cmd, &vk::RenderingInfo::default().render_area(area).layer_count(1).color_attachments(&color));
                        let drawn = ui.renderer.cmd_draw(cmd, self.swap_extent, frame.pixels_per_point, &frame.primitives);
                        dev.cmd_end_rendering(cmd);
                        drawn.map_err(|e| format!("menus: {e}"))?;
                    }
                }
                transition(&dev, cmd, &mut target, vk::ImageLayout::PRESENT_SRC_KHR);
            }
        }
        unsafe {
            dev.end_command_buffer(cmd)?;
            dev.reset_fences(&[self.frames[f].fence])?;
            let cmds = [cmd];
            let wait = [self.frames[f].acquired];
            let stages = [vk::PipelineStageFlags::TRANSFER];
            let mut submit = vk::SubmitInfo::default().command_buffers(&cmds);
            let signal;
            if let Some(i) = index {
                signal = [self.rendered[i as usize]];
                submit = submit.wait_semaphores(&wait).wait_dst_stage_mask(&stages).signal_semaphores(&signal);
            }
            dev.queue_submit(self.queue, &[submit], self.frames[f].fence)?;
            // Retired so far: possibly used by this or an earlier submission.
            self.frames[f].retired_images.append(&mut self.trash_images);
            self.frames[f].retired_buffers.append(&mut self.trash_buffers);
            self.frames[f].retired_spare.append(&mut self.trash_spare);
            if let (Some(i), Some(swapchain_fn)) = (index, &self.swapchain_fn) {
                let swapchains = [self.swapchain];
                let indices = [i];
                let signal = [self.rendered[i as usize]];
                match swapchain_fn.queue_present(self.queue, &vk::PresentInfoKHR::default().wait_semaphores(&signal).swapchains(&swapchains).image_indices(&indices)) {
                    Ok(false) => {}
                    // An upright swapchain on a turned display is reported
                    // suboptimal at every present (Android): that is the
                    // choice made in `rebuild`, not a reason to rebuild it,
                    // which this did once a frame.
                    Ok(true) if self.upright => {}
                    Ok(true) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => self.needs_rebuild = true,
                    Err(e) => return Err(e.into()),
                }
            }
        }
        if let Some((path, mut buf, w, h)) = readback {
            unsafe { dev.wait_for_fences(&[self.frames[f].fence], true, u64::MAX)? };
            let pixels = buf.alloc.as_mut().and_then(|a| a.mapped_slice_mut()).map(|s| s[..(w * h * 4) as usize].to_vec()).unwrap_or_default();
            self.destroy_buffer(buf);
            save_png(&path, w, h, &pixels)?;
        }
        Ok(index.is_some())
    }
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

fn as_bytes<T: Copy>(v: &[T]) -> &[u8] {
    // SAFETY: T is plain old data (`gx::Vertex` is repr(C) f32s; u32).
    unsafe { std::slice::from_raw_parts(v.as_ptr().cast::<u8>(), std::mem::size_of_val(v)) }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            if let Some(path) = &self.pipeline_cache_path {
                if let Ok(data) = self.device.get_pipeline_cache_data(self.pipeline_cache) {
                    let _ = std::fs::write(path, data);
                }
            }
        }
        let mut images: Vec<Image> = self.textures.drain().map(|(_, i)| i).collect();
        images.extend(self.xfbs.drain().map(|(_, (i, _))| i));
        images.append(&mut self.trash_images);
        let mut buffers = std::mem::take(&mut self.trash_buffers);
        buffers.append(&mut self.trash_spare);
        buffers.append(&mut self.spare);
        buffers.extend(self.prepared.take().map(|p| p.buffer));
        for f in &mut self.frames {
            images.append(&mut f.retired_images);
            buffers.append(&mut f.retired_buffers);
            buffers.append(&mut f.retired_spare);
            buffers.extend(f.vertices.take());
            buffers.extend(f.shadings.take());
            buffers.extend(f.staging.take());
            buffers.extend(f.readback.take());
        }
        for i in images {
            self.destroy_image(i);
        }
        for b in buffers {
            self.destroy_buffer(b);
        }
        let empty = |aspect| Image { image: vk::Image::null(), view: vk::ImageView::null(), alloc: None, w: 0, h: 0, levels: 1, aspect, layout: vk::ImageLayout::UNDEFINED };
        for img in [
            std::mem::replace(&mut self.efb_color, empty(vk::ImageAspectFlags::COLOR)),
            std::mem::replace(&mut self.efb_depth, empty(vk::ImageAspectFlags::DEPTH)),
            std::mem::replace(&mut self.native, empty(vk::ImageAspectFlags::COLOR)),
            std::mem::replace(&mut self.palette, empty(vk::ImageAspectFlags::COLOR)),
        ] {
            self.destroy_image(img);
        }
        unsafe {
            ManuallyDrop::drop(&mut self.alloc);
            for (_, s) in self.samplers.drain() {
                self.device.destroy_sampler(s, None);
            }
            for (_, p) in self.pipelines.drain() {
                self.device.destroy_pipeline(p, None);
            }
            self.device.destroy_pipeline(self.copy.pipeline, None);
            self.device.destroy_pipeline_layout(self.copy.layout, None);
            self.device.destroy_pipeline_cache(self.pipeline_cache, None);
            self.device.destroy_shader_module(self.vs, None);
            self.device.destroy_shader_module(self.fs, None);
            self.device.destroy_shader_module(self.fs_early, None);
            for m in self.copy_modules {
                self.device.destroy_shader_module(m, None);
            }
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_descriptor_pool(self.desc_pool, None);
            self.device.destroy_descriptor_set_layout(self.set_layout, None);
            self.device.destroy_descriptor_set_layout(self.copy_set_layout, None);
            for f in &self.frames {
                self.device.destroy_semaphore(f.acquired, None);
                self.device.destroy_fence(f.fence, None);
            }
            for &s in &self.rendered {
                self.device.destroy_semaphore(s, None);
            }
            for &v in &self.swap_views {
                self.device.destroy_image_view(v, None);
            }
            self.ui = None;
            self.device.destroy_command_pool(self.pool, None);
            if let Some(f) = &self.swapchain_fn {
                if self.swapchain != vk::SwapchainKHR::null() {
                    f.destroy_swapchain(self.swapchain, None);
                }
            }
            self.device.destroy_device(None);
            if let Some((f, s)) = &self.surface {
                if *s != vk::SurfaceKHR::null() {
                    f.destroy_surface(*s, None);
                }
            }
            if let Some((f, m)) = &self.debug {
                f.destroy_debug_utils_messenger(*m, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
