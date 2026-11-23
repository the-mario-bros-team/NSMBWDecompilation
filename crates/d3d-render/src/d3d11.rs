//! Direct3D 11 renderer, for GPUs without Vulkan 1.3 or Direct3D 12
//! (feature level 11.0 and up: shader model 5.0).
//!
//! As the Direct3D 12 one, with what Direct3D 11 lacks worked around:
//! - Shader model 5.0 cannot index an array of textures with a computed
//!   value: each draw's eight textures and samplers are bound to slots of
//!   their own (`shaders::ubershader_wgsl`), rebound when they change.
//! - Clears of part of a view are a draw (`common::CLEAR_HLSL`).
//! - Logic ops need Direct3D 11.1 and, as in Direct3D 12, an integer
//!   format: logic-op draws write integers through a UINT view of the EFB
//!   where the device has logic ops on RGBA8 UINT
//!   (`D3D11_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP`); otherwise
//!   `render::logic_as_blend` (exact for clear and no-op, copy for the rest).
//! - One immediate context: renderers sharing a device (players' windows)
//!   take turns, each holding it for a whole frame.
//!
//! Direct3D 11 keeps resources alive while the GPU uses them, so nothing
//! waits to be freed.

use std::collections::HashMap;
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use gx::{sh, Cmd, Output, Rect, Vertex, NO_TEXTURE, SHADING_WORDS};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use render::{as_bytes, at_scale, blend_key, rasterised_depth, Api, Backend, Blend, BlendFactor, CopyParams, EfbSnapshot, FrameStats, Options, OverlayRect, Res, UiFrame, Writeback, EARLY_DEPTH, EFB_H, EFB_W, MAX_XFBS};
use windows::core::{Interface, PCSTR};
use windows::Win32::Foundation::{HMODULE, HWND, RECT};
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

use crate::common::{self, BlitParams, Shaders, UI_VERTEX_SIZE};

const FRAMES: usize = 2;
const COLOR: DXGI_FORMAT = DXGI_FORMAT_R8G8B8A8_UNORM;
const SWAP_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM;
const VERTEX_SIZE: u32 = std::mem::size_of::<Vertex>() as u32;
const NO_VIEWS: [Option<ID3D11ShaderResourceView>; 12] = [const { None }; 12];

struct Image {
    tex: ID3D11Texture2D,
    srv: Option<ID3D11ShaderResourceView>,
    rtv: Option<ID3D11RenderTargetView>,
    /// The EFB's UINT view, for logic-op draws.
    rtv_uint: Option<ID3D11RenderTargetView>,
    w: u32,
    h: u32,
}

/// A buffer the CPU writes (`Map` with discard) and the GPU reads.
struct Dynamic {
    buf: ID3D11Buffer,
    size: u32,
    /// A raw view (`ByteAddressBuffer`) of it, for buffers the shaders read.
    srv: Option<ID3D11ShaderResourceView>,
}

/// A readback the frame owes: a staging copy of an EFB copy.
struct PendingWriteback {
    addr: u32,
    ram_hash: u64,
    tex_format: u32,
    w: u32,
    h: u32,
    stride: u32,
    staging: ID3D11Texture2D,
}

struct Prepared {
    key: (u64, u64),
    serial: u64,
    /// Vertices then indices.
    vb: Dynamic,
    index_offset: u32,
    /// Frame interpolation's model positions, matrix pairs and draw pairs.
    moves: Option<[ID3D11ShaderResourceView; 3]>,
    _moves_buf: Option<Dynamic>,
}

/// What renderers on one device share.
pub struct Gpu {
    factory: IDXGIFactory2,
    device: ID3D11Device,
    /// The immediate context, one renderer at a time.
    context: Mutex<ID3D11DeviceContext>,
    info: Option<ID3D11InfoQueue>,
    device_name: String,
    validation: bool,
    logic_ops: bool,
    uber_vs: ID3D11VertexShader,
    uber_fs: ID3D11PixelShader,
    uber_fs_early: ID3D11PixelShader,
    uber_fs_logic: ID3D11PixelShader,
    uber_layout: ID3D11InputLayout,
    copy_vs: ID3D11VertexShader,
    copy_fs: ID3D11PixelShader,
    blit_vs: ID3D11VertexShader,
    blit_ps: ID3D11PixelShader,
    ui_vs: ID3D11VertexShader,
    ui_ps: ID3D11PixelShader,
    ui_layout: ID3D11InputLayout,
    clear_vs: ID3D11VertexShader,
    clear_ps: ID3D11PixelShader,
    bilinear: ID3D11SamplerState,
    no_cull: ID3D11RasterizerState,
    opaque: ID3D11BlendState,
    ui_blend: ID3D11BlendState,
    no_depth: ID3D11DepthStencilState,
}

// The device is free-threaded; the context is behind its lock.
unsafe impl Send for Gpu {}
unsafe impl Sync for Gpu {}

fn validation_wanted() -> bool {
    cfg!(debug_assertions) || std::env::var_os("NSMBW_VALIDATION").is_some()
}

fn drain_messages(gpu: &Gpu) {
    let Some(info) = &gpu.info else { return };
    unsafe {
        let n = info.GetNumStoredMessages();
        for i in 0..n {
            let mut len = 0usize;
            if info.GetMessage(i, None, &mut len).is_err() || len == 0 {
                continue;
            }
            let mut buf = vec![0u64; len.div_ceil(8)];
            let msg = buf.as_mut_ptr().cast::<D3D11_MESSAGE>();
            if info.GetMessage(i, Some(msg), &mut len).is_err() {
                continue;
            }
            let m = &*msg;
            let text = String::from_utf8_lossy(std::slice::from_raw_parts(m.pDescription, m.DescriptionByteLength.saturating_sub(1)));
            let error = m.Severity == D3D11_MESSAGE_SEVERITY_ERROR || m.Severity == D3D11_MESSAGE_SEVERITY_CORRUPTION;
            if error || m.Severity == D3D11_MESSAGE_SEVERITY_WARNING {
                render::note_validation("d3d11", error, i64::from(m.ID.0), &text);
            }
        }
        info.ClearStoredMessages();
    }
}

fn d3d_blend(f: BlendFactor) -> D3D11_BLEND {
    match f {
        BlendFactor::Zero => D3D11_BLEND_ZERO,
        BlendFactor::One => D3D11_BLEND_ONE,
        BlendFactor::SrcColor => D3D11_BLEND_SRC_COLOR,
        BlendFactor::OneMinusSrcColor => D3D11_BLEND_INV_SRC_COLOR,
        BlendFactor::DstColor => D3D11_BLEND_DEST_COLOR,
        BlendFactor::OneMinusDstColor => D3D11_BLEND_INV_DEST_COLOR,
        BlendFactor::Src1Alpha => D3D11_BLEND_SRC1_ALPHA,
        BlendFactor::OneMinusSrc1Alpha => D3D11_BLEND_INV_SRC1_ALPHA,
        BlendFactor::DstAlpha => D3D11_BLEND_DEST_ALPHA,
        BlendFactor::OneMinusDstAlpha => D3D11_BLEND_INV_DEST_ALPHA,
    }
}

fn alpha_factor(f: D3D11_BLEND) -> D3D11_BLEND {
    match f {
        D3D11_BLEND_SRC_COLOR => D3D11_BLEND_SRC_ALPHA,
        D3D11_BLEND_INV_SRC_COLOR => D3D11_BLEND_INV_SRC_ALPHA,
        D3D11_BLEND_DEST_COLOR => D3D11_BLEND_DEST_ALPHA,
        D3D11_BLEND_INV_DEST_COLOR => D3D11_BLEND_INV_DEST_ALPHA,
        f => f,
    }
}

/// The blend state for blend key `key` (Direct3D 11.1's, for logic ops).
fn blend_desc(key: u32, logic_ops: bool) -> D3D11_BLEND_DESC1 {
    let (blend, mask) = render::blend_of(key);
    let mut write = 0u8;
    if mask & 1 != 0 {
        write |= (D3D11_COLOR_WRITE_ENABLE_RED.0 | D3D11_COLOR_WRITE_ENABLE_GREEN.0 | D3D11_COLOR_WRITE_ENABLE_BLUE.0) as u8;
    }
    if mask & 2 != 0 {
        write |= D3D11_COLOR_WRITE_ENABLE_ALPHA.0 as u8;
    }
    let mut d = D3D11_RENDER_TARGET_BLEND_DESC1 {
        BlendEnable: false.into(),
        LogicOpEnable: false.into(),
        SrcBlend: D3D11_BLEND_ONE,
        DestBlend: D3D11_BLEND_ZERO,
        BlendOp: D3D11_BLEND_OP_ADD,
        SrcBlendAlpha: D3D11_BLEND_ONE,
        DestBlendAlpha: D3D11_BLEND_ZERO,
        BlendOpAlpha: D3D11_BLEND_OP_ADD,
        LogicOp: D3D11_LOGIC_OP_NOOP,
        RenderTargetWriteMask: write,
    };
    let mut add = |src: D3D11_BLEND, dst: D3D11_BLEND, op: D3D11_BLEND_OP| {
        d.BlendEnable = true.into();
        (d.SrcBlend, d.DestBlend, d.BlendOp) = (src, dst, op);
        (d.SrcBlendAlpha, d.DestBlendAlpha, d.BlendOpAlpha) = (alpha_factor(src), alpha_factor(dst), op);
    };
    match blend {
        Blend::Off => {}
        Blend::Subtract => add(D3D11_BLEND_ONE, D3D11_BLEND_ONE, D3D11_BLEND_OP_REV_SUBTRACT),
        Blend::Add { src, dst } => add(d3d_blend(src), d3d_blend(dst), D3D11_BLEND_OP_ADD),
        Blend::Logic(op) if logic_ops => {
            const D3D: [D3D11_LOGIC_OP; 16] = [
                D3D11_LOGIC_OP_CLEAR,
                D3D11_LOGIC_OP_AND,
                D3D11_LOGIC_OP_AND_REVERSE,
                D3D11_LOGIC_OP_COPY,
                D3D11_LOGIC_OP_AND_INVERTED,
                D3D11_LOGIC_OP_NOOP,
                D3D11_LOGIC_OP_XOR,
                D3D11_LOGIC_OP_OR,
                D3D11_LOGIC_OP_NOR,
                D3D11_LOGIC_OP_EQUIV,
                D3D11_LOGIC_OP_INVERT,
                D3D11_LOGIC_OP_OR_REVERSE,
                D3D11_LOGIC_OP_COPY_INVERTED,
                D3D11_LOGIC_OP_OR_INVERTED,
                D3D11_LOGIC_OP_NAND,
                D3D11_LOGIC_OP_SET,
            ];
            d.LogicOpEnable = true.into();
            d.LogicOp = D3D[op as usize & 15];
        }
        Blend::Logic(op) => {
            if let Some((src, dst)) = render::logic_as_blend(op) {
                add(d3d_blend(src), d3d_blend(dst), D3D11_BLEND_OP_ADD);
            }
        }
    }
    let mut desc = D3D11_BLEND_DESC1::default();
    desc.RenderTarget[0] = d;
    desc
}

fn viewport(x: f32, y: f32, w: f32, h: f32) -> D3D11_VIEWPORT {
    D3D11_VIEWPORT { TopLeftX: x, TopLeftY: y, Width: w, Height: h, MinDepth: 0.0, MaxDepth: 1.0 }
}

fn rect(x: u32, y: u32, w: u32, h: u32) -> RECT {
    RECT { left: x as i32, top: y as i32, right: (x + w) as i32, bottom: (y + h) as i32 }
}

impl Gpu {
    fn create(pipeline_cache: Option<&std::path::Path>) -> Res<Arc<Gpu>> {
        unsafe {
            let factory: IDXGIFactory2 = CreateDXGIFactory1()?;
            let levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0];
            let wanted = validation_wanted();
            // The first hardware adapter that makes a feature level 11
            // device, preferring the high-performance GPU.
            let mut chosen = None;
            let by_pref: Option<IDXGIFactory6> = factory.cast().ok();
            for i in 0.. {
                let adapter: IDXGIAdapter1 = match &by_pref {
                    Some(f) => match f.EnumAdapterByGpuPreference(i, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE) {
                        Ok(a) => a,
                        Err(_) => break,
                    },
                    None => match factory.EnumAdapters1(i) {
                        Ok(a) => a,
                        Err(_) => break,
                    },
                };
                let desc = adapter.GetDesc1()?;
                if desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0 {
                    continue;
                }
                let name = String::from_utf16_lossy(&desc.Description).trim_end_matches('\0').to_string();
                let make = |flags: D3D11_CREATE_DEVICE_FLAG| {
                    let (mut device, mut context) = (None, None);
                    D3D11CreateDevice(&adapter, D3D_DRIVER_TYPE_UNKNOWN, HMODULE::default(), flags, Some(&levels), D3D11_SDK_VERSION, Some(&mut device), None, Some(&mut context)).map(|_| (device, context))
                };
                // The debug layer, where it is installed.
                let made = if wanted { make(D3D11_CREATE_DEVICE_DEBUG).map(|d| (d, true)).or_else(|_| make(D3D11_CREATE_DEVICE_FLAG(0)).map(|d| (d, false))) } else { make(D3D11_CREATE_DEVICE_FLAG(0)).map(|d| (d, false)) };
                if let Ok(((Some(device), Some(context)), validation)) = made {
                    chosen = Some((device, context, name, validation));
                    break;
                }
            }
            let (device, context, device_name, validation): (ID3D11Device, ID3D11DeviceContext, String, bool) = chosen.ok_or("no Direct3D 11 device with feature level 11.0")?;
            let info: Option<ID3D11InfoQueue> = if validation { device.cast().ok() } else { None };
            let mut options = D3D11_FEATURE_DATA_D3D11_OPTIONS::default();
            let has_options = device.CheckFeatureSupport(D3D11_FEATURE_D3D11_OPTIONS, (&mut options as *mut D3D11_FEATURE_DATA_D3D11_OPTIONS).cast(), std::mem::size_of_val(&options) as u32).is_ok();
            // Logic ops work on integer formats: the EFB's UINT view.
            let mut fs = D3D11_FEATURE_DATA_FORMAT_SUPPORT2 { InFormat: DXGI_FORMAT_R8G8B8A8_UINT, OutFormatSupport2: 0 };
            let format_logic = device.CheckFeatureSupport(D3D11_FEATURE_FORMAT_SUPPORT2, (&mut fs as *mut D3D11_FEATURE_DATA_FORMAT_SUPPORT2).cast(), std::mem::size_of_val(&fs) as u32).is_ok() && fs.OutFormatSupport2 & D3D11_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP.0 as u32 != 0;
            let device1: Option<ID3D11Device1> = device.cast().ok();
            let logic_ops = has_options && options.OutputMergerLogicOp.as_bool() && format_logic && device1.is_some();

            let s: Shaders = common::compile_all(Api::D3d11, common::shader_dir(pipeline_cache).as_deref())?;
            let vs = |code: &[u8]| -> Res<ID3D11VertexShader> {
                let mut v = None;
                device.CreateVertexShader(code, None, Some(&mut v))?;
                Ok(v.ok_or("no vertex shader")?)
            };
            let ps = |code: &[u8]| -> Res<ID3D11PixelShader> {
                let mut v = None;
                device.CreatePixelShader(code, None, Some(&mut v))?;
                Ok(v.ok_or("no pixel shader")?)
            };
            let loc = PCSTR(c"LOC".as_ptr().cast());
            let mut inputs = Vec::new();
            for i in 0..3 {
                inputs.push(D3D11_INPUT_ELEMENT_DESC { SemanticName: loc, SemanticIndex: i, Format: DXGI_FORMAT_R32G32B32A32_FLOAT, InputSlot: 0, AlignedByteOffset: 16 * i, InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA, InstanceDataStepRate: 0 });
            }
            for i in 0..8 {
                inputs.push(D3D11_INPUT_ELEMENT_DESC { SemanticName: loc, SemanticIndex: 3 + i, Format: DXGI_FORMAT_R32G32B32_FLOAT, InputSlot: 0, AlignedByteOffset: 48 + 12 * i, InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA, InstanceDataStepRate: 0 });
            }
            inputs.push(D3D11_INPUT_ELEMENT_DESC { SemanticName: loc, SemanticIndex: 11, Format: DXGI_FORMAT_R32_UINT, InputSlot: 1, AlignedByteOffset: 0, InputSlotClass: D3D11_INPUT_PER_INSTANCE_DATA, InstanceDataStepRate: 1 });
            let mut uber_layout = None;
            device.CreateInputLayout(&inputs, &s.uber.vs, Some(&mut uber_layout))?;
            let e = |name: &'static std::ffi::CStr, format, offset| D3D11_INPUT_ELEMENT_DESC { SemanticName: PCSTR(name.as_ptr().cast()), SemanticIndex: 0, Format: format, InputSlot: 0, AlignedByteOffset: offset, InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA, InstanceDataStepRate: 0 };
            let ui_inputs = [e(c"POSITION", DXGI_FORMAT_R32G32_FLOAT, 0), e(c"TEXCOORD", DXGI_FORMAT_R32G32_FLOAT, 8), e(c"COLOR", DXGI_FORMAT_R8G8B8A8_UNORM, 16)];
            let mut ui_layout = None;
            device.CreateInputLayout(&ui_inputs, &s.ui_vs, Some(&mut ui_layout))?;
            let sampler = D3D11_SAMPLER_DESC {
                Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
                AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
                MaxAnisotropy: 1,
                ComparisonFunc: D3D11_COMPARISON_NEVER,
                MaxLOD: 0.0,
                ..Default::default()
            };
            let mut bilinear = None;
            device.CreateSamplerState(&sampler, Some(&mut bilinear))?;
            let mut no_cull = None;
            device.CreateRasterizerState(&rasterizer(D3D11_CULL_NONE), Some(&mut no_cull))?;
            let mut opaque = None;
            let mut bd = D3D11_BLEND_DESC::default();
            bd.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;
            device.CreateBlendState(&bd, Some(&mut opaque))?;
            bd.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: true.into(),
                SrcBlend: D3D11_BLEND_ONE,
                DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
                BlendOp: D3D11_BLEND_OP_ADD,
                SrcBlendAlpha: D3D11_BLEND_INV_DEST_ALPHA,
                DestBlendAlpha: D3D11_BLEND_ONE,
                BlendOpAlpha: D3D11_BLEND_OP_ADD,
                RenderTargetWriteMask: D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
            };
            let mut ui_blend = None;
            device.CreateBlendState(&bd, Some(&mut ui_blend))?;
            let mut no_depth = None;
            device.CreateDepthStencilState(&D3D11_DEPTH_STENCIL_DESC { DepthEnable: false.into(), ..Default::default() }, Some(&mut no_depth))?;
            let gpu = Gpu {
                factory,
                device: device.clone(),
                context: Mutex::new(context),
                info,
                device_name,
                validation,
                logic_ops,
                uber_vs: vs(&s.uber.vs)?,
                uber_fs: ps(&s.uber.fs)?,
                uber_fs_early: ps(&s.uber.fs_early)?,
                uber_fs_logic: ps(&s.uber.fs_logic)?,
                uber_layout: uber_layout.ok_or("no input layout")?,
                copy_vs: vs(&s.copy_vs)?,
                copy_fs: ps(&s.copy_fs)?,
                blit_vs: vs(&s.blit_vs)?,
                blit_ps: ps(&s.blit_ps)?,
                ui_vs: vs(&s.ui_vs)?,
                ui_ps: ps(&s.ui_ps)?,
                ui_layout: ui_layout.ok_or("no input layout")?,
                clear_vs: vs(&s.clear_vs)?,
                clear_ps: ps(&s.clear_ps)?,
                bilinear: bilinear.ok_or("no sampler")?,
                no_cull: no_cull.ok_or("no rasterizer state")?,
                opaque: opaque.ok_or("no blend state")?,
                ui_blend: ui_blend.ok_or("no blend state")?,
                no_depth: no_depth.ok_or("no depth state")?,
            };
            Ok(Arc::new(gpu))
        }
    }

    fn lock(&self) -> MutexGuard<'_, ID3D11DeviceContext> {
        self.context.lock().unwrap_or_else(|e| e.into_inner())
    }
}

fn rasterizer(cull: D3D11_CULL_MODE) -> D3D11_RASTERIZER_DESC {
    D3D11_RASTERIZER_DESC { FillMode: D3D11_FILL_SOLID, CullMode: cull, FrontCounterClockwise: false.into(), DepthClipEnable: true.into(), ScissorEnable: true.into(), ..Default::default() }
}

pub struct Renderer {
    device_name: String,
    validation: bool,
    scale: f32,
    stretch: f32,
    efb_stale: bool,
    vsync: bool,
    copies_to_ram: bool,
    gpu: Arc<Gpu>,
    device: ID3D11Device,
    hwnd: Option<HWND>,
    swapchain: Option<IDXGISwapChain1>,
    swap_rtv: Option<ID3D11RenderTargetView>,
    swap_size: (u32, u32),
    needs_rebuild: bool,
    frame: usize,
    /// Per frame: EFB copies to read back once the GPU is done with it.
    pending: [Vec<PendingWriteback>; FRAMES],
    blends: HashMap<u32, ID3D11BlendState>,
    depths: HashMap<u32, ID3D11DepthStencilState>,
    rasters: HashMap<u32, ID3D11RasterizerState>,
    samplers: HashMap<u32, ID3D11SamplerState>,
    efb_color: Image,
    efb_depth: (ID3D11Texture2D, ID3D11DepthStencilView, ID3D11ShaderResourceView),
    efb_size: (u32, u32),
    native: Image,
    efb_ready: bool,
    vertices: Option<Dynamic>,
    shadings: Option<Dynamic>,
    /// The draw index per instance: 0, 1, 2...
    instance_ids: Option<(ID3D11Buffer, u32)>,
    interp: ID3D11Buffer,
    constants: ID3D11Buffer,
    index_offset: u32,
    push: [u32; 4],
    prepared: Option<Prepared>,
    /// (prepare serial, texture epoch) the draw states were written for.
    described: Option<(u64, u64)>,
    textures: HashMap<u32, Image>,
    textures_epoch: u64,
    xfbs: HashMap<u32, (Image, u64)>,
    xfb_serial: u64,
    writebacks: Vec<Writeback>,
    ui_textures: HashMap<egui::TextureId, Image>,
    ui_buffer: Option<Dynamic>,
    ui_frame: Option<UiFrame>,
    keys_path: Option<PathBuf>,
    aspect: f32,
    screenshot: Option<PathBuf>,
    overlay: Vec<OverlayRect>,
    stats: FrameStats,
}

unsafe impl Send for Renderer {}

fn create_texture(device: &ID3D11Device, w: u32, h: u32, levels: u32, format: DXGI_FORMAT, target: bool, data: Option<&[D3D11_SUBRESOURCE_DATA]>) -> Res<Image> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: w,
        Height: h,
        MipLevels: levels,
        ArraySize: 1,
        Format: format,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: (D3D11_BIND_SHADER_RESOURCE.0 | if target { D3D11_BIND_RENDER_TARGET.0 } else { 0 }) as u32,
        ..Default::default()
    };
    unsafe {
        let mut tex = None;
        device.CreateTexture2D(&desc, data.map(|d| d.as_ptr()), Some(&mut tex))?;
        let tex: ID3D11Texture2D = tex.ok_or("no texture")?;
        let mut srv = None;
        device.CreateShaderResourceView(&tex, None, Some(&mut srv))?;
        let mut rtv = None;
        if target {
            device.CreateRenderTargetView(&tex, None, Some(&mut rtv))?;
        }
        Ok(Image { tex, srv, rtv, rtv_uint: None, w, h })
    }
}

/// The EFB's colour: RGBA8 seen as UNORM (draws, copies) and as UINT
/// (logic-op draws).
fn create_efb_color(device: &ID3D11Device, w: u32, h: u32) -> Res<Image> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: w,
        Height: h,
        MipLevels: 1,
        ArraySize: 1,
        Format: DXGI_FORMAT_R8G8B8A8_TYPELESS,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: (D3D11_BIND_SHADER_RESOURCE.0 | D3D11_BIND_RENDER_TARGET.0) as u32,
        ..Default::default()
    };
    unsafe {
        let mut tex = None;
        device.CreateTexture2D(&desc, None, Some(&mut tex))?;
        let tex: ID3D11Texture2D = tex.ok_or("no EFB texture")?;
        let sd = D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: COLOR,
            ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
            Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 { Texture2D: D3D11_TEX2D_SRV { MostDetailedMip: 0, MipLevels: 1 } },
        };
        let mut srv = None;
        device.CreateShaderResourceView(&tex, Some(&sd), Some(&mut srv))?;
        let mut views = [None, None];
        for (v, format) in views.iter_mut().zip([COLOR, DXGI_FORMAT_R8G8B8A8_UINT]) {
            let d = D3D11_RENDER_TARGET_VIEW_DESC { Format: format, ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2D, ..Default::default() };
            device.CreateRenderTargetView(&tex, Some(&d), Some(v))?;
        }
        let [rtv, rtv_uint] = views;
        Ok(Image { tex, srv, rtv, rtv_uint, w, h })
    }
}

fn staging(device: &ID3D11Device, w: u32, h: u32) -> Res<ID3D11Texture2D> {
    let desc = D3D11_TEXTURE2D_DESC {
        Width: w,
        Height: h,
        MipLevels: 1,
        ArraySize: 1,
        Format: COLOR,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: D3D11_USAGE_STAGING,
        CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
        ..Default::default()
    };
    unsafe {
        let mut tex = None;
        device.CreateTexture2D(&desc, None, Some(&mut tex))?;
        Ok(tex.ok_or("no staging texture")?)
    }
}

/// Reads a staging texture's `w` x `h` RGBA8 texels.
fn read_staging(ctx: &ID3D11DeviceContext, tex: &ID3D11Texture2D, w: u32, h: u32) -> Res<Vec<u8>> {
    unsafe {
        let mut m = D3D11_MAPPED_SUBRESOURCE::default();
        ctx.Map(tex, 0, D3D11_MAP_READ, 0, Some(&mut m))?;
        let row = w as usize * 4;
        let mut out = Vec::with_capacity(row * h as usize);
        for y in 0..h as usize {
            out.extend_from_slice(std::slice::from_raw_parts(m.pData.cast::<u8>().add(y * m.RowPitch as usize), row));
        }
        ctx.Unmap(tex, 0);
        Ok(out)
    }
}

/// A dynamic buffer of at least `size` bytes for `bind`, with a raw view
/// when `raw`.
fn dynamic(device: &ID3D11Device, size: u32, bind: D3D11_BIND_FLAG, raw: bool) -> Res<Dynamic> {
    let size = size.next_power_of_two().max(4096);
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: size,
        Usage: D3D11_USAGE_DYNAMIC,
        BindFlags: bind.0 as u32,
        CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
        MiscFlags: if raw { D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS.0 as u32 } else { 0 },
        StructureByteStride: 0,
    };
    unsafe {
        let mut buf = None;
        device.CreateBuffer(&desc, None, Some(&mut buf))?;
        let buf: ID3D11Buffer = buf.ok_or("no buffer")?;
        let srv = if raw { Some(raw_view(device, &buf, 0, size / 4)?) } else { None };
        Ok(Dynamic { buf, size, srv })
    }
}

/// A raw (`ByteAddressBuffer`) view of `count` words of `buf` from word `first`.
fn raw_view(device: &ID3D11Device, buf: &ID3D11Buffer, first: u32, count: u32) -> Res<ID3D11ShaderResourceView> {
    let desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
        Format: DXGI_FORMAT_R32_TYPELESS,
        ViewDimension: D3D11_SRV_DIMENSION_BUFFEREX,
        Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 { BufferEx: D3D11_BUFFEREX_SRV { FirstElement: first, NumElements: count.max(1), Flags: D3D11_BUFFEREX_SRV_FLAG_RAW.0 as u32 } },
    };
    unsafe {
        let mut srv = None;
        device.CreateShaderResourceView(buf, Some(&desc), Some(&mut srv))?;
        Ok(srv.ok_or("no buffer view")?)
    }
}

/// Writes `fill` into `d` (discarding what it held).
fn write(ctx: &ID3D11DeviceContext, d: &Dynamic, fill: impl FnOnce(&mut [u8])) -> Res<()> {
    unsafe {
        let mut m = D3D11_MAPPED_SUBRESOURCE::default();
        ctx.Map(&d.buf, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut m))?;
        fill(std::slice::from_raw_parts_mut(m.pData.cast::<u8>(), d.size as usize));
        ctx.Unmap(&d.buf, 0);
    }
    Ok(())
}

fn constant_buffer(device: &ID3D11Device, size: u32) -> Res<ID3D11Buffer> {
    let desc = D3D11_BUFFER_DESC { ByteWidth: size, Usage: D3D11_USAGE_DYNAMIC, BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32, CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32, MiscFlags: 0, StructureByteStride: 0 };
    unsafe {
        let mut buf = None;
        device.CreateBuffer(&desc, None, Some(&mut buf))?;
        Ok(buf.ok_or("no constant buffer")?)
    }
}

fn set_constants<T: Copy>(ctx: &ID3D11DeviceContext, buf: &ID3D11Buffer, v: &T) -> Res<()> {
    unsafe {
        let mut m = D3D11_MAPPED_SUBRESOURCE::default();
        ctx.Map(buf, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut m))?;
        std::ptr::copy_nonoverlapping((v as *const T).cast::<u8>(), m.pData.cast::<u8>(), std::mem::size_of::<T>());
        ctx.Unmap(buf, 0);
    }
    Ok(())
}

impl Renderer {
    pub fn new(display: RawDisplayHandle, window: RawWindowHandle, opts: Options) -> Res<Self> {
        let _ = display;
        let gpu = Gpu::create(opts.pipeline_cache.as_deref())?;
        Self::create(gpu, Some(window), opts)
    }

    pub fn headless(opts: Options) -> Res<Self> {
        let gpu = Gpu::create(opts.pipeline_cache.as_deref())?;
        Self::create(gpu, None, opts)
    }

    fn create(gpu: Arc<Gpu>, window: Option<RawWindowHandle>, opts: Options) -> Res<Self> {
        let hwnd = match window {
            Some(RawWindowHandle::Win32(h)) => Some(HWND(h.hwnd.get() as *mut c_void)),
            Some(_) => return Err("Direct3D 11 draws to Win32 windows only".into()),
            None => None,
        };
        let scale = opts.scale.max(0.25);
        let device = gpu.device.clone();
        let (ew, eh) = (at_scale(EFB_W, scale), at_scale(EFB_H, scale));
        let (efb_color, efb_depth) = Self::create_efb(&device, ew, eh)?;
        let native = create_texture(&device, EFB_W, EFB_H, 1, COLOR, true, None)?;
        let keys_path = opts.pipeline_cache.as_ref().map(|p| p.with_file_name("pipelines-d3d11.bin"));
        Ok(Self {
            device_name: gpu.device_name.clone(),
            validation: gpu.validation,
            scale,
            stretch: 1.0,
            efb_stale: false,
            vsync: opts.vsync,
            copies_to_ram: opts.efb_copies_to_ram,
            interp: constant_buffer(&device, 16)?,
            constants: constant_buffer(&device, 64)?,
            gpu,
            device,
            hwnd,
            swapchain: None,
            swap_rtv: None,
            swap_size: (0, 0),
            needs_rebuild: true,
            frame: 0,
            pending: Default::default(),
            blends: HashMap::new(),
            depths: HashMap::new(),
            rasters: HashMap::new(),
            samplers: HashMap::new(),
            efb_color,
            efb_depth,
            efb_size: (ew, eh),
            native,
            efb_ready: false,
            vertices: None,
            shadings: None,
            instance_ids: None,
            index_offset: 0,
            push: [0; 4],
            prepared: None,
            described: None,
            textures: HashMap::new(),
            textures_epoch: 0,
            xfbs: HashMap::new(),
            xfb_serial: 0,
            writebacks: Vec::new(),
            ui_textures: HashMap::new(),
            ui_buffer: None,
            ui_frame: None,
            keys_path,
            aspect: 16.0 / 9.0,
            screenshot: None,
            overlay: Vec::new(),
            stats: FrameStats::default(),
        })
    }

    #[allow(clippy::type_complexity)]
    fn create_efb(device: &ID3D11Device, w: u32, h: u32) -> Res<(Image, (ID3D11Texture2D, ID3D11DepthStencilView, ID3D11ShaderResourceView))> {
        let color = create_efb_color(device, w, h)?;
        let desc = D3D11_TEXTURE2D_DESC {
            Width: w,
            Height: h,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R32_TYPELESS,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_DEPTH_STENCIL.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
            ..Default::default()
        };
        unsafe {
            let mut tex = None;
            device.CreateTexture2D(&desc, None, Some(&mut tex))?;
            let tex: ID3D11Texture2D = tex.ok_or("no depth texture")?;
            let mut dsv = None;
            let dd = D3D11_DEPTH_STENCIL_VIEW_DESC { Format: DXGI_FORMAT_D32_FLOAT, ViewDimension: D3D11_DSV_DIMENSION_TEXTURE2D, ..Default::default() };
            device.CreateDepthStencilView(&tex, Some(&dd), Some(&mut dsv))?;
            let sd = D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_R32_FLOAT,
                ViewDimension: D3D11_SRV_DIMENSION_TEXTURE2D,
                Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0 { Texture2D: D3D11_TEX2D_SRV { MostDetailedMip: 0, MipLevels: 1 } },
            };
            let mut srv = None;
            device.CreateShaderResourceView(&tex, Some(&sd), Some(&mut srv))?;
            Ok((color, (tex, dsv.ok_or("no depth view")?, srv.ok_or("no depth view")?)))
        }
    }

    fn sx(&self) -> f32 {
        self.scale * self.stretch
    }
    fn sy(&self) -> f32 {
        self.scale
    }

    pub fn set_stretch(&mut self, stretch: f32) {
        if self.stretch != stretch {
            self.stretch = stretch;
            self.efb_stale = true;
        }
    }

    fn remake_efb(&mut self) -> Res<()> {
        let (ew, eh) = (at_scale(EFB_W, self.sx()), at_scale(EFB_H, self.sy()));
        (self.efb_color, self.efb_depth) = Self::create_efb(&self.device, ew, eh)?;
        self.efb_size = (ew, eh);
        self.efb_ready = false;
        self.efb_stale = false;
        Ok(())
    }

    fn sampler(&mut self, key: u32) -> Res<ID3D11SamplerState> {
        if let Some(s) = self.samplers.get(&key) {
            return Ok(s.clone());
        }
        let d = render::sampler_desc(key);
        let wrap = |w: u32| match w {
            0 => D3D11_TEXTURE_ADDRESS_CLAMP,
            1 => D3D11_TEXTURE_ADDRESS_WRAP,
            _ => D3D11_TEXTURE_ADDRESS_MIRROR,
        };
        let filter = u32::from(d.min_linear) << 4 | u32::from(d.mag_linear) << 2 | u32::from(d.mip_linear);
        let desc = D3D11_SAMPLER_DESC {
            Filter: D3D11_FILTER(filter as i32),
            AddressU: wrap(d.wrap_u),
            AddressV: wrap(d.wrap_v),
            AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
            MipLODBias: d.lod_bias,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D11_COMPARISON_NEVER,
            BorderColor: [0.0; 4],
            MinLOD: 0.0,
            MaxLOD: d.max_lod.unwrap_or(D3D11_FLOAT32_MAX),
        };
        let mut s = None;
        unsafe { self.device.CreateSamplerState(&desc, Some(&mut s))? };
        let s: ID3D11SamplerState = s.ok_or("no sampler")?;
        self.samplers.insert(key, s.clone());
        Ok(s)
    }

    fn blend_state(&mut self, key: u32) -> Res<ID3D11BlendState> {
        let key = key & 0x1_FFFF;
        if let Some(b) = self.blends.get(&key) {
            return Ok(b.clone());
        }
        let desc = blend_desc(key, self.gpu.logic_ops);
        let b: ID3D11BlendState = match self.device.cast::<ID3D11Device1>() {
            Ok(d1) => unsafe {
                let mut b = None;
                d1.CreateBlendState1(&desc, Some(&mut b))?;
                b.ok_or("no blend state")?.cast()?
            },
            Err(_) => unsafe {
                // Direct3D 11.0: no logic ops (`blend_desc` gave none).
                let r = desc.RenderTarget[0];
                let mut d = D3D11_BLEND_DESC::default();
                d.RenderTarget[0] = D3D11_RENDER_TARGET_BLEND_DESC {
                    BlendEnable: r.BlendEnable,
                    SrcBlend: r.SrcBlend,
                    DestBlend: r.DestBlend,
                    BlendOp: r.BlendOp,
                    SrcBlendAlpha: r.SrcBlendAlpha,
                    DestBlendAlpha: r.DestBlendAlpha,
                    BlendOpAlpha: r.BlendOpAlpha,
                    RenderTargetWriteMask: r.RenderTargetWriteMask,
                };
                let mut b = None;
                self.device.CreateBlendState(&d, Some(&mut b))?;
                b.ok_or("no blend state")?
            },
        };
        self.blends.insert(key, b.clone());
        Ok(b)
    }

    fn depth_state(&mut self, z: u32) -> Res<ID3D11DepthStencilState> {
        if let Some(d) = self.depths.get(&z) {
            return Ok(d.clone());
        }
        let desc = D3D11_DEPTH_STENCIL_DESC {
            DepthEnable: (z & 1 != 0).into(),
            DepthWriteMask: if z & 1 != 0 && z & 0x10 != 0 { D3D11_DEPTH_WRITE_MASK_ALL } else { D3D11_DEPTH_WRITE_MASK_ZERO },
            DepthFunc: D3D11_COMPARISON_FUNC(((z >> 1) & 7) as i32 + 1),
            ..Default::default()
        };
        let mut d = None;
        unsafe { self.device.CreateDepthStencilState(&desc, Some(&mut d))? };
        let d: ID3D11DepthStencilState = d.ok_or("no depth state")?;
        self.depths.insert(z, d.clone());
        Ok(d)
    }

    fn raster_state(&mut self, cull: u32) -> Res<ID3D11RasterizerState> {
        if let Some(r) = self.rasters.get(&cull) {
            return Ok(r.clone());
        }
        let mode = match cull {
            1 => D3D11_CULL_BACK,
            2 => D3D11_CULL_FRONT,
            _ => D3D11_CULL_NONE,
        };
        let mut r = None;
        unsafe { self.device.CreateRasterizerState(&rasterizer(mode), Some(&mut r))? };
        let r: ID3D11RasterizerState = r.ok_or("no rasterizer state")?;
        self.rasters.insert(cull, r.clone());
        Ok(r)
    }

    pub fn set_ui(&mut self, frame: Option<UiFrame>, textures: &egui::TexturesDelta) -> Res<()> {
        let gpu = self.gpu.clone();
        let ctx = gpu.lock();
        for (id, delta) in textures.set.iter().flat_map(|(id, d)| d.iter().map(move |d| (id, d))) {
            let (pixels, w, h, pos) = common::ui_pixels(delta);
            let (w, h) = (w as u32, h as u32);
            match pos {
                None => {
                    let data = D3D11_SUBRESOURCE_DATA { pSysMem: pixels.as_ptr().cast(), SysMemPitch: w * 4, SysMemSlicePitch: 0 };
                    let img = create_texture(&self.device, w, h, 1, DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, false, Some(&[data]))?;
                    self.ui_textures.insert(*id, img);
                }
                Some([x, y]) => {
                    if let Some(img) = self.ui_textures.get(id) {
                        let b = D3D11_BOX { left: x as u32, top: y as u32, front: 0, right: x as u32 + w, bottom: y as u32 + h, back: 1 };
                        unsafe { ctx.UpdateSubresource(&img.tex, 0, Some(&b), pixels.as_ptr().cast(), w * 4, 0) };
                    }
                }
            }
        }
        for id in &textures.free {
            self.ui_textures.remove(id);
        }
        self.ui_frame = frame;
        Ok(())
    }

    pub fn pipelines_compiling(&mut self) -> u32 {
        0
    }

    fn rebuild(&mut self, ctx: &ID3D11DeviceContext, width: u32, height: u32) -> Res<()> {
        let Some(hwnd) = self.hwnd else { return Ok(()) };
        self.swap_rtv = None;
        unsafe {
            ctx.OMSetRenderTargets(None, None);
            ctx.Flush();
            match &self.swapchain {
                Some(s) => s.ResizeBuffers(2, width, height, SWAP_FORMAT, DXGI_SWAP_CHAIN_FLAG(0))?,
                None => {
                    let mut desc = DXGI_SWAP_CHAIN_DESC1 {
                        Width: width,
                        Height: height,
                        Format: SWAP_FORMAT,
                        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                        BufferCount: 2,
                        Scaling: DXGI_SCALING_STRETCH,
                        SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                        AlphaMode: DXGI_ALPHA_MODE_IGNORE,
                        Flags: 0,
                        ..Default::default()
                    };
                    let s = match self.gpu.factory.CreateSwapChainForHwnd(&self.device, hwnd, &desc, None, None) {
                        Ok(s) => s,
                        // Before Windows 10: the bitblt model.
                        Err(_) => {
                            desc.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
                            desc.BufferCount = 1;
                            self.gpu.factory.CreateSwapChainForHwnd(&self.device, hwnd, &desc, None, None)?
                        }
                    };
                    let _ = self.gpu.factory.MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER);
                    self.swapchain = Some(s);
                }
            }
            let s = self.swapchain.as_ref().unwrap();
            let desc = s.GetDesc1()?;
            self.swap_size = (desc.Width, desc.Height);
            let back: ID3D11Texture2D = s.GetBuffer(0)?;
            let mut rtv = None;
            self.device.CreateRenderTargetView(&back, None, Some(&mut rtv))?;
            self.swap_rtv = rtv;
        }
        self.needs_rebuild = false;
        Ok(())
    }

    /// Frame `f`'s readbacks, now that the GPU is past it (`Map` waits).
    fn collect_writebacks(&mut self, ctx: &ID3D11DeviceContext, f: usize) -> Res<()> {
        for p in std::mem::take(&mut self.pending[f]) {
            let rgba = read_staging(ctx, &p.staging, p.w, p.h)?;
            self.writebacks.push(Writeback { addr: p.addr, ram_hash: p.ram_hash, tex_format: p.tex_format, width: p.w, height: p.h, stride: p.stride, rgba });
        }
        Ok(())
    }

    pub fn render(&mut self, out: &Output, vertices: &[Vertex], fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        self.render_with(out, vertices.len(), &mut |dst| dst.copy_from_slice(vertices), fresh, xfb_addr, width, height)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_with(&mut self, out: &Output, count: usize, fill: &mut dyn FnMut(&mut [Vertex]), fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let gpu = self.gpu.clone();
        let ctx = gpu.lock();
        let f = self.frame;
        if self.efb_stale {
            self.remake_efb()?;
        }
        self.collect_writebacks(&ctx, f)?;
        if fresh {
            self.upload_textures(out)?;
            for c in &out.cmds {
                if let Cmd::CopyTex(c) = c {
                    let (w, h) = (at_scale(c.w, self.sx()), at_scale(c.h, self.sy()));
                    if self.textures.get(&c.id).is_some_and(|i| i.w == w && i.h == h) {
                        continue;
                    }
                    let img = create_texture(&self.device, w, h, 1, COLOR, true, None)?;
                    self.textures_epoch += 1;
                    self.textures.insert(c.id, img);
                }
            }
        } else {
            self.stats.uploads = 0;
        }
        self.described = None;
        self.write_draw_states(&ctx, out)?;
        let vlen = count * VERTEX_SIZE as usize;
        self.index_offset = vlen.next_multiple_of(4) as u32;
        let total = (self.index_offset as usize + out.indices.len() * 4).max(VERTEX_SIZE as usize) as u32;
        if self.vertices.as_ref().is_none_or(|b| b.size < total) {
            self.vertices = Some(dynamic(&self.device, total, D3D11_BIND_FLAG(D3D11_BIND_VERTEX_BUFFER.0 | D3D11_BIND_INDEX_BUFFER.0), false)?);
        }
        let at = self.index_offset as usize;
        write(&ctx, self.vertices.as_ref().unwrap(), |dst| {
            let bytes = &mut dst[..vlen];
            assert!(bytes.as_ptr().align_offset(std::mem::align_of::<Vertex>()) == 0, "vertex buffer misaligned");
            // SAFETY: aligned (checked), large enough, `Vertex` is plain floats.
            let verts = unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast::<Vertex>(), count) };
            fill(verts);
            let idx = as_bytes(&out.indices);
            dst[at..at + idx.len()].copy_from_slice(idx);
        })?;
        let vb = self.vertices.as_ref().unwrap().buf.clone();
        self.record_efb(&ctx, f, &vb, None, out, fresh)?;
        let presented = self.present(&ctx, xfb_addr, width, height)?;
        drain_messages(&self.gpu);
        self.frame = (f + 1) % FRAMES;
        Ok(presented)
    }

    pub fn prepared(&self) -> Option<(u64, u64)> {
        self.prepared.as_ref().map(|p| p.key)
    }

    pub fn prepare(&mut self, key: (u64, u64), out: &Output, moves: Option<&gx::interp::Moves>) -> Res<()> {
        self.prepare_with(key, out, &out.vertices, moves)
    }

    pub fn prepare_with(&mut self, key: (u64, u64), out: &Output, vertices: &[Vertex], moves: Option<&gx::interp::Moves>) -> Res<()> {
        let gpu = self.gpu.clone();
        let ctx = gpu.lock();
        let vlen = vertices.len() * VERTEX_SIZE as usize;
        let index_offset = vlen.next_multiple_of(4);
        let need = (index_offset + out.indices.len() * 4).max(16) as u32;
        // The last prepared buffer is used again when big enough: frames
        // already drawn from it keep what they read (`Map` discards).
        let old = self.prepared.take();
        let serial = old.as_ref().map_or(1, |p| p.serial + 1);
        let vb = match old {
            Some(p) if p.vb.size >= need => p.vb,
            _ => dynamic(&self.device, need, D3D11_BIND_FLAG(D3D11_BIND_VERTEX_BUFFER.0 | D3D11_BIND_INDEX_BUFFER.0), false)?,
        };
        write(&ctx, &vb, |dst| {
            dst[..vlen].copy_from_slice(as_bytes(vertices));
            let idx = as_bytes(&out.indices);
            dst[index_offset..index_offset + idx.len()].copy_from_slice(idx);
        })?;
        let parts: Option<[&[u8]; 3]> = moves.filter(|m| !m.mats.is_empty()).map(|m| [as_bytes(&m.model), as_bytes(&m.mats), as_bytes(&m.recs)]);
        let (mut views, mut moves_buf) = (None, None);
        if let Some(parts) = parts {
            let mut at = 0usize;
            let mut ranges = [(0usize, 0usize); 3];
            for (r, p) in ranges.iter_mut().zip(&parts) {
                at = at.next_multiple_of(16);
                *r = (at, p.len().max(16));
                at += r.1;
            }
            let b = dynamic(&self.device, at as u32, D3D11_BIND_SHADER_RESOURCE, true)?;
            write(&ctx, &b, |dst| {
                for ((o, _), p) in ranges.iter().zip(&parts) {
                    dst[*o..*o + p.len()].copy_from_slice(p);
                }
            })?;
            let v = |i: usize| raw_view(&self.device, &b.buf, (ranges[i].0 / 4) as u32, (ranges[i].1 / 4) as u32);
            views = Some([v(0)?, v(1)?, v(2)?]);
            moves_buf = Some(b);
        }
        self.prepared = Some(Prepared { key, serial, vb, index_offset: index_offset as u32, moves: views, _moves_buf: moves_buf });
        self.described = None;
        Ok(())
    }

    pub fn render_prepared(&mut self, out: &Output, t: f32, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let gpu = self.gpu.clone();
        let ctx = gpu.lock();
        let f = self.frame;
        if self.efb_stale {
            self.remake_efb()?;
        }
        self.collect_writebacks(&ctx, f)?;
        self.stats.uploads = 0;
        let p = self.prepared.as_ref().ok_or("render_prepared: nothing prepared")?;
        let (serial, vb, index_offset, moves) = (p.serial, p.vb.buf.clone(), p.index_offset, p.moves.clone());
        let described = (serial, self.textures_epoch);
        if self.described != Some(described) {
            self.write_draw_states(&ctx, out)?;
            self.described = Some(described);
        }
        self.index_offset = index_offset;
        self.push = [t.to_bits(), u32::from(moves.is_some()), 0, 0];
        let recorded = self.record_efb(&ctx, f, &vb, moves.as_ref(), out, false);
        self.push = [0; 4];
        recorded?;
        let presented = self.present(&ctx, xfb_addr, width, height)?;
        drain_messages(&self.gpu);
        self.frame = (f + 1) % FRAMES;
        Ok(presented)
    }

    pub fn evict(&mut self, ids: &[u32]) {
        for id in ids {
            if self.textures.remove(id).is_some() {
                self.textures_epoch += 1;
            }
        }
    }

    pub fn take_writebacks(&mut self) -> Vec<Writeback> {
        std::mem::take(&mut self.writebacks)
    }

    pub fn finish(&mut self) -> Res<()> {
        let gpu = self.gpu.clone();
        let ctx = gpu.lock();
        for f in 0..FRAMES {
            self.collect_writebacks(&ctx, f)?;
        }
        drain_messages(&self.gpu);
        Ok(())
    }

    pub fn efb_snapshot(&mut self) -> Res<EfbSnapshot> {
        let gpu = self.gpu.clone();
        let ctx = gpu.lock();
        let full = Rect { x: 0, y: 0, w: EFB_W, h: EFB_H };
        let src = self.scaled_f(full);
        let mut planes = Vec::new();
        for flags in [4u32, 2 | 4] {
            let params = CopyParams { src, dst: [EFB_W as f32, EFB_H as f32], format: 6, flags };
            let s = self.copy_to_native(&ctx, params)?;
            planes.push(read_staging(&ctx, &s, EFB_W, EFB_H)?);
        }
        let d = planes.pop().unwrap();
        let color = planes.pop().unwrap();
        let depth = d.chunks_exact(4).map(|p| u32::from(p[0]) << 16 | u32::from(p[1]) << 8 | u32::from(p[2])).collect();
        drain_messages(&self.gpu);
        Ok(EfbSnapshot { color, depth })
    }

    fn upload_textures(&mut self, out: &Output) -> Res<()> {
        self.stats.uploads = out.new_textures.len();
        for t in &out.new_textures {
            let (width, height, levels) = match &t.replacement {
                Some(r) => (r.width, r.height, r.levels.as_slice()),
                None => (t.width, t.height, t.levels.as_slice()),
            };
            if levels.is_empty() {
                continue;
            }
            let mut data = Vec::new();
            let mut w = width;
            for l in levels {
                data.push(D3D11_SUBRESOURCE_DATA { pSysMem: l.as_ptr().cast(), SysMemPitch: w * 4, SysMemSlicePitch: 0 });
                w = (w / 2).max(1);
            }
            // Render targets too: EFB copies may be patched into them.
            let img = create_texture(&self.device, width, height, levels.len() as u32, COLOR, true, Some(&data))?;
            self.textures_epoch += 1;
            self.textures.insert(t.id, img);
        }
        Ok(())
    }

    /// The draw states: each texture slot is its map's own (0..7) or none;
    /// the EFB scale filled in.
    fn write_draw_states(&mut self, ctx: &ID3D11DeviceContext, out: &Output) -> Res<()> {
        let mut words: Vec<u32> = Vec::with_capacity(out.shadings.len().max(1) * SHADING_WORDS);
        let (scale_x, scale_y) = (self.sx().to_bits(), self.sy().to_bits());
        let mut used = std::collections::HashSet::new();
        for s in &out.shadings {
            let mut w = s.0;
            w[sh::SCALE] = scale_x;
            w[sh::SCALE_Y] = scale_y;
            for m in 0..8 {
                let id = w[sh::TEX + m];
                if id == NO_TEXTURE {
                    continue;
                }
                w[sh::TEX + m] = if self.textures.contains_key(&id) {
                    used.insert(id);
                    m as u32
                } else {
                    NO_TEXTURE
                };
            }
            words.extend_from_slice(&w);
        }
        if words.is_empty() {
            words.resize(SHADING_WORDS, 0);
        }
        self.stats.textures = used.len();
        let bytes = words.len() * 4;
        if self.shadings.as_ref().is_none_or(|b| (b.size as usize) < bytes) {
            self.shadings = Some(dynamic(&self.device, bytes as u32, D3D11_BIND_SHADER_RESOURCE, true)?);
        }
        write(ctx, self.shadings.as_ref().unwrap(), |dst| dst[..bytes].copy_from_slice(as_bytes(&words)))?;
        let n = (words.len() / SHADING_WORDS) as u32;
        if self.instance_ids.as_ref().is_none_or(|(_, len)| *len < n) {
            let len = n.next_power_of_two().max(1024);
            let ids: Vec<u32> = (0..len).collect();
            let desc = D3D11_BUFFER_DESC { ByteWidth: len * 4, Usage: D3D11_USAGE_IMMUTABLE, BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32, ..Default::default() };
            let data = D3D11_SUBRESOURCE_DATA { pSysMem: ids.as_ptr().cast(), ..Default::default() };
            let mut buf = None;
            unsafe { self.device.CreateBuffer(&desc, Some(&data), Some(&mut buf))? };
            self.instance_ids = Some((buf.ok_or("no buffer")?, len));
        }
        Ok(())
    }

    fn scaled(&self, r: Rect) -> RECT {
        let edge = |v: u32, s: f32, size: u32| ((v as f32 * s).round() as u32).min(size);
        let (iw, ih) = self.efb_size;
        let (x0, y0) = (edge(r.x, self.sx(), iw - 1), edge(r.y, self.sy(), ih - 1));
        let (x1, y1) = (edge(r.x + r.w, self.sx(), iw).max(x0 + 1), edge(r.y + r.h, self.sy(), ih).max(y0 + 1));
        RECT { left: x0 as i32, top: y0 as i32, right: x1 as i32, bottom: y1 as i32 }
    }

    fn scaled_f(&self, r: Rect) -> [f32; 4] {
        let (sx, sy) = (self.sx(), self.sy());
        [r.x as f32 * sx, r.y as f32 * sy, r.w as f32 * sx, r.h as f32 * sy]
    }

    /// Binds the EFB and everything the draws use.
    fn begin_efb(&mut self, ctx: &ID3D11DeviceContext, vb: &ID3D11Buffer, moves: Option<&[ID3D11ShaderResourceView; 3]>) -> Res<()> {
        let g = &self.gpu;
        set_constants(ctx, &self.interp, &self.push)?;
        unsafe {
            // The copy pass may have left the EFB bound as a texture.
            ctx.PSSetShaderResources(0, Some(&[None, None]));
            ctx.OMSetRenderTargets(Some(&[self.efb_color.rtv.clone()]), &self.efb_depth.1);
            ctx.RSSetViewports(Some(&[viewport(0.0, 0.0, self.efb_size.0 as f32, self.efb_size.1 as f32)]));
            ctx.IASetInputLayout(&g.uber_layout);
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            let ids = &self.instance_ids.as_ref().unwrap().0;
            let bufs = [Some(vb.clone()), Some(ids.clone())];
            let strides = [VERTEX_SIZE, 4];
            let offsets = [0u32, 0];
            ctx.IASetVertexBuffers(0, 2, Some(bufs.as_ptr()), Some(strides.as_ptr()), Some(offsets.as_ptr()));
            ctx.IASetIndexBuffer(vb, DXGI_FORMAT_R32_UINT, self.index_offset);
            ctx.VSSetShader(&g.uber_vs, None);
            ctx.VSSetConstantBuffers(0, Some(&[Some(self.interp.clone())]));
            let shadings = self.shadings.as_ref().unwrap().srv.clone();
            let m = moves.map(|m| [Some(m[0].clone()), Some(m[1].clone()), Some(m[2].clone())]).unwrap_or([shadings.clone(), shadings.clone(), shadings.clone()]);
            ctx.VSSetShaderResources(1, Some(&m));
            ctx.PSSetShaderResources(0, Some(&[shadings]));
        }
        Ok(())
    }

    /// The copy pass from the EFB into `params.dst` texels at the top left of `rtv`.
    fn copy_pass(&mut self, ctx: &ID3D11DeviceContext, rtv: &ID3D11RenderTargetView, params: CopyParams) -> Res<()> {
        let g = &self.gpu;
        let (w, h) = (params.dst[0], params.dst[1]);
        set_constants(ctx, &self.constants, &params)?;
        unsafe {
            // The target may be bound as a draw's texture.
            ctx.PSSetShaderResources(0, Some(&NO_VIEWS));
            ctx.OMSetRenderTargets(Some(&[Some(rtv.clone())]), None);
            ctx.PSSetShaderResources(0, Some(&[self.efb_color.srv.clone(), Some(self.efb_depth.2.clone())]));
            ctx.VSSetShader(&g.copy_vs, None);
            ctx.PSSetShader(&g.copy_fs, None);
            ctx.PSSetConstantBuffers(0, Some(&[Some(self.constants.clone())]));
            ctx.IASetInputLayout(None);
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            ctx.RSSetState(&g.no_cull);
            ctx.RSSetViewports(Some(&[viewport(0.0, 0.0, w, h)]));
            ctx.RSSetScissorRects(Some(&[rect(0, 0, w as u32, h as u32)]));
            ctx.OMSetBlendState(&g.opaque, None, u32::MAX);
            ctx.OMSetDepthStencilState(&g.no_depth, 0);
            ctx.Draw(3, 0);
            ctx.PSSetShaderResources(0, Some(&[None, None]));
        }
        Ok(())
    }

    /// The copy pass at native size into a new staging texture.
    fn copy_to_native(&mut self, ctx: &ID3D11DeviceContext, params: CopyParams) -> Res<ID3D11Texture2D> {
        let rtv = self.native.rtv.clone().unwrap();
        self.copy_pass(ctx, &rtv, params)?;
        let (w, h) = (params.dst[0] as u32, params.dst[1] as u32);
        let s = staging(&self.device, w, h)?;
        let b = D3D11_BOX { left: 0, top: 0, front: 0, right: w, bottom: h, back: 1 };
        unsafe { ctx.CopySubresourceRegion(&s, 0, 0, 0, 0, &self.native.tex, 0, Some(&b)) };
        Ok(s)
    }

    /// A bilinear blit of `src` (x, y, w, h of a `sw` x `sh` image) onto
    /// `dst` (x, y, w, h) of `rtv`.
    #[allow(clippy::too_many_arguments)]
    fn blit(&mut self, ctx: &ID3D11DeviceContext, srv: &ID3D11ShaderResourceView, src: (u32, u32, u32, u32), sw: u32, sh: u32, rtv: &ID3D11RenderTargetView, dst: (u32, u32, u32, u32)) -> Res<()> {
        let g = &self.gpu;
        set_constants(ctx, &self.constants, &BlitParams::new(src, sw, sh, dst))?;
        unsafe {
            // The target may be bound as a draw's texture.
            ctx.PSSetShaderResources(0, Some(&NO_VIEWS));
            ctx.OMSetRenderTargets(Some(&[Some(rtv.clone())]), None);
            ctx.PSSetShaderResources(0, Some(&[Some(srv.clone())]));
            ctx.PSSetSamplers(0, Some(&[Some(g.bilinear.clone())]));
            ctx.VSSetShader(&g.blit_vs, None);
            ctx.PSSetShader(&g.blit_ps, None);
            ctx.PSSetConstantBuffers(0, Some(&[Some(self.constants.clone())]));
            ctx.IASetInputLayout(None);
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            ctx.RSSetState(&g.no_cull);
            ctx.RSSetViewports(Some(&[viewport(dst.0 as f32, dst.1 as f32, dst.2 as f32, dst.3 as f32)]));
            ctx.RSSetScissorRects(Some(&[rect(dst.0, dst.1, dst.2, dst.3)]));
            ctx.OMSetBlendState(&g.opaque, None, u32::MAX);
            ctx.OMSetDepthStencilState(&g.no_depth, 0);
            ctx.Draw(3, 0);
            ctx.PSSetShaderResources(0, Some(&[None]));
        }
        Ok(())
    }

    /// Fills `r` of the bound targets: `color` (None: kept) and depth
    /// (None: kept), with a draw.
    fn clear_rect(&mut self, ctx: &ID3D11DeviceContext, r: RECT, color: Option<[f32; 4]>, depth: Option<f32>) -> Res<()> {
        let g = self.gpu.clone();
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct C {
            color: [f32; 4],
            depth: [f32; 4],
        }
        set_constants(ctx, &self.constants, &C { color: color.unwrap_or([0.0; 4]), depth: [depth.unwrap_or(0.0), 0.0, 0.0, 0.0] })?;
        // Write masks: colour or nothing; depth always-passing or untouched.
        let blend_key = if color.is_some() { 0x18 } else { 0 };
        let blend = self.blend_state(blend_key)?;
        let z = if depth.is_some() { 0x1F } else { 0 };
        let ds = self.depth_state(z)?;
        unsafe {
            ctx.VSSetShader(&g.clear_vs, None);
            ctx.PSSetShader(&g.clear_ps, None);
            ctx.VSSetConstantBuffers(0, Some(&[Some(self.constants.clone())]));
            ctx.PSSetConstantBuffers(0, Some(&[Some(self.constants.clone())]));
            ctx.IASetInputLayout(None);
            ctx.RSSetState(&g.no_cull);
            let (w, h) = ((r.right - r.left) as f32, (r.bottom - r.top) as f32);
            ctx.RSSetViewports(Some(&[viewport(r.left as f32, r.top as f32, w, h)]));
            ctx.RSSetScissorRects(Some(&[r]));
            ctx.OMSetBlendState(&blend, None, u32::MAX);
            ctx.OMSetDepthStencilState(&ds, 0);
            ctx.Draw(3, 0);
        }
        Ok(())
    }

    fn record_efb(&mut self, ctx: &ID3D11DeviceContext, f: usize, vb: &ID3D11Buffer, moves: Option<&[ID3D11ShaderResourceView; 3]>, out: &Output, fresh: bool) -> Res<()> {
        if !self.efb_ready {
            unsafe {
                ctx.ClearRenderTargetView(self.efb_color.rtv.as_ref().unwrap(), &[0.0, 0.0, 0.0, 1.0]);
                ctx.ClearDepthStencilView(&self.efb_depth.1, D3D11_CLEAR_DEPTH.0 as u32, 1.0, 0);
            }
            self.efb_ready = true;
        }
        let readback = fresh && self.copies_to_ram;
        let mut drawing = false;
        // Whether the EFB's UINT view is bound (logic-op draws).
        let mut uint_bound = false;
        // Per draw state last set: (blend key, cull, z mode, scissor), and
        // the eight textures and samplers.
        let mut last: Option<(u32, u32, u32, RECT)> = None;
        let mut bound_tex = [u32::MAX - 1; 8];
        let mut bound_smp = [u32::MAX - 1; 8];
        let mut draws = 0;
        let full = RECT { left: 0, top: 0, right: self.efb_size.0 as i32, bottom: self.efb_size.1 as i32 };
        for c in &out.cmds {
            match *c {
                Cmd::Draw(d) => {
                    if d.scissor.w == 0 || d.scissor.h == 0 || d.raster.cull == 3 {
                        continue;
                    }
                    if !drawing {
                        self.begin_efb(ctx, vb, moves)?;
                        drawing = true;
                        uint_bound = false;
                        last = None;
                        bound_tex = [u32::MAX - 1; 8];
                        bound_smp = [u32::MAX - 1; 8];
                    }
                    let Some(s) = out.shadings.get(d.shading as usize) else { continue };
                    let early = rasterised_depth(&s.0);
                    let key = blend_key(&d.raster) | if early { EARLY_DEPTH } else { 0 };
                    let scissor = self.scaled(d.scissor);
                    let state = (key, d.raster.cull, d.raster.zmode, scissor);
                    let prev = last.unwrap_or((u32::MAX, u32::MAX, u32::MAX, RECT::default()));
                    unsafe {
                        if prev.0 != key || last.is_none() {
                            let b = self.blend_state(key)?;
                            ctx.OMSetBlendState(&b, None, u32::MAX);
                            let uint = self.gpu.logic_ops && matches!(render::blend_of(key).0, Blend::Logic(_));
                            let ps = if uint {
                                &self.gpu.uber_fs_logic
                            } else if early {
                                &self.gpu.uber_fs_early
                            } else {
                                &self.gpu.uber_fs
                            };
                            ctx.PSSetShader(ps, None);
                            if uint != uint_bound {
                                let rtv = if uint { &self.efb_color.rtv_uint } else { &self.efb_color.rtv };
                                ctx.OMSetRenderTargets(Some(&[rtv.clone()]), &self.efb_depth.1);
                                uint_bound = uint;
                            }
                        }
                        if prev.1 != d.raster.cull {
                            let r = self.raster_state(d.raster.cull)?;
                            ctx.RSSetState(&r);
                        }
                        if prev.2 != d.raster.zmode {
                            let z = self.depth_state(d.raster.zmode)?;
                            ctx.OMSetDepthStencilState(&z, 0);
                        }
                        if prev.3 != scissor || last.is_none() {
                            ctx.RSSetScissorRects(Some(&[scissor]));
                        }
                    }
                    last = Some(state);
                    // Textures at slots 4..11, samplers at 0..7.
                    for m in 0..8 {
                        let (id, key) = (s.0[sh::TEX + m], s.0[sh::SAMPLER + m]);
                        let id = if self.textures.contains_key(&id) { id } else { NO_TEXTURE };
                        if bound_tex[m] != id {
                            let v = self.textures.get(&id).and_then(|t| t.srv.clone());
                            unsafe { ctx.PSSetShaderResources(4 + m as u32, Some(&[v])) };
                            bound_tex[m] = id;
                        }
                        if id != NO_TEXTURE && bound_smp[m] != key {
                            let smp = self.sampler(key)?;
                            unsafe { ctx.PSSetSamplers(m as u32, Some(&[Some(smp)])) };
                            bound_smp[m] = key;
                        }
                    }
                    unsafe { ctx.DrawIndexedInstanced(d.count, 1, d.first, 0, d.shading) };
                    draws += 1;
                }
                Cmd::Clear(c) => {
                    if !drawing {
                        self.begin_efb(ctx, vb, moves)?;
                        drawing = true;
                        bound_tex = [u32::MAX - 1; 8];
                        bound_smp = [u32::MAX - 1; 8];
                    }
                    let r = self.scaled(c.rect);
                    let color = (c.color_update || c.alpha_update).then(|| c.color.map(|v| f32::from(v) / 255.0));
                    let depth = c.z_update.then(|| c.z as f32 / 16_777_216.0);
                    if r == full {
                        unsafe {
                            if let Some(col) = color {
                                ctx.ClearRenderTargetView(self.efb_color.rtv.as_ref().unwrap(), &col);
                            }
                            if let Some(z) = depth {
                                ctx.ClearDepthStencilView(&self.efb_depth.1, D3D11_CLEAR_DEPTH.0 as u32, z, 0);
                            }
                        }
                    } else if color.is_some() || depth.is_some() {
                        self.clear_rect(ctx, r, color, depth)?;
                        // The draws' own shaders and states again.
                        drawing = false;
                    }
                }
                Cmd::CopyXfb { src, addr, height } => {
                    drawing = false;
                    let (w, h) = (at_scale(src.w, self.sx()), at_scale(height, self.sy()));
                    if self.xfbs.get(&addr).is_some_and(|(img, _)| img.w != w || img.h != h) {
                        self.xfbs.remove(&addr);
                    }
                    if !self.xfbs.contains_key(&addr) {
                        if self.xfbs.len() >= MAX_XFBS {
                            let oldest = *self.xfbs.iter().min_by_key(|(_, (_, s))| *s).unwrap().0;
                            self.xfbs.remove(&oldest);
                        }
                        let img = create_texture(&self.device, w, h, 1, COLOR, true, None)?;
                        self.xfbs.insert(addr, (img, 0));
                    }
                    self.xfb_serial += 1;
                    let serial = self.xfb_serial;
                    let s = self.scaled(src);
                    let src_rect = (s.left as u32, s.top as u32, (s.right - s.left) as u32, (s.bottom - s.top) as u32);
                    let (dst, ser) = self.xfbs.get_mut(&addr).unwrap();
                    *ser = serial;
                    let rtv = dst.rtv.clone().unwrap();
                    let srv = self.efb_color.srv.clone().unwrap();
                    let (ew, eh) = self.efb_size;
                    unsafe { ctx.OMSetRenderTargets(None, None) };
                    self.blit(ctx, &srv, src_rect, ew, eh, &rtv, (0, 0, w, h))?;
                }
                Cmd::CopyTex(c) => {
                    drawing = false;
                    let flags = u32::from(c.intensity) | u32::from(c.depth) << 1 | u32::from(c.efb_alpha) << 2;
                    let src = self.scaled_f(c.src);
                    unsafe { ctx.OMSetRenderTargets(None, None) };
                    if let Some(dst) = self.textures.get(&c.id) {
                        let params = CopyParams { src, dst: [dst.w as f32, dst.h as f32], format: c.format, flags };
                        let rtv = dst.rtv.clone().unwrap();
                        self.copy_pass(ctx, &rtv, params)?;
                    }
                    if readback {
                        let params = CopyParams { src, dst: [c.w as f32, c.h as f32], format: c.format, flags };
                        let staging = self.copy_to_native(ctx, params)?;
                        self.pending[f].push(PendingWriteback { addr: c.addr, ram_hash: c.ram_hash, tex_format: c.tex_format, w: c.w, h: c.h, stride: c.stride, staging });
                    }
                }
                Cmd::Patch(p) => {
                    drawing = false;
                    if p.copy != p.target {
                        if let (Some(src), Some(dst)) = (self.textures.get(&p.copy), self.textures.get(&p.target)) {
                            if p.x + p.w <= dst.w && p.y + p.h <= dst.h {
                                let (srv, sw, sh, rtv) = (src.srv.clone().unwrap(), src.w, src.h, dst.rtv.clone().unwrap());
                                unsafe { ctx.OMSetRenderTargets(None, None) };
                                self.blit(ctx, &srv, (0, 0, sw, sh), sw, sh, &rtv, (p.x, p.y, p.w, p.h))?;
                            }
                        }
                    }
                }
            }
        }
        unsafe {
            ctx.OMSetRenderTargets(None, None);
            // Nothing left bound that a later frame writes to.
            ctx.PSSetShaderResources(0, Some(&NO_VIEWS));
        }
        self.stats.draws = draws;
        self.stats.pipelines = self.blends.len();
        Ok(())
    }

    fn draw_ui(&mut self, ctx: &ID3D11DeviceContext, rtv: &ID3D11RenderTargetView) -> Res<()> {
        let Some(frame) = &self.ui_frame else { return Ok(()) };
        if frame.primitives.is_empty() {
            return Ok(());
        }
        let ppp = frame.pixels_per_point;
        let (tw, th) = self.swap_size;
        let mut vbytes = Vec::new();
        let mut ibytes: Vec<u8> = Vec::new();
        let mut draws = Vec::new();
        for p in &frame.primitives {
            let egui::epaint::Primitive::Mesh(mesh) = &p.primitive else { continue };
            if mesh.indices.is_empty() {
                continue;
            }
            let c = p.clip_rect;
            let x0 = (c.min.x * ppp).round().clamp(0.0, tw as f32) as i32;
            let y0 = (c.min.y * ppp).round().clamp(0.0, th as f32) as i32;
            let x1 = (c.max.x * ppp).round().clamp(0.0, tw as f32) as i32;
            let y1 = (c.max.y * ppp).round().clamp(0.0, th as f32) as i32;
            if x1 <= x0 || y1 <= y0 {
                continue;
            }
            let base = (vbytes.len() / UI_VERTEX_SIZE as usize) as i32;
            vbytes.extend_from_slice(as_bytes(&mesh.vertices));
            let first = (ibytes.len() / 4) as u32;
            ibytes.extend_from_slice(as_bytes(&mesh.indices));
            draws.push((mesh.texture_id, RECT { left: x0, top: y0, right: x1, bottom: y1 }, first, mesh.indices.len() as u32, base));
        }
        if draws.is_empty() {
            return Ok(());
        }
        let ioff = vbytes.len().next_multiple_of(16);
        let need = (ioff + ibytes.len()) as u32;
        if self.ui_buffer.as_ref().is_none_or(|b| b.size < need) {
            self.ui_buffer = Some(dynamic(&self.device, need, D3D11_BIND_FLAG(D3D11_BIND_VERTEX_BUFFER.0 | D3D11_BIND_INDEX_BUFFER.0), false)?);
        }
        let b = self.ui_buffer.as_ref().unwrap();
        write(ctx, b, |dst| {
            dst[..vbytes.len()].copy_from_slice(&vbytes);
            dst[ioff..ioff + ibytes.len()].copy_from_slice(&ibytes);
        })?;
        let screen = [tw as f32 / ppp, th as f32 / ppp, 0.0, 0.0];
        set_constants(ctx, &self.constants, &screen)?;
        let g = &self.gpu;
        unsafe {
            ctx.OMSetRenderTargets(Some(&[Some(rtv.clone())]), None);
            ctx.VSSetShader(&g.ui_vs, None);
            ctx.PSSetShader(&g.ui_ps, None);
            ctx.VSSetConstantBuffers(0, Some(&[Some(self.constants.clone())]));
            ctx.IASetInputLayout(&g.ui_layout);
            ctx.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            let bufs = [Some(b.buf.clone())];
            ctx.IASetVertexBuffers(0, 1, Some(bufs.as_ptr()), Some(&UI_VERTEX_SIZE), Some(&0));
            ctx.IASetIndexBuffer(&b.buf, DXGI_FORMAT_R32_UINT, ioff as u32);
            ctx.RSSetState(&g.no_cull);
            ctx.RSSetViewports(Some(&[viewport(0.0, 0.0, tw as f32, th as f32)]));
            ctx.OMSetBlendState(&g.ui_blend, None, u32::MAX);
            ctx.OMSetDepthStencilState(&g.no_depth, 0);
            ctx.PSSetSamplers(0, Some(&[Some(g.bilinear.clone())]));
        }
        for (tex, clip, first, count, base) in draws {
            let Some(srv) = self.ui_textures.get(&tex).and_then(|t| t.srv.clone()) else { continue };
            unsafe {
                ctx.PSSetShaderResources(0, Some(&[Some(srv)]));
                ctx.RSSetScissorRects(Some(&[clip]));
                ctx.DrawIndexed(count, first, base);
            }
        }
        Ok(())
    }

    fn present(&mut self, ctx: &ID3D11DeviceContext, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let xfb = self.xfbs.get(&xfb_addr).map(|_| xfb_addr).or_else(|| self.xfbs.iter().max_by_key(|(_, (_, s))| *s).map(|(a, _)| *a));
        if let Some(addr) = xfb.filter(|_| self.screenshot.is_some()) {
            let path = self.screenshot.take().unwrap();
            let img = &self.xfbs[&addr].0;
            let s = staging(&self.device, img.w, img.h)?;
            unsafe { ctx.CopyResource(&s, &img.tex) };
            let pixels = read_staging(ctx, &s, img.w, img.h)?;
            render::save_png(&path, img.w, img.h, &pixels)?;
        }
        if width == 0 || height == 0 || self.hwnd.is_none() {
            return Ok(false);
        }
        if self.needs_rebuild || self.swap_size != (width, height) {
            self.rebuild(ctx, width, height)?;
        }
        let Some(rtv) = self.swap_rtv.clone() else { return Ok(false) };
        let (tw, th) = self.swap_size;
        unsafe { ctx.ClearRenderTargetView(&rtv, &[0.0, 0.0, 0.0, 1.0]) };
        if let Some(addr) = xfb {
            let src = &self.xfbs[&addr].0;
            let (srv, sw, sh) = (src.srv.clone().unwrap(), src.w, src.h);
            let dst = render::letterbox(tw, th, self.aspect);
            self.blit(ctx, &srv, (0, 0, sw, sh), sw, sh, &rtv, dst)?;
        }
        for (x0, y0, x1, y1, c) in render::overlay_rects(&self.overlay, tw, th) {
            let col = c.map(|v| f32::from(v) / 255.0);
            unsafe { ctx.OMSetRenderTargets(Some(&[Some(rtv.clone())]), None) };
            self.clear_rect(ctx, RECT { left: x0, top: y0, right: x1, bottom: y1 }, Some(col), None)?;
        }
        self.draw_ui(ctx, &rtv)?;
        unsafe {
            ctx.OMSetRenderTargets(None, None);
            let r = self.swapchain.as_ref().unwrap().Present(u32::from(self.vsync), DXGI_PRESENT(0));
            if r.is_err() {
                self.needs_rebuild = true;
                r.ok()?;
            }
        }
        Ok(true)
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        drain_messages(&self.gpu);
        if let Some(path) = &self.keys_path {
            render::save_keys(path, self.blends.keys().copied());
        }
    }
}

impl Backend for Renderer {
    fn api(&self) -> Api {
        Api::D3d11
    }
    fn device_name(&self) -> &str {
        &self.device_name
    }
    fn validation(&self) -> bool {
        self.validation
    }
    fn scale(&self) -> f32 {
        self.scale
    }
    fn set_stretch(&mut self, stretch: f32) {
        Renderer::set_stretch(self, stretch)
    }
    fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
    fn set_screenshot(&mut self, path: Option<PathBuf>) {
        self.screenshot = path;
    }
    fn set_overlay(&mut self, rects: Vec<OverlayRect>) {
        self.overlay = rects;
    }
    fn stats(&self) -> FrameStats {
        self.stats
    }
    fn set_vsync(&mut self, on: bool) {
        self.vsync = on;
    }
    fn resized(&mut self) {
        self.needs_rebuild = true;
    }
    fn set_ui(&mut self, frame: Option<UiFrame>, textures: &egui::TexturesDelta) -> Res<()> {
        Renderer::set_ui(self, frame, textures)
    }
    fn pipelines_compiling(&mut self) -> u32 {
        Renderer::pipelines_compiling(self)
    }
    fn render(&mut self, out: &Output, vertices: &[Vertex], fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        Renderer::render(self, out, vertices, fresh, xfb_addr, width, height)
    }
    fn render_with(&mut self, out: &Output, count: usize, fill: &mut dyn FnMut(&mut [Vertex]), fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        Renderer::render_with(self, out, count, fill, fresh, xfb_addr, width, height)
    }
    fn prepared(&self) -> Option<(u64, u64)> {
        Renderer::prepared(self)
    }
    fn prepare(&mut self, key: (u64, u64), out: &Output, moves: Option<&gx::interp::Moves>) -> Res<()> {
        Renderer::prepare(self, key, out, moves)
    }
    fn prepare_with(&mut self, key: (u64, u64), out: &Output, vertices: &[Vertex], moves: Option<&gx::interp::Moves>) -> Res<()> {
        Renderer::prepare_with(self, key, out, vertices, moves)
    }
    fn render_prepared(&mut self, out: &Output, t: f32, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        Renderer::render_prepared(self, out, t, xfb_addr, width, height)
    }
    fn evict(&mut self, ids: &[u32]) {
        Renderer::evict(self, ids)
    }
    fn take_writebacks(&mut self) -> Vec<Writeback> {
        Renderer::take_writebacks(self)
    }
    fn finish(&mut self) -> Res<()> {
        Renderer::finish(self)
    }
    fn efb_snapshot(&mut self) -> Res<EfbSnapshot> {
        Renderer::efb_snapshot(self)
    }
    fn new_sharing(&self, display: RawDisplayHandle, window: RawWindowHandle, opts: Options) -> Res<Box<dyn Backend>> {
        let _ = display;
        Ok(Box::new(Renderer::create(self.gpu.clone(), Some(window), opts)?))
    }
}
