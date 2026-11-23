//! Direct3D 12 renderer: `vk-render`'s design on Direct3D 12.
//!
//! - The same WGSL ubershader, as HLSL (shader model 5.1). Draw states,
//!   frame interpolation's buffers and naga's sampler index buffer are root
//!   shader resource views; textures and samplers are descriptor tables
//!   into each frame's part of the shader-visible heaps.
//! - Pipeline states are keyed by blend state as in Vulkan, and also by cull
//!   mode and depth state, which Direct3D 12 bakes into them.
//! - Blits (XFB copies, texture patches, presenting) are a bilinear draw;
//!   overlay rectangles are clears.
//! - Logic ops: Direct3D has them on integer formats only (an RTX 5070 Ti
//!   reports none for RGBA8 UNORM, and a pipeline asking for one fails).
//!   Logic-op draws write the colour as integers (`fs_logic`) through a
//!   UINT view of the EFB, where the device has logic ops on RGBA8 UINT
//!   (`OutputMergerLogicOp` and `D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP`);
//!   otherwise `render::logic_as_blend` (exact for clear and no-op, copy for
//!   the rest).
//!
//! The debug layer is on in debug builds and with `NSMBW_VALIDATION` set,
//! when it is installed (Windows' optional "Graphics Tools").

use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;

use gx::{sh, Cmd, Output, Rect, Vertex, NO_TEXTURE, SHADING_WORDS};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use render::{as_bytes, at_scale, blend_key, rasterised_depth, Api, Backend, Blend, BlendFactor, CopyParams, EfbSnapshot, FrameStats, Options, OverlayRect, Res, UiFrame, Writeback, EARLY_DEPTH, EFB_H, EFB_W, MAX_XFBS};
use windows::core::{Interface, PCSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE, HWND, RECT};
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject, INFINITE};

use crate::common::{self, BlitParams, Shaders, UI_VERTEX_SIZE};

const FRAMES: usize = 2;
const MAX_TEXTURES: u32 = 4096;
const MAX_SAMPLERS: u32 = 256;
/// Per frame in the shader-visible heap: the texture table, then room for
/// the copy passes', blits' and menus' views.
const SCRATCH: u32 = 4096;
const FRAME_VIEWS: u32 = MAX_TEXTURES + SCRATCH;
const COLOR: DXGI_FORMAT = DXGI_FORMAT_R8G8B8A8_UNORM;
const SWAP_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM;
const VERTEX_SIZE: u32 = std::mem::size_of::<Vertex>() as u32;
const SWAP_BUFFERS: u32 = 3;

/// Pipeline key bits beyond `render::blend_key`'s (and `EARLY_DEPTH`): the
/// cull mode and the depth mode, baked into Direct3D 12 pipelines.
const CULL_SHIFT: u32 = 18;
const ZMODE_SHIFT: u32 = 20;

fn full_key(blend: u32, cull: u32, zmode: u32) -> u32 {
    blend | (cull & 3) << CULL_SHIFT | (zmode & 0x1F) << ZMODE_SHIFT
}

/// A raw pointer into mapped memory, moved between threads with its renderer.
#[derive(Clone, Copy)]
struct Mapped(*mut u8);
unsafe impl Send for Mapped {}
unsafe impl Sync for Mapped {}

struct Image {
    res: ID3D12Resource,
    w: u32,
    h: u32,
    state: D3D12_RESOURCE_STATES,
    /// Slots in the CPU heaps.
    srv: Option<u32>,
    rtv: Option<u32>,
    /// The EFB's UINT view, for logic-op draws.
    rtv_uint: Option<u32>,
    dsv: Option<u32>,
}

struct Buffer {
    res: ID3D12Resource,
    size: u64,
    /// Mapped for the buffer's life (upload and readback heaps).
    ptr: Mapped,
}

impl Buffer {
    fn bytes(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.0, self.size as usize) }
    }
    fn gpu(&self) -> u64 {
        unsafe { self.res.GetGPUVirtualAddress() }
    }
}

/// A descriptor heap whose slots are handed out one at a time.
struct Heap {
    heap: ID3D12DescriptorHeap,
    cpu: usize,
    gpu: u64,
    inc: u32,
    free: Vec<u32>,
    next: u32,
    cap: u32,
}

impl Heap {
    fn new(device: &ID3D12Device, kind: D3D12_DESCRIPTOR_HEAP_TYPE, cap: u32, visible: bool) -> Res<Self> {
        let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: kind, NumDescriptors: cap, Flags: if visible { D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE } else { D3D12_DESCRIPTOR_HEAP_FLAG_NONE }, NodeMask: 0 };
        unsafe {
            let heap: ID3D12DescriptorHeap = device.CreateDescriptorHeap(&desc)?;
            let cpu = heap.GetCPUDescriptorHandleForHeapStart().ptr;
            let gpu = if visible { heap.GetGPUDescriptorHandleForHeapStart().ptr } else { 0 };
            Ok(Self { heap, cpu, gpu, inc: device.GetDescriptorHandleIncrementSize(kind), free: Vec::new(), next: 0, cap })
        }
    }
    fn alloc(&mut self) -> Res<u32> {
        if let Some(i) = self.free.pop() {
            return Ok(i);
        }
        if self.next >= self.cap {
            return Err("out of descriptor slots".into());
        }
        self.next += 1;
        Ok(self.next - 1)
    }
    fn release(&mut self, i: u32) {
        self.free.push(i);
    }
    fn cpu(&self, i: u32) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: self.cpu + (i * self.inc) as usize }
    }
    fn gpu(&self, i: u32) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        D3D12_GPU_DESCRIPTOR_HANDLE { ptr: self.gpu + u64::from(i * self.inc) }
    }
}

/// A readback the frame owes: texels at `offset` of its buffer, rows
/// `pitch` bytes apart.
struct PendingWriteback {
    addr: u32,
    ram_hash: u64,
    tex_format: u32,
    w: u32,
    h: u32,
    stride: u32,
    offset: usize,
    pitch: usize,
}

struct Frame {
    alloc: ID3D12CommandAllocator,
    /// The renderer's fence value this frame's last submission signals.
    fence_value: u64,
    vertices: Option<Buffer>,
    shadings: Option<Buffer>,
    staging: Option<Buffer>,
    readback: Option<Buffer>,
    writebacks: Vec<PendingWriteback>,
    /// (prepare serial, texture epoch) the texture table and draw states
    /// were last written for by `render_prepared`.
    described: Option<(u64, u64)>,
    /// The frame's root SRV for the draw states.
    shadings_gpu: u64,
    retired_images: Vec<Image>,
    retired_buffers: Vec<Buffer>,
    retired_spare: Vec<Buffer>,
    /// Next free scratch view of the frame's part of the shader-visible heap.
    scratch: u32,
}

/// A field kept on the GPU for in-between frames (`Renderer::prepare`).
struct Prepared {
    key: (u64, u64),
    serial: u64,
    buffer: Buffer,
    index_offset: u64,
    moves: Option<[(u64, u64); 3]>,
}

/// The menus' textures and buffers.
struct UiLayer {
    textures: HashMap<egui::TextureId, Image>,
    /// Per frame: vertices then indices.
    buffers: Vec<Option<Buffer>>,
}

/// The startup pipelines' worker threads.
struct Prewarm {
    built: Receiver<(u32, ID3D12PipelineState)>,
    remaining: Arc<AtomicU32>,
    stop: Arc<AtomicBool>,
    workers: Vec<JoinHandle<()>>,
    total: u32,
    started: Instant,
}

/// What renderers on one GPU share: the device, its queue, the compiled
/// shaders and root signatures.
pub struct Gpu {
    factory: IDXGIFactory4,
    device: ID3D12Device,
    queue: ID3D12CommandQueue,
    info: Option<ID3D12InfoQueue>,
    device_name: String,
    validation: bool,
    /// Logic ops on the EFB's format.
    logic_ops: bool,
    shaders: Shaders,
    root: ID3D12RootSignature,
    copy_root: ID3D12RootSignature,
    /// The blits' and the menus': constants at b0, one texture at t0 and a
    /// bilinear clamping sampler.
    blit_root: ID3D12RootSignature,
    /// naga's sampler index buffer: 0..255 (each frame's samplers start its table).
    sampler_index: Buffer,
    /// Serialises reading the debug layer's messages.
    info_lock: Mutex<()>,
}

/// Prints the debug layer's messages so far.
fn drain_messages(gpu: &Gpu) {
    let Some(info) = &gpu.info else { return };
    let _lock = gpu.info_lock.lock().unwrap();
    unsafe {
        let n = info.GetNumStoredMessages();
        for i in 0..n {
            let mut len = 0usize;
            if info.GetMessage(i, None, &mut len).is_err() || len == 0 {
                continue;
            }
            let mut buf = vec![0u64; len.div_ceil(8)];
            let msg = buf.as_mut_ptr().cast::<D3D12_MESSAGE>();
            if info.GetMessage(i, Some(msg), &mut len).is_err() {
                continue;
            }
            let m = &*msg;
            let text = String::from_utf8_lossy(std::slice::from_raw_parts(m.pDescription, m.DescriptionByteLength.saturating_sub(1)));
            let error = m.Severity == D3D12_MESSAGE_SEVERITY_ERROR || m.Severity == D3D12_MESSAGE_SEVERITY_CORRUPTION;
            if error || m.Severity == D3D12_MESSAGE_SEVERITY_WARNING {
                render::note_validation("d3d12", error, i64::from(m.ID.0), &text);
            }
        }
        info.ClearStoredMessages();
    }
}

fn transition_barrier(res: &ID3D12Resource, from: D3D12_RESOURCE_STATES, to: D3D12_RESOURCE_STATES) -> D3D12_RESOURCE_BARRIER {
    D3D12_RESOURCE_BARRIER {
        Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
        Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
        Anonymous: D3D12_RESOURCE_BARRIER_0 {
            Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                // Borrowed, not AddRef'd: ManuallyDrop never releases it.
                pResource: unsafe { std::mem::transmute_copy(res) },
                Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                StateBefore: from,
                StateAfter: to,
            }),
        },
    }
}

/// Moves `img` to `state`.
fn transition(list: &ID3D12GraphicsCommandList, img: &mut Image, state: D3D12_RESOURCE_STATES) {
    if img.state == state {
        return;
    }
    unsafe { list.ResourceBarrier(&[transition_barrier(&img.res, img.state, state)]) };
    img.state = state;
}

fn borrowed(res: &ID3D12Resource) -> ManuallyDrop<Option<ID3D12Resource>> {
    unsafe { std::mem::transmute_copy(res) }
}

fn rect(x: u32, y: u32, w: u32, h: u32) -> RECT {
    RECT { left: x as i32, top: y as i32, right: (x + w) as i32, bottom: (y + h) as i32 }
}

fn viewport(x: f32, y: f32, w: f32, h: f32) -> D3D12_VIEWPORT {
    D3D12_VIEWPORT { TopLeftX: x, TopLeftY: y, Width: w, Height: h, MinDepth: 0.0, MaxDepth: 1.0 }
}

fn heap_props(kind: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
    D3D12_HEAP_PROPERTIES { Type: kind, ..Default::default() }
}

fn d3d_blend(f: BlendFactor) -> D3D12_BLEND {
    match f {
        BlendFactor::Zero => D3D12_BLEND_ZERO,
        BlendFactor::One => D3D12_BLEND_ONE,
        BlendFactor::SrcColor => D3D12_BLEND_SRC_COLOR,
        BlendFactor::OneMinusSrcColor => D3D12_BLEND_INV_SRC_COLOR,
        BlendFactor::DstColor => D3D12_BLEND_DEST_COLOR,
        BlendFactor::OneMinusDstColor => D3D12_BLEND_INV_DEST_COLOR,
        BlendFactor::Src1Alpha => D3D12_BLEND_SRC1_ALPHA,
        BlendFactor::OneMinusSrc1Alpha => D3D12_BLEND_INV_SRC1_ALPHA,
        BlendFactor::DstAlpha => D3D12_BLEND_DEST_ALPHA,
        BlendFactor::OneMinusDstAlpha => D3D12_BLEND_INV_DEST_ALPHA,
    }
}

/// The alpha channel's factor for a colour one (Direct3D wants alpha
/// factors for alpha).
fn alpha_factor(f: D3D12_BLEND) -> D3D12_BLEND {
    match f {
        D3D12_BLEND_SRC_COLOR => D3D12_BLEND_SRC_ALPHA,
        D3D12_BLEND_INV_SRC_COLOR => D3D12_BLEND_INV_SRC_ALPHA,
        D3D12_BLEND_DEST_COLOR => D3D12_BLEND_DEST_ALPHA,
        D3D12_BLEND_INV_DEST_COLOR => D3D12_BLEND_INV_DEST_ALPHA,
        f => f,
    }
}

/// The EFB pipeline's blend state for key `key`.
fn blend_desc(key: u32, logic_ops: bool) -> D3D12_RENDER_TARGET_BLEND_DESC {
    let (blend, mask) = render::blend_of(key);
    let mut write = 0u8;
    if mask & 1 != 0 {
        write |= (D3D12_COLOR_WRITE_ENABLE_RED.0 | D3D12_COLOR_WRITE_ENABLE_GREEN.0 | D3D12_COLOR_WRITE_ENABLE_BLUE.0) as u8;
    }
    if mask & 2 != 0 {
        write |= D3D12_COLOR_WRITE_ENABLE_ALPHA.0 as u8;
    }
    let mut d = D3D12_RENDER_TARGET_BLEND_DESC {
        BlendEnable: false.into(),
        LogicOpEnable: false.into(),
        SrcBlend: D3D12_BLEND_ONE,
        DestBlend: D3D12_BLEND_ZERO,
        BlendOp: D3D12_BLEND_OP_ADD,
        SrcBlendAlpha: D3D12_BLEND_ONE,
        DestBlendAlpha: D3D12_BLEND_ZERO,
        BlendOpAlpha: D3D12_BLEND_OP_ADD,
        LogicOp: D3D12_LOGIC_OP_NOOP,
        RenderTargetWriteMask: write,
    };
    let mut add = |src: D3D12_BLEND, dst: D3D12_BLEND, op: D3D12_BLEND_OP| {
        d.BlendEnable = true.into();
        (d.SrcBlend, d.DestBlend, d.BlendOp) = (src, dst, op);
        (d.SrcBlendAlpha, d.DestBlendAlpha, d.BlendOpAlpha) = (alpha_factor(src), alpha_factor(dst), op);
    };
    match blend {
        Blend::Off => {}
        Blend::Subtract => add(D3D12_BLEND_ONE, D3D12_BLEND_ONE, D3D12_BLEND_OP_REV_SUBTRACT),
        Blend::Add { src, dst } => add(d3d_blend(src), d3d_blend(dst), D3D12_BLEND_OP_ADD),
        Blend::Logic(op) if logic_ops => {
            // GX lists the logic ops as Vulkan does; Direct3D in another order.
            const D3D: [D3D12_LOGIC_OP; 16] = [
                D3D12_LOGIC_OP_CLEAR,
                D3D12_LOGIC_OP_AND,
                D3D12_LOGIC_OP_AND_REVERSE,
                D3D12_LOGIC_OP_COPY,
                D3D12_LOGIC_OP_AND_INVERTED,
                D3D12_LOGIC_OP_NOOP,
                D3D12_LOGIC_OP_XOR,
                D3D12_LOGIC_OP_OR,
                D3D12_LOGIC_OP_NOR,
                D3D12_LOGIC_OP_EQUIV,
                D3D12_LOGIC_OP_INVERT,
                D3D12_LOGIC_OP_OR_REVERSE,
                D3D12_LOGIC_OP_COPY_INVERTED,
                D3D12_LOGIC_OP_OR_INVERTED,
                D3D12_LOGIC_OP_NAND,
                D3D12_LOGIC_OP_SET,
            ];
            d.LogicOpEnable = true.into();
            d.LogicOp = D3D[op as usize & 15];
        }
        Blend::Logic(op) => {
            if let Some((src, dst)) = render::logic_as_blend(op) {
                add(d3d_blend(src), d3d_blend(dst), D3D12_BLEND_OP_ADD);
            }
        }
    }
    d
}

fn bytecode(code: &[u8]) -> D3D12_SHADER_BYTECODE {
    D3D12_SHADER_BYTECODE { pShaderBytecode: code.as_ptr().cast(), BytecodeLength: code.len() }
}

/// The ubershader's vertex inputs (naga names location n `LOCn`): the
/// vertex in slot 0 (`gx::Vertex`), the draw index per instance in slot 1.
fn uber_inputs() -> Vec<D3D12_INPUT_ELEMENT_DESC> {
    let mut v = Vec::new();
    let loc = PCSTR(c"LOC".as_ptr().cast());
    for i in 0..3 {
        v.push(D3D12_INPUT_ELEMENT_DESC { SemanticName: loc, SemanticIndex: i, Format: DXGI_FORMAT_R32G32B32A32_FLOAT, InputSlot: 0, AlignedByteOffset: 16 * i, InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, InstanceDataStepRate: 0 });
    }
    for i in 0..8 {
        v.push(D3D12_INPUT_ELEMENT_DESC { SemanticName: loc, SemanticIndex: 3 + i, Format: DXGI_FORMAT_R32G32B32_FLOAT, InputSlot: 0, AlignedByteOffset: 48 + 12 * i, InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, InstanceDataStepRate: 0 });
    }
    v.push(D3D12_INPUT_ELEMENT_DESC { SemanticName: loc, SemanticIndex: 11, Format: DXGI_FORMAT_R32_UINT, InputSlot: 1, AlignedByteOffset: 0, InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA, InstanceDataStepRate: 1 });
    v
}

fn rasterizer(cull: D3D12_CULL_MODE) -> D3D12_RASTERIZER_DESC {
    D3D12_RASTERIZER_DESC {
        FillMode: D3D12_FILL_MODE_SOLID,
        CullMode: cull,
        // Clockwise is front, as Vulkan is told.
        FrontCounterClockwise: false.into(),
        DepthClipEnable: true.into(),
        ..Default::default()
    }
}

/// The draw pipeline for full key `key`. Safe on any thread.
fn create_pipeline(gpu: &Gpu, key: u32) -> Res<ID3D12PipelineState> {
    let inputs = uber_inputs();
    let mut blend = D3D12_BLEND_DESC::default();
    blend.RenderTarget[0] = blend_desc(key, gpu.logic_ops);
    let z = (key >> ZMODE_SHIFT) & 0x1F;
    let cull = match (key >> CULL_SHIFT) & 3 {
        1 => D3D12_CULL_MODE_BACK,
        2 => D3D12_CULL_MODE_FRONT,
        _ => D3D12_CULL_MODE_NONE,
    };
    let depth = D3D12_DEPTH_STENCIL_DESC {
        DepthEnable: (z & 1 != 0).into(),
        DepthWriteMask: if z & 1 != 0 && z & 0x10 != 0 { D3D12_DEPTH_WRITE_MASK_ALL } else { D3D12_DEPTH_WRITE_MASK_ZERO },
        // GX lists the compare functions as Vulkan does, Direct3D from 1.
        DepthFunc: D3D12_COMPARISON_FUNC(((z >> 1) & 7) as i32 + 1),
        ..Default::default()
    };
    let uint = uses_uint(key, gpu.logic_ops);
    let fs = if uint {
        &gpu.shaders.uber.fs_logic
    } else if key & EARLY_DEPTH != 0 {
        &gpu.shaders.uber.fs_early
    } else {
        &gpu.shaders.uber.fs
    };
    let mut desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
        pRootSignature: borrowed_root(&gpu.root),
        VS: bytecode(&gpu.shaders.uber.vs),
        PS: bytecode(fs),
        BlendState: blend,
        SampleMask: u32::MAX,
        RasterizerState: rasterizer(cull),
        DepthStencilState: depth,
        InputLayout: D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: inputs.as_ptr(), NumElements: inputs.len() as u32 },
        PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
        NumRenderTargets: 1,
        DSVFormat: DXGI_FORMAT_D32_FLOAT,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        ..Default::default()
    };
    desc.RTVFormats[0] = if uint { DXGI_FORMAT_R8G8B8A8_UINT } else { COLOR };
    Ok(unsafe { gpu.device.CreateGraphicsPipelineState(&desc)? })
}

/// Whether pipeline key `key` draws through the EFB's UINT view: a logic
/// op where the device has them (on that format).
fn uses_uint(key: u32, logic_ops: bool) -> bool {
    logic_ops && matches!(render::blend_of(key).0, Blend::Logic(_))
}

fn borrowed_root(r: &ID3D12RootSignature) -> ManuallyDrop<Option<ID3D12RootSignature>> {
    unsafe { std::mem::transmute_copy(r) }
}

/// A pipeline drawing a full-screen triangle (no vertex input) into one
/// `format` target: the copy pass, or a blit (`blend`: the menus' blending).
fn simple_pipeline(gpu: &Gpu, root: &ID3D12RootSignature, vs: &[u8], ps: &[u8], format: DXGI_FORMAT, inputs: &[D3D12_INPUT_ELEMENT_DESC], ui: bool) -> Res<ID3D12PipelineState> {
    let mut blend = D3D12_BLEND_DESC::default();
    blend.RenderTarget[0] = D3D12_RENDER_TARGET_BLEND_DESC {
        BlendEnable: ui.into(),
        LogicOpEnable: false.into(),
        SrcBlend: D3D12_BLEND_ONE,
        DestBlend: D3D12_BLEND_INV_SRC_ALPHA,
        BlendOp: D3D12_BLEND_OP_ADD,
        SrcBlendAlpha: D3D12_BLEND_INV_DEST_ALPHA,
        DestBlendAlpha: D3D12_BLEND_ONE,
        BlendOpAlpha: D3D12_BLEND_OP_ADD,
        LogicOp: D3D12_LOGIC_OP_NOOP,
        RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
    };
    let mut desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
        pRootSignature: borrowed_root(root),
        VS: bytecode(vs),
        PS: bytecode(ps),
        BlendState: blend,
        SampleMask: u32::MAX,
        RasterizerState: rasterizer(D3D12_CULL_MODE_NONE),
        InputLayout: D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: inputs.as_ptr(), NumElements: inputs.len() as u32 },
        PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
        NumRenderTargets: 1,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        ..Default::default()
    };
    desc.RTVFormats[0] = format;
    Ok(unsafe { gpu.device.CreateGraphicsPipelineState(&desc)? })
}

fn ui_inputs() -> [D3D12_INPUT_ELEMENT_DESC; 3] {
    let e = |name: &'static std::ffi::CStr, format, offset| D3D12_INPUT_ELEMENT_DESC { SemanticName: PCSTR(name.as_ptr().cast()), SemanticIndex: 0, Format: format, InputSlot: 0, AlignedByteOffset: offset, InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, InstanceDataStepRate: 0 };
    [e(c"POSITION", DXGI_FORMAT_R32G32_FLOAT, 0), e(c"TEXCOORD", DXGI_FORMAT_R32G32_FLOAT, 8), e(c"COLOR", DXGI_FORMAT_R8G8B8A8_UNORM, 16)]
}

fn serialize_root(params: &[D3D12_ROOT_PARAMETER1], samplers: &[D3D12_STATIC_SAMPLER_DESC], device: &ID3D12Device) -> Res<ID3D12RootSignature> {
    let desc = D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
        Version: D3D_ROOT_SIGNATURE_VERSION_1_1,
        Anonymous: D3D12_VERSIONED_ROOT_SIGNATURE_DESC_0 {
            Desc_1_1: D3D12_ROOT_SIGNATURE_DESC1 {
                NumParameters: params.len() as u32,
                pParameters: params.as_ptr(),
                NumStaticSamplers: samplers.len() as u32,
                pStaticSamplers: samplers.as_ptr(),
                Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
            },
        },
    };
    let mut blob = None;
    let mut errors = None;
    unsafe {
        if let Err(e) = D3D12SerializeVersionedRootSignature(&desc, &mut blob, Some(&mut errors)) {
            let text = errors.as_ref().map(|b| String::from_utf8_lossy(common::blob(b)).into_owned()).unwrap_or_default();
            return Err(format!("root signature: {e} {text}").into());
        }
        let blob = blob.ok_or("root signature: no blob")?;
        Ok(device.CreateRootSignature(0, common::blob(&blob))?)
    }
}

fn root_srv(register: u32, visibility: D3D12_SHADER_VISIBILITY) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: D3D12_ROOT_PARAMETER_TYPE_SRV,
        Anonymous: D3D12_ROOT_PARAMETER1_0 { Descriptor: D3D12_ROOT_DESCRIPTOR1 { ShaderRegister: register, RegisterSpace: 0, Flags: D3D12_ROOT_DESCRIPTOR_FLAG_DATA_VOLATILE } },
        ShaderVisibility: visibility,
    }
}

fn root_constants(count: u32, visibility: D3D12_SHADER_VISIBILITY) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
        Anonymous: D3D12_ROOT_PARAMETER1_0 { Constants: D3D12_ROOT_CONSTANTS { ShaderRegister: 0, RegisterSpace: 0, Num32BitValues: count } },
        ShaderVisibility: visibility,
    }
}

fn table(range: &D3D12_DESCRIPTOR_RANGE1) -> D3D12_ROOT_PARAMETER1 {
    D3D12_ROOT_PARAMETER1 {
        ParameterType: D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
        Anonymous: D3D12_ROOT_PARAMETER1_0 { DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE1 { NumDescriptorRanges: 1, pDescriptorRanges: range } },
        ShaderVisibility: D3D12_SHADER_VISIBILITY_PIXEL,
    }
}

fn range(kind: D3D12_DESCRIPTOR_RANGE_TYPE, count: u32, register: u32, space: u32) -> D3D12_DESCRIPTOR_RANGE1 {
    D3D12_DESCRIPTOR_RANGE1 {
        RangeType: kind,
        NumDescriptors: count,
        BaseShaderRegister: register,
        RegisterSpace: space,
        // Not every slot is written, nor read.
        Flags: D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_VOLATILE,
        OffsetInDescriptorsFromTableStart: 0,
    }
}

/// Root parameters of the ubershader's root signature.
const ROOT_INTERP: u32 = 0;
const ROOT_SHADINGS: u32 = 1;
const ROOT_MODEL: u32 = 2;
const ROOT_MATS: u32 = 3;
const ROOT_RECS: u32 = 4;
const ROOT_SAMPLER_INDEX: u32 = 5;
const ROOT_TEXTURES: u32 = 6;
const ROOT_SAMPLERS: u32 = 7;

impl Gpu {
    fn create(validation_wanted: bool, pipeline_cache: Option<&std::path::Path>) -> Res<Arc<Gpu>> {
        unsafe {
            let mut validation = false;
            if validation_wanted {
                let mut debug: Option<ID3D12Debug> = None;
                if D3D12GetDebugInterface(&mut debug).is_ok() {
                    if let Some(d) = debug {
                        d.EnableDebugLayer();
                        validation = true;
                    }
                }
            }
            let factory: IDXGIFactory4 = CreateDXGIFactory2(if validation { DXGI_CREATE_FACTORY_DEBUG } else { DXGI_CREATE_FACTORY_FLAGS(0) })?;
            // The first hardware adapter that makes a feature level 11.0
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
                let mut device: Option<ID3D12Device> = None;
                if D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device).is_ok() {
                    let name = String::from_utf16_lossy(&desc.Description).trim_end_matches('\0').to_string();
                    chosen = device.map(|d| (d, name));
                    break;
                }
            }
            let (device, device_name) = chosen.ok_or("no Direct3D 12 device")?;
            let mut options = D3D12_FEATURE_DATA_D3D12_OPTIONS::default();
            device.CheckFeatureSupport(D3D12_FEATURE_D3D12_OPTIONS, (&mut options as *mut D3D12_FEATURE_DATA_D3D12_OPTIONS).cast(), std::mem::size_of_val(&options) as u32)?;
            // 4096 textures and 256 samplers in tables need binding tier 2.
            if options.ResourceBindingTier.0 < D3D12_RESOURCE_BINDING_TIER_2.0 {
                return Err("the Direct3D 12 device has resource binding tier 1 (4096 textures a draw need tier 2)".into());
            }
            // Logic ops work on integer formats: the EFB's UINT view.
            let mut fs = D3D12_FEATURE_DATA_FORMAT_SUPPORT { Format: DXGI_FORMAT_R8G8B8A8_UINT, ..Default::default() };
            let format_logic = device.CheckFeatureSupport(D3D12_FEATURE_FORMAT_SUPPORT, (&mut fs as *mut D3D12_FEATURE_DATA_FORMAT_SUPPORT).cast(), std::mem::size_of_val(&fs) as u32).is_ok() && (fs.Support2.0 & D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP.0) != 0;
            let logic_ops = options.OutputMergerLogicOp.as_bool() && format_logic;
            let info: Option<ID3D12InfoQueue> = if validation { device.cast().ok() } else { None };
            if let Some(info) = &info {
                let _ = info.SetMessageCountLimit(4096);
            }
            let queue: ID3D12CommandQueue = device.CreateCommandQueue(&D3D12_COMMAND_QUEUE_DESC { Type: D3D12_COMMAND_LIST_TYPE_DIRECT, ..Default::default() })?;

            let shaders = common::compile_all(Api::D3d12, common::shader_dir(pipeline_cache).as_deref())?;
            let vs = D3D12_SHADER_VISIBILITY_VERTEX;
            let ps = D3D12_SHADER_VISIBILITY_PIXEL;
            // Unbounded tables: naga declares 2048 samplers (the heap's size).
            let textures = range(D3D12_DESCRIPTOR_RANGE_TYPE_SRV, MAX_TEXTURES, 0, 1);
            let samplers = range(D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER, u32::MAX, 0, 0);
            let params = [root_constants(2, vs), root_srv(0, ps), root_srv(1, vs), root_srv(2, vs), root_srv(3, vs), root_srv(4, ps), table(&textures), table(&samplers)];
            let root = serialize_root(&params, &[], &device)?;
            let efb = range(D3D12_DESCRIPTOR_RANGE_TYPE_SRV, 2, 0, 0);
            let copy_root = serialize_root(&[root_constants((std::mem::size_of::<CopyParams>() / 4) as u32, ps), table(&efb)], &[], &device)?;
            let one = range(D3D12_DESCRIPTOR_RANGE_TYPE_SRV, 1, 0, 0);
            let bilinear = D3D12_STATIC_SAMPLER_DESC {
                Filter: D3D12_FILTER_MIN_MAG_MIP_LINEAR,
                AddressU: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
                AddressV: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
                AddressW: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
                MaxAnisotropy: 1,
                ComparisonFunc: D3D12_COMPARISON_FUNC_NEVER,
                MaxLOD: 0.0,
                ShaderVisibility: ps,
                ..Default::default()
            };
            let blit_root = serialize_root(&[root_constants(8, D3D12_SHADER_VISIBILITY_ALL), table(&one)], &[bilinear], &device)?;

            // naga's sampler index buffer: 0..255.
            let mut sampler_index = create_buffer(&device, 1024, D3D12_HEAP_TYPE_UPLOAD)?;
            let words: Vec<u32> = (0..256).collect();
            sampler_index.bytes().copy_from_slice(as_bytes(&words));
            let gpu = Gpu { factory, device, queue, info, device_name, validation, logic_ops, shaders, root, copy_root, blit_root, sampler_index, info_lock: Mutex::new(()) };
            Ok(Arc::new(gpu))
        }
    }
}

/// A mapped buffer in an upload or readback heap.
fn create_buffer(device: &ID3D12Device, size: u64, kind: D3D12_HEAP_TYPE) -> Res<Buffer> {
    let desc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
        Width: size,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        ..Default::default()
    };
    let state = if kind == D3D12_HEAP_TYPE_READBACK { D3D12_RESOURCE_STATE_COPY_DEST } else { D3D12_RESOURCE_STATE_GENERIC_READ };
    unsafe {
        let mut res: Option<ID3D12Resource> = None;
        device.CreateCommittedResource(&heap_props(kind), D3D12_HEAP_FLAG_NONE, &desc, state, None, &mut res)?;
        let res = res.ok_or("no buffer")?;
        let mut ptr = std::ptr::null_mut::<c_void>();
        // Readback buffers are read whole; upload buffers never.
        let none = D3D12_RANGE { Begin: 0, End: 0 };
        res.Map(0, if kind == D3D12_HEAP_TYPE_READBACK { None } else { Some(&none) }, Some(&mut ptr))?;
        Ok(Buffer { res, size, ptr: Mapped(ptr.cast()) })
    }
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
    device: ID3D12Device,
    hwnd: Option<HWND>,
    swapchain: Option<IDXGISwapChain3>,
    swap_images: Vec<Image>,
    swap_size: (u32, u32),
    needs_rebuild: bool,
    fence: ID3D12Fence,
    fence_value: u64,
    event: HANDLE,
    list: ID3D12GraphicsCommandList,
    frames: Vec<Frame>,
    frame: usize,
    /// CPU heaps for views, render targets, depth and samplers; the
    /// shader-visible views (FRAMES x FRAME_VIEWS) and samplers.
    srv_heap: Heap,
    rtv_heap: Heap,
    dsv_heap: Heap,
    sampler_heap: Heap,
    views: Heap,
    sampler_table: Heap,
    pipelines: HashMap<u32, ID3D12PipelineState>,
    prewarm: Option<Prewarm>,
    keys_path: Option<PathBuf>,
    copy_pso: ID3D12PipelineState,
    /// Blits into the EFB's format and into the swapchain's; the menus.
    blit_pso: ID3D12PipelineState,
    blit_swap_pso: ID3D12PipelineState,
    ui_pso: ID3D12PipelineState,
    efb_color: Image,
    efb_depth: Image,
    native: Image,
    efb_ready: bool,
    index_offset: u64,
    push: [u32; 2],
    /// Draw index per instance: 0, 1, 2...
    instance_ids: Option<Buffer>,
    prepared: Option<Prepared>,
    spare: Vec<Buffer>,
    trash_spare: Vec<Buffer>,
    textures: HashMap<u32, Image>,
    textures_epoch: u64,
    /// Sampler slots in the CPU sampler heap by GX key.
    samplers: HashMap<u32, u32>,
    xfbs: HashMap<u32, (Image, u64)>,
    xfb_serial: u64,
    writebacks: Vec<Writeback>,
    trash_images: Vec<Image>,
    trash_buffers: Vec<Buffer>,
    ui: UiLayer,
    ui_frame: Option<UiFrame>,
    aspect: f32,
    screenshot: Option<PathBuf>,
    overlay: Vec<OverlayRect>,
    stats: FrameStats,
}

// COM pointers are free-threaded in Direct3D 12; the event handle is ours.
unsafe impl Send for Renderer {}

impl Renderer {
    pub fn new(display: RawDisplayHandle, window: RawWindowHandle, opts: Options) -> Res<Self> {
        let _ = display;
        let gpu = Gpu::create(validation_wanted(), opts.pipeline_cache.as_deref())?;
        Self::create(gpu, Some(window), opts)
    }

    pub fn headless(opts: Options) -> Res<Self> {
        let gpu = Gpu::create(validation_wanted(), opts.pipeline_cache.as_deref())?;
        Self::create(gpu, None, opts)
    }

    fn create(gpu: Arc<Gpu>, window: Option<RawWindowHandle>, opts: Options) -> Res<Self> {
        let hwnd = match window {
            Some(RawWindowHandle::Win32(h)) => Some(HWND(h.hwnd.get() as *mut c_void)),
            Some(_) => return Err("Direct3D 12 draws to Win32 windows only".into()),
            None => None,
        };
        let scale = opts.scale.max(0.25);
        let device = gpu.device.clone();
        unsafe {
            let fence: ID3D12Fence = device.CreateFence(0, D3D12_FENCE_FLAG_NONE)?;
            let event = CreateEventW(None, false, false, None)?;
            let mut frames = Vec::new();
            for _ in 0..FRAMES {
                frames.push(Frame {
                    alloc: device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?,
                    fence_value: 0,
                    vertices: None,
                    shadings: None,
                    staging: None,
                    readback: None,
                    writebacks: Vec::new(),
                    described: None,
                    shadings_gpu: 0,
                    retired_images: Vec::new(),
                    retired_buffers: Vec::new(),
                    retired_spare: Vec::new(),
                    scratch: 0,
                });
            }
            let list: ID3D12GraphicsCommandList = device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &frames[0].alloc, None)?;
            list.Close()?;
            let mut srv_heap = Heap::new(&device, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, 65536, false)?;
            let mut rtv_heap = Heap::new(&device, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, 16384, false)?;
            let mut dsv_heap = Heap::new(&device, D3D12_DESCRIPTOR_HEAP_TYPE_DSV, 4, false)?;
            let sampler_heap = Heap::new(&device, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, 4096, false)?;
            let views = Heap::new(&device, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, FRAME_VIEWS * FRAMES as u32, true)?;
            let sampler_table = Heap::new(&device, D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, 2048, true)?;

            let (ew, eh) = (at_scale(EFB_W, scale), at_scale(EFB_H, scale));
            let (efb_color, efb_depth) = Self::create_efb(&device, &mut srv_heap, &mut rtv_heap, &mut dsv_heap, ew, eh)?;
            let native = create_image(&device, &mut srv_heap, &mut rtv_heap, EFB_W, EFB_H, 1, COLOR, true)?;

            let copy_pso = simple_pipeline(&gpu, &gpu.copy_root, &gpu.shaders.copy_vs, &gpu.shaders.copy_fs, COLOR, &[], false)?;
            let blit_pso = simple_pipeline(&gpu, &gpu.blit_root, &gpu.shaders.blit_vs, &gpu.shaders.blit_ps, COLOR, &[], false)?;
            let blit_swap_pso = simple_pipeline(&gpu, &gpu.blit_root, &gpu.shaders.blit_vs, &gpu.shaders.blit_ps, SWAP_FORMAT, &[], false)?;
            let ui_pso = simple_pipeline(&gpu, &gpu.blit_root, &gpu.shaders.ui_vs, &gpu.shaders.ui_ps, SWAP_FORMAT, &ui_inputs(), true)?;
            let keys_path = opts.pipeline_cache.as_ref().map(|p| p.with_file_name("pipelines-d3d12.bin"));

            let mut r = Self {
                device_name: gpu.device_name.clone(),
                validation: gpu.validation,
                scale,
                stretch: 1.0,
                efb_stale: false,
                vsync: opts.vsync,
                copies_to_ram: opts.efb_copies_to_ram,
                gpu,
                device,
                hwnd,
                swapchain: None,
                swap_images: Vec::new(),
                swap_size: (0, 0),
                needs_rebuild: true,
                fence,
                fence_value: 0,
                event,
                list,
                frames,
                frame: 0,
                srv_heap,
                rtv_heap,
                dsv_heap,
                sampler_heap,
                views,
                sampler_table,
                pipelines: HashMap::new(),
                prewarm: None,
                keys_path,
                copy_pso,
                blit_pso,
                blit_swap_pso,
                ui_pso,
                efb_color,
                efb_depth,
                native,
                efb_ready: false,
                index_offset: 0,
                push: [0; 2],
                instance_ids: None,
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
                ui: UiLayer { textures: HashMap::new(), buffers: (0..FRAMES).map(|_| None).collect() },
                ui_frame: None,
                aspect: 16.0 / 9.0,
                screenshot: None,
                overlay: Vec::new(),
                stats: FrameStats::default(),
            };
            if r.keys_path.is_some() {
                r.start_prewarm();
            }
            Ok(r)
        }
    }

    /// The EFB's colour and depth images at `w` x `h`.
    fn create_efb(device: &ID3D12Device, srv: &mut Heap, rtv: &mut Heap, dsv: &mut Heap, w: u32, h: u32) -> Res<(Image, Image)> {
        let color = create_efb_color(device, srv, rtv, w, h)?;
        let desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: u64::from(w),
            Height: h,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_R32_TYPELESS,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Flags: D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL,
            ..Default::default()
        };
        let clear = D3D12_CLEAR_VALUE { Format: DXGI_FORMAT_D32_FLOAT, Anonymous: D3D12_CLEAR_VALUE_0 { DepthStencil: D3D12_DEPTH_STENCIL_VALUE { Depth: 1.0, Stencil: 0 } } };
        unsafe {
            let mut res: Option<ID3D12Resource> = None;
            device.CreateCommittedResource(&heap_props(D3D12_HEAP_TYPE_DEFAULT), D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_DEPTH_WRITE, Some(&clear), &mut res)?;
            let res = res.ok_or("no depth image")?;
            let s = srv.alloc()?;
            let view = D3D12_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_R32_FLOAT,
                ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2D,
                Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 { Texture2D: D3D12_TEX2D_SRV { MipLevels: 1, ..Default::default() } },
            };
            device.CreateShaderResourceView(&res, Some(&view), srv.cpu(s));
            let d = dsv.alloc()?;
            let dview = D3D12_DEPTH_STENCIL_VIEW_DESC { Format: DXGI_FORMAT_D32_FLOAT, ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2D, ..Default::default() };
            device.CreateDepthStencilView(&res, Some(&dview), dsv.cpu(d));
            let depth = Image { res, w, h, state: D3D12_RESOURCE_STATE_DEPTH_WRITE, srv: Some(s), rtv: None, rtv_uint: None, dsv: Some(d) };
            Ok((color, depth))
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

    /// Waits for everything this renderer submitted.
    fn wait_idle(&mut self) -> Res<()> {
        self.fence_value += 1;
        unsafe { self.gpu.queue.Signal(&self.fence, self.fence_value)? };
        self.wait_fence(self.fence_value)
    }

    fn wait_fence(&self, value: u64) -> Res<()> {
        unsafe {
            if self.fence.GetCompletedValue() < value {
                self.fence.SetEventOnCompletion(value, self.event)?;
                WaitForSingleObject(self.event, INFINITE);
            }
        }
        Ok(())
    }

    fn remake_efb(&mut self) -> Res<()> {
        self.wait_idle()?;
        let (ew, eh) = (at_scale(EFB_W, self.sx()), at_scale(EFB_H, self.sy()));
        let (color, depth) = Self::create_efb(&self.device, &mut self.srv_heap, &mut self.rtv_heap, &mut self.dsv_heap, ew, eh)?;
        let old = [std::mem::replace(&mut self.efb_color, color), std::mem::replace(&mut self.efb_depth, depth)];
        for img in old {
            self.destroy_image(img);
        }
        self.efb_ready = false;
        self.efb_stale = false;
        Ok(())
    }

    fn destroy_image(&mut self, img: Image) {
        if let Some(s) = img.srv {
            self.srv_heap.release(s);
        }
        if let Some(s) = img.rtv {
            self.rtv_heap.release(s);
        }
        if let Some(s) = img.rtv_uint {
            self.rtv_heap.release(s);
        }
        if let Some(s) = img.dsv {
            self.dsv_heap.release(s);
        }
    }

    pub fn set_vsync(&mut self, on: bool) {
        self.vsync = on;
    }

    pub fn resized(&mut self) {
        self.needs_rebuild = true;
    }

    fn slot(&mut self, f: usize, which: u8) -> &mut Option<Buffer> {
        match which {
            0 => &mut self.frames[f].vertices,
            1 => &mut self.frames[f].shadings,
            2 => &mut self.frames[f].staging,
            _ => &mut self.frames[f].readback,
        }
    }

    /// A mapped buffer of at least `size` bytes in `slot` of frame `f` (0
    /// vertices, 1 draw states, 2 upload staging, 3 readback).
    fn frame_buffer(&mut self, f: usize, which: u8, size: u64) -> Res<()> {
        if self.slot(f, which).as_ref().is_some_and(|b| b.size >= size) {
            return Ok(());
        }
        if let Some(old) = self.slot(f, which).take() {
            self.trash_buffers.push(old);
        }
        let kind = if which == 3 { D3D12_HEAP_TYPE_READBACK } else { D3D12_HEAP_TYPE_UPLOAD };
        let b = create_buffer(&self.device, size.next_power_of_two().max(4096), kind)?;
        *self.slot(f, which) = Some(b);
        Ok(())
    }

    /// The CPU sampler slot for GX sampler key `key`.
    fn sampler(&mut self, key: u32) -> Res<u32> {
        if let Some(&s) = self.samplers.get(&key) {
            return Ok(s);
        }
        let d = render::sampler_desc(key);
        let wrap = |w: u32| match w {
            0 => D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            1 => D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            _ => D3D12_TEXTURE_ADDRESS_MODE_MIRROR,
        };
        // D3D12_ENCODE_BASIC_FILTER: min << 4 | mag << 2 | mip (1: linear).
        let filter = u32::from(d.min_linear) << 4 | u32::from(d.mag_linear) << 2 | u32::from(d.mip_linear);
        let desc = D3D12_SAMPLER_DESC {
            Filter: D3D12_FILTER(filter as i32),
            AddressU: wrap(d.wrap_u),
            AddressV: wrap(d.wrap_v),
            AddressW: D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
            MipLODBias: d.lod_bias,
            MaxAnisotropy: 1,
            ComparisonFunc: D3D12_COMPARISON_FUNC_NEVER,
            BorderColor: [0.0; 4],
            MinLOD: 0.0,
            MaxLOD: d.max_lod.unwrap_or(D3D12_FLOAT32_MAX),
        };
        let s = self.sampler_heap.alloc()?;
        unsafe { self.device.CreateSampler(&desc, self.sampler_heap.cpu(s)) };
        self.samplers.insert(key, s);
        Ok(s)
    }

    fn pipeline(&mut self, key: u32) -> Res<ID3D12PipelineState> {
        if let Some(p) = self.pipelines.get(&key) {
            return Ok(p.clone());
        }
        self.collect_prewarmed();
        if let Some(p) = self.pipelines.get(&key) {
            return Ok(p.clone());
        }
        let p = create_pipeline(&self.gpu, key)?;
        self.pipelines.insert(key, p.clone());
        Ok(p)
    }

    fn start_prewarm(&mut self) {
        let mut keys = render::startup_keys(self.keys_path.as_deref());
        // The Vulkan startup keys are blend states only: with no cull, and
        // no depth or a LEQUAL test with and without writes.
        keys.retain(|k| k >> CULL_SHIFT != 0);
        for &k in render::STARTUP_KEYS {
            for z in [0, 0x07, 0x17] {
                keys.push(full_key(k, 0, z));
            }
        }
        keys.sort_unstable();
        keys.dedup();
        let total = keys.len() as u32;
        let queue = Arc::new(Mutex::new(keys));
        let remaining = Arc::new(AtomicU32::new(total));
        let stop = Arc::new(AtomicBool::new(false));
        let (tx, built) = std::sync::mpsc::channel();
        let threads = std::thread::available_parallelism().map_or(2, |n| n.get()).saturating_sub(2).clamp(1, 4);
        let workers = (0..threads)
            .filter_map(|i| {
                let (gpu, queue, remaining, stop, tx) = (self.gpu.clone(), queue.clone(), remaining.clone(), stop.clone(), tx.clone());
                std::thread::Builder::new()
                    .name(format!("pipelines-{i}"))
                    .spawn(move || {
                        while !stop.load(Ordering::Relaxed) {
                            let Some(key) = queue.lock().unwrap().pop() else { break };
                            if let Ok(p) = create_pipeline(&gpu, key) {
                                let _ = tx.send((key, p));
                            }
                            remaining.fetch_sub(1, Ordering::Relaxed);
                        }
                    })
                    .ok()
            })
            .collect();
        self.prewarm = Some(Prewarm { built, remaining, stop, workers, total, started: Instant::now() });
    }

    fn collect_prewarmed(&mut self) {
        let Some(w) = &self.prewarm else { return };
        let done: Vec<(u32, ID3D12PipelineState)> = w.built.try_iter().collect();
        for (key, p) in done {
            self.pipelines.entry(key).or_insert(p);
        }
    }

    pub fn pipelines_compiling(&mut self) -> u32 {
        self.collect_prewarmed();
        let Some(w) = &self.prewarm else { return 0 };
        let left = w.remaining.load(Ordering::Relaxed);
        if left == 0 && !w.workers.is_empty() {
            let w = self.prewarm.as_mut().unwrap();
            for t in w.workers.drain(..) {
                let _ = t.join();
            }
            println!("shaders: {} pipelines built at startup in {:.2} s", w.total, w.started.elapsed().as_secs_f32());
            self.collect_prewarmed();
        }
        left
    }

    pub fn set_ui(&mut self, frame: Option<UiFrame>, textures: &egui::TexturesDelta) -> Res<()> {
        if !textures.set.is_empty() {
            self.one_shot(|r, list| {
                for (id, deltas) in &textures.set {
                    for delta in deltas {
                        r.ui_texture(list, *id, delta)?;
                    }
                }
                Ok(())
            })?;
        }
        if !textures.free.is_empty() {
            self.wait_idle()?;
            for id in &textures.free {
                if let Some(img) = self.ui.textures.remove(id) {
                    self.destroy_image(img);
                }
            }
        }
        self.ui_frame = frame;
        Ok(())
    }

    /// Uploads an egui texture (whole or a part) in `list`; its staging
    /// buffer is retired with the frame.
    fn ui_texture(&mut self, list: &ID3D12GraphicsCommandList, id: egui::TextureId, delta: &egui::epaint::ImageDelta) -> Res<()> {
        let (pixels, w, h, pos) = common::ui_pixels(delta);
        let (w, h) = (w as u32, h as u32);
        if pos.is_none() {
            let img = create_image(&self.device, &mut self.srv_heap, &mut self.rtv_heap, w, h, 1, DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, false)?;
            if let Some(old) = self.ui.textures.insert(id, img) {
                self.trash_images.push(old);
            }
        }
        let [x, y] = pos.unwrap_or([0, 0]);
        let Some(img) = self.ui.textures.get_mut(&id) else { return Ok(()) };
        let pitch = (w * 4).next_multiple_of(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT);
        let mut staging = create_buffer(&self.device, u64::from(pitch * h), D3D12_HEAP_TYPE_UPLOAD)?;
        let bytes = staging.bytes();
        for row in 0..h as usize {
            bytes[row * pitch as usize..][..w as usize * 4].copy_from_slice(&pixels[row * w as usize * 4..][..w as usize * 4]);
        }
        transition(list, img, D3D12_RESOURCE_STATE_COPY_DEST);
        let footprint = D3D12_PLACED_SUBRESOURCE_FOOTPRINT { Offset: 0, Footprint: D3D12_SUBRESOURCE_FOOTPRINT { Format: DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, Width: w, Height: h, Depth: 1, RowPitch: pitch } };
        copy_buffer_to_image(list, &staging.res, footprint, &img.res, 0, x as u32, y as u32);
        transition(list, img, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
        self.trash_buffers.push(staging);
        Ok(())
    }

    /// Records into a temporary allocator and list, submits and waits.
    fn one_shot(&mut self, record: impl FnOnce(&mut Self, &ID3D12GraphicsCommandList) -> Res<()>) -> Res<()> {
        unsafe {
            let alloc: ID3D12CommandAllocator = self.device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?;
            let list: ID3D12GraphicsCommandList = self.device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &alloc, None)?;
            let recorded = record(self, &list);
            list.Close()?;
            recorded?;
            self.gpu.queue.ExecuteCommandLists(&[Some(list.cast()?)]);
            self.wait_idle()?;
            drain_messages(&self.gpu);
        }
        // Whatever was retired meanwhile is done with.
        for img in std::mem::take(&mut self.trash_images) {
            self.destroy_image(img);
        }
        self.trash_buffers.clear();
        Ok(())
    }

    fn has_window(&self) -> bool {
        self.hwnd.is_some()
    }

    fn rebuild(&mut self, width: u32, height: u32) -> Res<()> {
        let Some(hwnd) = self.hwnd else { return Ok(()) };
        self.wait_idle()?;
        for img in std::mem::take(&mut self.swap_images) {
            self.destroy_image(img);
        }
        unsafe {
            match &self.swapchain {
                Some(s) => s.ResizeBuffers(SWAP_BUFFERS, width, height, SWAP_FORMAT, DXGI_SWAP_CHAIN_FLAG(0))?,
                None => {
                    let desc = DXGI_SWAP_CHAIN_DESC1 {
                        Width: width,
                        Height: height,
                        Format: SWAP_FORMAT,
                        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                        BufferCount: SWAP_BUFFERS,
                        Scaling: DXGI_SCALING_NONE,
                        SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                        AlphaMode: DXGI_ALPHA_MODE_IGNORE,
                        Flags: 0,
                        ..Default::default()
                    };
                    let s: IDXGISwapChain1 = self.gpu.factory.CreateSwapChainForHwnd(&self.gpu.queue, hwnd, &desc, None, None)?;
                    // The frontend handles Alt+Enter (borderless fullscreen).
                    let _ = self.gpu.factory.MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER);
                    self.swapchain = Some(s.cast()?);
                }
            }
            let s = self.swapchain.as_ref().unwrap();
            let desc = s.GetDesc1()?;
            self.swap_size = (desc.Width, desc.Height);
            for i in 0..SWAP_BUFFERS {
                let res: ID3D12Resource = s.GetBuffer(i)?;
                let slot = self.rtv_heap.alloc()?;
                self.device.CreateRenderTargetView(&res, None, self.rtv_heap.cpu(slot));
                self.swap_images.push(Image { res, w: desc.Width, h: desc.Height, state: D3D12_RESOURCE_STATE_PRESENT, srv: None, rtv: Some(slot), rtv_uint: None, dsv: None });
            }
        }
        self.needs_rebuild = false;
        Ok(())
    }

    /// Waits for frame `f`'s last submission and frees what it held.
    fn begin_frame(&mut self, f: usize) -> Res<()> {
        self.wait_fence(self.frames[f].fence_value)?;
        drain_messages(&self.gpu);
        if self.efb_stale {
            self.remake_efb()?;
        }
        self.collect_writebacks(f);
        for img in std::mem::take(&mut self.frames[f].retired_images) {
            self.destroy_image(img);
        }
        self.frames[f].retired_buffers.clear();
        self.reuse_spares(f);
        self.frames[f].scratch = 0;
        unsafe {
            self.frames[f].alloc.Reset()?;
            self.list.Reset(&self.frames[f].alloc, None)?;
            self.list.SetDescriptorHeaps(&[Some(self.views.heap.clone()), Some(self.sampler_table.heap.clone())]);
        }
        Ok(())
    }

    pub fn render(&mut self, out: &Output, vertices: &[Vertex], fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        self.render_with(out, vertices.len(), &mut |dst| dst.copy_from_slice(vertices), fresh, xfb_addr, width, height)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_with(&mut self, out: &Output, count: usize, fill: &mut dyn FnMut(&mut [Vertex]), fresh: bool, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let f = self.frame;
        self.begin_frame(f)?;
        let list = self.list.clone();
        if fresh {
            self.upload_textures(f, &list, out)?;
            for c in &out.cmds {
                if let Cmd::CopyTex(c) = c {
                    let (w, h) = (at_scale(c.w, self.sx()), at_scale(c.h, self.sy()));
                    if self.textures.get(&c.id).is_some_and(|i| i.w == w && i.h == h) {
                        continue;
                    }
                    let mut img = create_image(&self.device, &mut self.srv_heap, &mut self.rtv_heap, w, h, 1, COLOR, true)?;
                    // Draws before the copy may sample it already.
                    transition(&list, &mut img, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
                    self.textures_epoch += 1;
                    if let Some(old) = self.textures.insert(c.id, img) {
                        self.trash_images.push(old);
                    }
                }
            }
        } else {
            self.stats.uploads = 0;
        }
        self.frames[f].described = None;
        self.write_draw_states(f, out)?;
        let vlen = count as u64 * u64::from(VERTEX_SIZE);
        self.index_offset = vlen.next_multiple_of(4);
        let total = (self.index_offset + out.indices.len() as u64 * 4).max(u64::from(VERTEX_SIZE));
        self.frame_buffer(f, 0, total)?;
        {
            let at = self.index_offset as usize;
            let b = self.frames[f].vertices.as_mut().unwrap();
            let dst = b.bytes();
            let bytes = &mut dst[..count * std::mem::size_of::<Vertex>()];
            assert!(bytes.as_ptr().align_offset(std::mem::align_of::<Vertex>()) == 0, "vertex buffer misaligned");
            // SAFETY: aligned (checked), large enough, `Vertex` is plain floats.
            let verts = unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast::<Vertex>(), count) };
            fill(verts);
            let idx = as_bytes(&out.indices);
            dst[at..at + idx.len()].copy_from_slice(idx);
        }
        let vb = self.frames[f].vertices.as_ref().unwrap();
        let (vb_gpu, vb_size) = (vb.gpu(), vb.size);
        self.record_efb(f, &list, vb_gpu, vb_size, None, out, fresh)?;
        let presented = self.present(f, &list, xfb_addr, width, height)?;
        self.frame = (f + 1) % FRAMES;
        Ok(presented)
    }

    fn reuse_spares(&mut self, f: usize) {
        const KEEP: usize = 3;
        let back = std::mem::take(&mut self.frames[f].retired_spare);
        self.spare.extend(back);
        if self.spare.len() > KEEP {
            self.spare.sort_by_key(|b| std::cmp::Reverse(b.size));
            self.spare.truncate(KEEP);
        }
    }

    pub fn prepared(&self) -> Option<(u64, u64)> {
        self.prepared.as_ref().map(|p| p.key)
    }

    pub fn prepare(&mut self, key: (u64, u64), out: &Output, moves: Option<&gx::interp::Moves>) -> Res<()> {
        self.prepare_with(key, out, &out.vertices, moves)
    }

    pub fn prepare_with(&mut self, key: (u64, u64), out: &Output, vertices: &[Vertex], moves: Option<&gx::interp::Moves>) -> Res<()> {
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
            None => create_buffer(&self.device, need.next_power_of_two(), D3D12_HEAP_TYPE_UPLOAD)?,
        };
        {
            let dst = buffer.bytes();
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
            self.trash_spare.push(old.buffer);
        }
        Ok(())
    }

    pub fn render_prepared(&mut self, out: &Output, t: f32, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let f = self.frame;
        self.begin_frame(f)?;
        let list = self.list.clone();
        self.stats.uploads = 0;
        let p = self.prepared.as_ref().ok_or("render_prepared: nothing prepared")?;
        let (serial, vb_gpu, vb_size, index_offset, moves) = (p.serial, p.buffer.gpu(), p.buffer.size, p.index_offset, p.moves);
        let described = (serial, self.textures_epoch);
        if self.frames[f].described != Some(described) {
            self.write_draw_states(f, out)?;
            self.frames[f].described = Some(described);
        }
        self.index_offset = index_offset;
        self.push = [t.to_bits(), u32::from(moves.is_some())];
        let recorded = self.record_efb(f, &list, vb_gpu, vb_size, moves.map(|m| m.map(|(o, _)| vb_gpu + o)), out, false);
        self.push = [0; 2];
        recorded?;
        let presented = self.present(f, &list, xfb_addr, width, height)?;
        self.frame = (f + 1) % FRAMES;
        Ok(presented)
    }

    pub fn evict(&mut self, ids: &[u32]) {
        for id in ids {
            if let Some(img) = self.textures.remove(id) {
                self.textures_epoch += 1;
                self.trash_images.push(img);
            }
        }
    }

    pub fn take_writebacks(&mut self) -> Vec<Writeback> {
        std::mem::take(&mut self.writebacks)
    }

    pub fn finish(&mut self) -> Res<()> {
        self.wait_idle()?;
        drain_messages(&self.gpu);
        for f in 0..FRAMES {
            self.collect_writebacks(f);
        }
        Ok(())
    }

    fn collect_writebacks(&mut self, f: usize) {
        let pending = std::mem::take(&mut self.frames[f].writebacks);
        if pending.is_empty() {
            return;
        }
        let b = self.frames[f].readback.as_mut().expect("readback buffer");
        let bytes = b.bytes();
        for p in pending {
            let row = p.w as usize * 4;
            let mut rgba = Vec::with_capacity(row * p.h as usize);
            for y in 0..p.h as usize {
                rgba.extend_from_slice(&bytes[p.offset + y * p.pitch..][..row]);
            }
            self.writebacks.push(Writeback { addr: p.addr, ram_hash: p.ram_hash, tex_format: p.tex_format, width: p.w, height: p.h, stride: p.stride, rgba });
        }
    }

    pub fn efb_snapshot(&mut self) -> Res<EfbSnapshot> {
        self.wait_idle()?;
        let pitch = (EFB_W * 4).next_multiple_of(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT) as usize;
        let bytes = (pitch * EFB_H as usize) as u64;
        let color_buf = create_buffer(&self.device, bytes, D3D12_HEAP_TYPE_READBACK)?;
        let depth_buf = create_buffer(&self.device, bytes, D3D12_HEAP_TYPE_READBACK)?;
        let full = Rect { x: 0, y: 0, w: EFB_W, h: EFB_H };
        let (cb, db) = (color_buf.res.clone(), depth_buf.res.clone());
        self.one_shot(|r, list| {
            unsafe { list.SetDescriptorHeaps(&[Some(r.views.heap.clone()), Some(r.sampler_table.heap.clone())]) };
            let src = r.scaled_f(full);
            for (flags, buf) in [(4u32, &cb), (2 | 4, &db)] {
                let params = CopyParams { src, dst: [EFB_W as f32, EFB_H as f32], format: 6, flags };
                r.copy_to_native(list, 0, params, buf, 0)?;
            }
            Ok(())
        })?;
        let read = |mut b: Buffer| -> Vec<u8> {
            let s = b.bytes();
            (0..EFB_H as usize).flat_map(|y| s[y * pitch..][..EFB_W as usize * 4].to_vec()).collect()
        };
        let color = read(color_buf);
        let d = read(depth_buf);
        let depth = d.chunks_exact(4).map(|p| u32::from(p[0]) << 16 | u32::from(p[1]) << 8 | u32::from(p[2])).collect();
        Ok(EfbSnapshot { color, depth })
    }

    fn upload_textures(&mut self, f: usize, list: &ID3D12GraphicsCommandList, out: &Output) -> Res<()> {
        self.stats.uploads = out.new_textures.len();
        fn source(t: &gx::texture::NewTexture) -> (u32, u32, &[Vec<u8>]) {
            match &t.replacement {
                Some(r) => (r.width, r.height, r.levels.as_slice()),
                None => (t.width, t.height, t.levels.as_slice()),
            }
        }
        if out.new_textures.iter().all(|t| source(t).2.is_empty()) {
            return Ok(());
        }
        // Each level at a 512-byte boundary, its rows 256 bytes apart.
        let mut total = 0u64;
        for t in &out.new_textures {
            let (mut w, mut h, levels) = source(t);
            for _ in levels {
                total = total.next_multiple_of(u64::from(D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT));
                total += u64::from((w * 4).next_multiple_of(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT) * h);
                w = (w / 2).max(1);
                h = (h / 2).max(1);
            }
        }
        self.frame_buffer(f, 2, total)?;
        let mut copies = Vec::new();
        {
            let bytes = self.frames[f].staging.as_mut().unwrap().bytes();
            let mut at = 0u64;
            for t in &out.new_textures {
                let (width, height, levels) = source(t);
                let (mut w, mut h) = (width, height);
                let mut regions = Vec::new();
                for data in levels {
                    at = at.next_multiple_of(u64::from(D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT));
                    let pitch = (w * 4).next_multiple_of(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT);
                    let row = w as usize * 4;
                    for y in 0..h as usize {
                        bytes[at as usize + y * pitch as usize..][..row].copy_from_slice(&data[y * row..][..row]);
                    }
                    regions.push(D3D12_PLACED_SUBRESOURCE_FOOTPRINT { Offset: at, Footprint: D3D12_SUBRESOURCE_FOOTPRINT { Format: COLOR, Width: w, Height: h, Depth: 1, RowPitch: pitch } });
                    at += u64::from(pitch * h);
                    w = (w / 2).max(1);
                    h = (h / 2).max(1);
                }
                copies.push((t.id, width, height, levels.len() as u32, regions));
            }
        }
        let staging = self.frames[f].staging.as_ref().unwrap().res.clone();
        for (id, w, h, levels, regions) in copies {
            // Render targets too: EFB copies may be patched into them.
            let mut img = create_image(&self.device, &mut self.srv_heap, &mut self.rtv_heap, w, h, levels, COLOR, true)?;
            transition(list, &mut img, D3D12_RESOURCE_STATE_COPY_DEST);
            for (level, fp) in regions.into_iter().enumerate() {
                copy_buffer_to_image(list, &staging, fp, &img.res, level as u32, 0, 0);
            }
            transition(list, &mut img, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
            self.textures_epoch += 1;
            if let Some(old) = self.textures.insert(id, img) {
                self.trash_images.push(old);
            }
        }
        if total > 32 << 20 {
            if let Some(b) = self.frames[f].staging.take() {
                self.trash_buffers.push(b);
            }
        }
        Ok(())
    }

    /// Fills frame `f`'s draw states (texture ids and sampler keys replaced
    /// by slots of its tables, the EFB scale filled in) and its tables.
    fn write_draw_states(&mut self, f: usize, out: &Output) -> Res<()> {
        let mut tex_slots: HashMap<u32, u32> = HashMap::new();
        let mut views: Vec<u32> = Vec::new();
        let mut sampler_slots: HashMap<u32, u32> = HashMap::new();
        let mut samplers: Vec<u32> = Vec::new();
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
                        views.push(img.srv.expect("texture view"));
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
                        samplers.push(self.sampler(key)?);
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
        self.frame_buffer(f, 1, bytes)?;
        let b = self.frames[f].shadings.as_mut().unwrap();
        b.bytes()[..bytes as usize].copy_from_slice(as_bytes(&words));
        self.frames[f].shadings_gpu = b.gpu();
        let base = f as u32 * FRAME_VIEWS;
        for (i, &v) in views.iter().enumerate() {
            unsafe { self.device.CopyDescriptorsSimple(1, self.views.cpu(base + i as u32), self.srv_heap.cpu(v), D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) };
        }
        let sbase = f as u32 * MAX_SAMPLERS;
        for (i, &s) in samplers.iter().enumerate() {
            unsafe { self.device.CopyDescriptorsSimple(1, self.sampler_table.cpu(sbase + i as u32), self.sampler_heap.cpu(s), D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER) };
        }
        // The draw index buffer: 0, 1, 2... as many as there are draw states.
        let n = (words.len() / SHADING_WORDS) as u64;
        if self.instance_ids.as_ref().is_none_or(|b| b.size < n * 4) {
            let mut b = create_buffer(&self.device, (n * 4).next_power_of_two().max(4096), D3D12_HEAP_TYPE_UPLOAD)?;
            let ids: Vec<u32> = (0..(b.size / 4) as u32).collect();
            b.bytes().copy_from_slice(as_bytes(&ids));
            if let Some(old) = self.instance_ids.replace(b) {
                self.trash_buffers.push(old);
            }
        }
        Ok(())
    }

    /// A view of frame `f`'s scratch part of the shader-visible heap,
    /// holding a copy of CPU view `srv`, for a table starting at it.
    fn scratch_views(&mut self, f: usize, srvs: &[u32]) -> Res<D3D12_GPU_DESCRIPTOR_HANDLE> {
        let fr = &mut self.frames[f];
        if fr.scratch + srvs.len() as u32 > SCRATCH {
            return Err("out of scratch views this frame".into());
        }
        let at = f as u32 * FRAME_VIEWS + MAX_TEXTURES + fr.scratch;
        fr.scratch += srvs.len() as u32;
        for (i, &s) in srvs.iter().enumerate() {
            unsafe { self.device.CopyDescriptorsSimple(1, self.views.cpu(at + i as u32), self.srv_heap.cpu(s), D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) };
        }
        Ok(self.views.gpu(at))
    }

    fn scaled(&self, r: Rect) -> RECT {
        let edge = |v: u32, s: f32, size: u32| ((v as f32 * s).round() as u32).min(size);
        let (iw, ih) = (self.efb_color.w, self.efb_color.h);
        let (x0, y0) = (edge(r.x, self.sx(), iw - 1), edge(r.y, self.sy(), ih - 1));
        let (x1, y1) = (edge(r.x + r.w, self.sx(), iw).max(x0 + 1), edge(r.y + r.h, self.sy(), ih).max(y0 + 1));
        RECT { left: x0 as i32, top: y0 as i32, right: x1 as i32, bottom: y1 as i32 }
    }

    fn scaled_f(&self, r: Rect) -> [f32; 4] {
        let (sx, sy) = (self.sx(), self.sy());
        [r.x as f32 * sx, r.y as f32 * sy, r.w as f32 * sx, r.h as f32 * sy]
    }

    /// Binds the EFB as the render target with everything the draws use.
    fn begin_efb(&mut self, f: usize, list: &ID3D12GraphicsCommandList, vb: (u64, u64), moves: Option<[u64; 3]>) {
        transition(list, &mut self.efb_color, D3D12_RESOURCE_STATE_RENDER_TARGET);
        transition(list, &mut self.efb_depth, D3D12_RESOURCE_STATE_DEPTH_WRITE);
        unsafe {
            let rtv = self.rtv_heap.cpu(self.efb_color.rtv.unwrap());
            let dsv = self.dsv_heap.cpu(self.efb_depth.dsv.unwrap());
            list.OMSetRenderTargets(1, Some(&rtv), false, Some(&dsv));
            list.RSSetViewports(&[viewport(0.0, 0.0, self.efb_color.w as f32, self.efb_color.h as f32)]);
            list.SetGraphicsRootSignature(&self.gpu.root);
            list.SetGraphicsRoot32BitConstants(ROOT_INTERP, 2, self.push.as_ptr().cast(), 0);
            let shadings = self.frames[f].shadings_gpu;
            list.SetGraphicsRootShaderResourceView(ROOT_SHADINGS, shadings);
            let m = moves.unwrap_or([shadings; 3]);
            list.SetGraphicsRootShaderResourceView(ROOT_MODEL, m[0]);
            list.SetGraphicsRootShaderResourceView(ROOT_MATS, m[1]);
            list.SetGraphicsRootShaderResourceView(ROOT_RECS, m[2]);
            list.SetGraphicsRootShaderResourceView(ROOT_SAMPLER_INDEX, self.gpu.sampler_index.gpu());
            list.SetGraphicsRootDescriptorTable(ROOT_TEXTURES, self.views.gpu(f as u32 * FRAME_VIEWS));
            list.SetGraphicsRootDescriptorTable(ROOT_SAMPLERS, self.sampler_table.gpu(f as u32 * MAX_SAMPLERS));
            list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            let ids = self.instance_ids.as_ref().unwrap();
            let vbs = [
                D3D12_VERTEX_BUFFER_VIEW { BufferLocation: vb.0, SizeInBytes: vb.1 as u32, StrideInBytes: VERTEX_SIZE },
                D3D12_VERTEX_BUFFER_VIEW { BufferLocation: ids.gpu(), SizeInBytes: ids.size as u32, StrideInBytes: 4 },
            ];
            list.IASetVertexBuffers(0, Some(&vbs));
            let ib = D3D12_INDEX_BUFFER_VIEW { BufferLocation: vb.0 + self.index_offset, SizeInBytes: (vb.1 - self.index_offset) as u32, Format: DXGI_FORMAT_R32_UINT };
            list.IASetIndexBuffer(Some(&ib));
        }
    }

    /// The copy pass from the EFB into the `params.dst` texels at the top
    /// left of render target `rtv` (frame `f`'s views).
    fn copy_pass(&mut self, f: usize, list: &ID3D12GraphicsCommandList, rtv: u32, params: CopyParams) -> Res<()> {
        transition(list, &mut self.efb_color, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
        transition(list, &mut self.efb_depth, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
        let table = self.scratch_views(f, &[self.efb_color.srv.unwrap(), self.efb_depth.srv.unwrap()])?;
        let (w, h) = (params.dst[0], params.dst[1]);
        unsafe {
            let target = self.rtv_heap.cpu(rtv);
            list.OMSetRenderTargets(1, Some(&target), false, None);
            list.SetPipelineState(&self.copy_pso);
            list.SetGraphicsRootSignature(&self.gpu.copy_root);
            list.SetGraphicsRoot32BitConstants(0, (std::mem::size_of::<CopyParams>() / 4) as u32, (&params as *const CopyParams).cast(), 0);
            list.SetGraphicsRootDescriptorTable(1, table);
            list.RSSetViewports(&[viewport(0.0, 0.0, w, h)]);
            list.RSSetScissorRects(&[rect(0, 0, w as u32, h as u32)]);
            list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            list.DrawInstanced(3, 1, 0, 0);
        }
        Ok(())
    }

    /// The copy pass at native size, then into `buffer` at `offset`; returns
    /// the rows' pitch there.
    fn copy_to_native(&mut self, list: &ID3D12GraphicsCommandList, f: usize, params: CopyParams, buffer: &ID3D12Resource, offset: u64) -> Res<usize> {
        transition(list, &mut self.native, D3D12_RESOURCE_STATE_RENDER_TARGET);
        self.copy_pass(f, list, self.native.rtv.unwrap(), params)?;
        transition(list, &mut self.native, D3D12_RESOURCE_STATE_COPY_SOURCE);
        let (w, h) = (params.dst[0] as u32, params.dst[1] as u32);
        let pitch = (w * 4).next_multiple_of(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT);
        let fp = D3D12_PLACED_SUBRESOURCE_FOOTPRINT { Offset: offset, Footprint: D3D12_SUBRESOURCE_FOOTPRINT { Format: COLOR, Width: w, Height: h, Depth: 1, RowPitch: pitch } };
        copy_image_to_buffer(list, &self.native.res, fp, buffer, (0, 0, w, h));
        Ok(pitch as usize)
    }

    /// A bilinear blit of `src` (x, y, w, h of a `sw` x `sh` image whose CPU
    /// view is `srv`) onto `dst` (x, y, w, h) of render target `rtv`.
    #[allow(clippy::too_many_arguments)]
    fn blit(&mut self, f: usize, list: &ID3D12GraphicsCommandList, srv: u32, src: (u32, u32, u32, u32), sw: u32, sh: u32, rtv: D3D12_CPU_DESCRIPTOR_HANDLE, dst: (u32, u32, u32, u32), swap: bool) -> Res<()> {
        let table = self.scratch_views(f, &[srv])?;
        let params = BlitParams::new(src, sw, sh, dst);
        unsafe {
            list.OMSetRenderTargets(1, Some(&rtv), false, None);
            list.SetPipelineState(if swap { &self.blit_swap_pso } else { &self.blit_pso });
            list.SetGraphicsRootSignature(&self.gpu.blit_root);
            list.SetGraphicsRoot32BitConstants(0, 8, (&params as *const BlitParams).cast(), 0);
            list.SetGraphicsRootDescriptorTable(1, table);
            list.RSSetViewports(&[viewport(dst.0 as f32, dst.1 as f32, dst.2 as f32, dst.3 as f32)]);
            list.RSSetScissorRects(&[rect(dst.0, dst.1, dst.2, dst.3)]);
            list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            list.DrawInstanced(3, 1, 0, 0);
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn record_efb(&mut self, f: usize, list: &ID3D12GraphicsCommandList, vb_gpu: u64, vb_size: u64, moves: Option<[u64; 3]>, out: &Output, fresh: bool) -> Res<()> {
        if !self.efb_ready {
            transition(list, &mut self.efb_color, D3D12_RESOURCE_STATE_RENDER_TARGET);
            transition(list, &mut self.efb_depth, D3D12_RESOURCE_STATE_DEPTH_WRITE);
            unsafe {
                list.ClearRenderTargetView(self.rtv_heap.cpu(self.efb_color.rtv.unwrap()), &[0.0, 0.0, 0.0, 1.0], None);
                list.ClearDepthStencilView(self.dsv_heap.cpu(self.efb_depth.dsv.unwrap()), D3D12_CLEAR_FLAG_DEPTH, 1.0, 0, None);
            }
            self.efb_ready = true;
        }
        let row_bytes = |w: u32, h: u32| u64::from((w * 4).next_multiple_of(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT) * h).next_multiple_of(u64::from(D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT));
        let readback_bytes: u64 = if fresh && self.copies_to_ram { out.cmds.iter().filter_map(|c| if let Cmd::CopyTex(c) = c { Some(row_bytes(c.w, c.h)) } else { None }).sum() } else { 0 };
        if readback_bytes > 0 {
            self.frame_buffer(f, 3, readback_bytes)?;
        }
        let readback = (readback_bytes > 0).then(|| self.frames[f].readback.as_ref().unwrap().res.clone());
        let mut readback_at = 0u64;
        let mut drawing = false;
        let mut bound: Option<u32> = None;
        let mut last_scissor: Option<RECT> = None;
        let mut draws = 0;
        for c in &out.cmds {
            match *c {
                Cmd::Draw(d) => {
                    if d.scissor.w == 0 || d.scissor.h == 0 || d.raster.cull == 3 {
                        continue;
                    }
                    if !drawing {
                        self.begin_efb(f, list, (vb_gpu, vb_size), moves);
                        drawing = true;
                        bound = None;
                        last_scissor = None;
                    }
                    let early = out.shadings.get(d.shading as usize).is_some_and(|s| rasterised_depth(&s.0));
                    let key = full_key(blend_key(&d.raster) | if early { EARLY_DEPTH } else { 0 }, d.raster.cull, d.raster.zmode);
                    if bound != Some(key) {
                        let p = self.pipeline(key)?;
                        unsafe { list.SetPipelineState(&p) };
                        let uint = uses_uint(key, self.gpu.logic_ops);
                        if bound.is_some_and(|b| uses_uint(b, self.gpu.logic_ops)) != uint {
                            let rtv = self.rtv_heap.cpu(if uint { self.efb_color.rtv_uint } else { self.efb_color.rtv }.unwrap());
                            let dsv = self.dsv_heap.cpu(self.efb_depth.dsv.unwrap());
                            unsafe { list.OMSetRenderTargets(1, Some(&rtv), false, Some(&dsv)) };
                        }
                        bound = Some(key);
                    }
                    let scissor = self.scaled(d.scissor);
                    unsafe {
                        if last_scissor != Some(scissor) {
                            list.RSSetScissorRects(&[scissor]);
                            last_scissor = Some(scissor);
                        }
                        list.DrawIndexedInstanced(d.count, 1, d.first, 0, d.shading);
                    }
                    draws += 1;
                }
                Cmd::Clear(c) => {
                    if !drawing {
                        self.begin_efb(f, list, (vb_gpu, vb_size), moves);
                        drawing = true;
                        bound = None;
                        last_scissor = None;
                    }
                    let r = [self.scaled(c.rect)];
                    unsafe {
                        if c.color_update || c.alpha_update {
                            let col = c.color.map(|v| f32::from(v) / 255.0);
                            list.ClearRenderTargetView(self.rtv_heap.cpu(self.efb_color.rtv.unwrap()), &col, Some(&r));
                        }
                        if c.z_update {
                            let depth = c.z as f32 / 16_777_216.0;
                            list.ClearDepthStencilView(self.dsv_heap.cpu(self.efb_depth.dsv.unwrap()), D3D12_CLEAR_FLAG_DEPTH, depth, 0, Some(&r));
                        }
                    }
                }
                Cmd::CopyXfb { src, addr, height } => {
                    drawing = false;
                    let (w, h) = (at_scale(src.w, self.sx()), at_scale(height, self.sy()));
                    if self.xfbs.get(&addr).is_some_and(|(img, _)| img.w != w || img.h != h) {
                        let (img, _) = self.xfbs.remove(&addr).unwrap();
                        self.trash_images.push(img);
                    }
                    if !self.xfbs.contains_key(&addr) {
                        if self.xfbs.len() >= MAX_XFBS {
                            let oldest = *self.xfbs.iter().min_by_key(|(_, (_, s))| *s).unwrap().0;
                            let (img, _) = self.xfbs.remove(&oldest).unwrap();
                            self.trash_images.push(img);
                        }
                        let img = create_image(&self.device, &mut self.srv_heap, &mut self.rtv_heap, w, h, 1, COLOR, true)?;
                        self.xfbs.insert(addr, (img, 0));
                    }
                    self.xfb_serial += 1;
                    let serial = self.xfb_serial;
                    let s = self.scaled(src);
                    let src_rect = (s.left as u32, s.top as u32, (s.right - s.left) as u32, (s.bottom - s.top) as u32);
                    transition(list, &mut self.efb_color, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
                    let (dst, ser) = self.xfbs.get_mut(&addr).unwrap();
                    *ser = serial;
                    transition(list, dst, D3D12_RESOURCE_STATE_RENDER_TARGET);
                    let rtv = self.rtv_heap.cpu(dst.rtv.unwrap());
                    let (srv, ew, eh) = (self.efb_color.srv.unwrap(), self.efb_color.w, self.efb_color.h);
                    self.blit(f, list, srv, src_rect, ew, eh, rtv, (0, 0, w, h), false)?;
                    let (dst, _) = self.xfbs.get_mut(&addr).unwrap();
                    transition(list, dst, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
                }
                Cmd::CopyTex(c) => {
                    drawing = false;
                    let flags = u32::from(c.intensity) | u32::from(c.depth) << 1 | u32::from(c.efb_alpha) << 2;
                    let src = self.scaled_f(c.src);
                    if let Some(dst) = self.textures.get_mut(&c.id) {
                        transition(list, dst, D3D12_RESOURCE_STATE_RENDER_TARGET);
                        let params = CopyParams { src, dst: [dst.w as f32, dst.h as f32], format: c.format, flags };
                        let rtv = dst.rtv.unwrap();
                        self.copy_pass(f, list, rtv, params)?;
                        let dst = self.textures.get_mut(&c.id).unwrap();
                        transition(list, dst, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
                    }
                    if let Some(buffer) = &readback {
                        let params = CopyParams { src, dst: [c.w as f32, c.h as f32], format: c.format, flags };
                        let pitch = self.copy_to_native(list, f, params, buffer, readback_at)?;
                        self.frames[f].writebacks.push(PendingWriteback { addr: c.addr, ram_hash: c.ram_hash, tex_format: c.tex_format, w: c.w, h: c.h, stride: c.stride, offset: readback_at as usize, pitch });
                        readback_at += row_bytes(c.w, c.h);
                    }
                }
                Cmd::Patch(p) => {
                    drawing = false;
                    if p.copy != p.target {
                        self.patch(f, list, p)?;
                    }
                }
            }
        }
        self.stats.draws = draws;
        self.stats.pipelines = self.pipelines.len();
        Ok(())
    }

    /// Draws EFB copy `p.copy` into texture `p.target` at the patch's place.
    fn patch(&mut self, f: usize, list: &ID3D12GraphicsCommandList, p: gx::Patch) -> Res<()> {
        let (Some(src), Some(dst)) = (self.textures.get(&p.copy), self.textures.get(&p.target)) else { return Ok(()) };
        if p.x + p.w > dst.w || p.y + p.h > dst.h {
            return Ok(());
        }
        let (srv, sw, sh, rtv) = (src.srv.unwrap(), src.w, src.h, dst.rtv.unwrap());
        transition(list, self.textures.get_mut(&p.target).unwrap(), D3D12_RESOURCE_STATE_RENDER_TARGET);
        let rtv = self.rtv_heap.cpu(rtv);
        self.blit(f, list, srv, (0, 0, sw, sh), sw, sh, rtv, (p.x, p.y, p.w, p.h), false)?;
        transition(list, self.textures.get_mut(&p.target).unwrap(), D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
        Ok(())
    }

    /// Draws the menus into the bound swapchain image.
    fn draw_ui(&mut self, f: usize, list: &ID3D12GraphicsCommandList, rtv: D3D12_CPU_DESCRIPTOR_HANDLE) -> Res<()> {
        let Some(frame) = &self.ui_frame else { return Ok(()) };
        if frame.primitives.is_empty() {
            return Ok(());
        }
        let ppp = frame.pixels_per_point;
        let (tw, th) = self.swap_size;
        // Vertices then indices of every mesh, in the frame's buffer.
        let mut vbytes = Vec::new();
        let mut ibytes: Vec<u8> = Vec::new();
        let mut draws = Vec::new();
        for p in &frame.primitives {
            let egui::epaint::Primitive::Mesh(mesh) = &p.primitive else { continue };
            if mesh.indices.is_empty() {
                continue;
            }
            let c = p.clip_rect;
            let x0 = (c.min.x * ppp).round().clamp(0.0, tw as f32) as u32;
            let y0 = (c.min.y * ppp).round().clamp(0.0, th as f32) as u32;
            let x1 = (c.max.x * ppp).round().clamp(0.0, tw as f32) as u32;
            let y1 = (c.max.y * ppp).round().clamp(0.0, th as f32) as u32;
            if x1 <= x0 || y1 <= y0 {
                continue;
            }
            let base = (vbytes.len() / UI_VERTEX_SIZE as usize) as i32;
            vbytes.extend_from_slice(as_bytes(&mesh.vertices));
            let first = (ibytes.len() / 4) as u32;
            ibytes.extend_from_slice(as_bytes(&mesh.indices));
            draws.push((mesh.texture_id, RECT { left: x0 as i32, top: y0 as i32, right: x1 as i32, bottom: y1 as i32 }, first, mesh.indices.len() as u32, base));
        }
        if draws.is_empty() {
            return Ok(());
        }
        let ioff = (vbytes.len() as u64).next_multiple_of(256);
        let need = ioff + ibytes.len() as u64;
        if self.ui.buffers[f].as_ref().is_none_or(|b| b.size < need) {
            let b = create_buffer(&self.device, need.next_power_of_two(), D3D12_HEAP_TYPE_UPLOAD)?;
            if let Some(old) = self.ui.buffers[f].replace(b) {
                self.trash_buffers.push(old);
            }
        }
        let b = self.ui.buffers[f].as_mut().unwrap();
        b.bytes()[..vbytes.len()].copy_from_slice(&vbytes);
        b.bytes()[ioff as usize..need as usize].copy_from_slice(&ibytes);
        let (gpu, size) = (b.gpu(), b.size);
        let screen = [tw as f32 / ppp, th as f32 / ppp, 0.0, 0.0];
        unsafe {
            list.OMSetRenderTargets(1, Some(&rtv), false, None);
            list.SetPipelineState(&self.ui_pso);
            list.SetGraphicsRootSignature(&self.gpu.blit_root);
            list.SetGraphicsRoot32BitConstants(0, 4, screen.as_ptr().cast(), 0);
            list.RSSetViewports(&[viewport(0.0, 0.0, tw as f32, th as f32)]);
            list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            list.IASetVertexBuffers(0, Some(&[D3D12_VERTEX_BUFFER_VIEW { BufferLocation: gpu, SizeInBytes: ioff as u32, StrideInBytes: UI_VERTEX_SIZE }]));
            list.IASetIndexBuffer(Some(&D3D12_INDEX_BUFFER_VIEW { BufferLocation: gpu + ioff, SizeInBytes: (size - ioff) as u32, Format: DXGI_FORMAT_R32_UINT }));
        }
        for (tex, clip, first, count, base) in draws {
            let Some(srv) = self.ui.textures.get(&tex).and_then(|t| t.srv) else { continue };
            let table = self.scratch_views(f, &[srv])?;
            unsafe {
                list.SetGraphicsRootDescriptorTable(1, table);
                list.RSSetScissorRects(&[clip]);
                list.DrawIndexedInstanced(count, 1, first, base, 0);
            }
        }
        Ok(())
    }

    fn present(&mut self, f: usize, list: &ID3D12GraphicsCommandList, xfb_addr: u32, width: u32, height: u32) -> Res<bool> {
        let xfb = self.xfbs.get(&xfb_addr).map(|_| xfb_addr).or_else(|| self.xfbs.iter().max_by_key(|(_, (_, s))| *s).map(|(a, _)| *a));
        let mut readback = None;
        if let Some(addr) = xfb.filter(|_| self.screenshot.is_some()) {
            let path = self.screenshot.take().unwrap();
            let (img, _) = self.xfbs.get_mut(&addr).unwrap();
            let (w, h) = (img.w, img.h);
            let pitch = (w * 4).next_multiple_of(D3D12_TEXTURE_DATA_PITCH_ALIGNMENT);
            let buf = create_buffer(&self.device, u64::from(pitch * h), D3D12_HEAP_TYPE_READBACK)?;
            transition(list, img, D3D12_RESOURCE_STATE_COPY_SOURCE);
            let fp = D3D12_PLACED_SUBRESOURCE_FOOTPRINT { Offset: 0, Footprint: D3D12_SUBRESOURCE_FOOTPRINT { Format: COLOR, Width: w, Height: h, Depth: 1, RowPitch: pitch } };
            copy_image_to_buffer(list, &img.res, fp, &buf.res, (0, 0, w, h));
            transition(list, img, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
            readback = Some((path, buf, w, h, pitch as usize));
        }

        let mut index = None;
        if width > 0 && height > 0 && self.has_window() {
            if self.needs_rebuild || self.swap_size != (width, height) {
                // The list is open: rebuilding waits for the GPU, which is fine.
                self.rebuild(width, height)?;
            }
            if let Some(s) = &self.swapchain {
                index = Some(unsafe { s.GetCurrentBackBufferIndex() } as usize);
            }
        }
        if let Some(i) = index {
            let (tw, th) = self.swap_size;
            let target = &mut self.swap_images[i];
            transition(list, target, D3D12_RESOURCE_STATE_RENDER_TARGET);
            let rtv = self.rtv_heap.cpu(target.rtv.unwrap());
            unsafe { list.ClearRenderTargetView(rtv, &[0.0, 0.0, 0.0, 1.0], None) };
            if let Some(addr) = xfb {
                let (src, _) = &self.xfbs[&addr];
                let (srv, sw, sh) = (src.srv.unwrap(), src.w, src.h);
                let dst = render::letterbox(tw, th, self.aspect);
                self.blit(f, list, srv, (0, 0, sw, sh), sw, sh, rtv, dst, true)?;
            }
            for (x0, y0, x1, y1, c) in render::overlay_rects(&self.overlay, tw, th) {
                let col = c.map(|v| f32::from(v) / 255.0);
                unsafe { list.ClearRenderTargetView(rtv, &col, Some(&[RECT { left: x0, top: y0, right: x1, bottom: y1 }])) };
            }
            self.draw_ui(f, list, rtv)?;
            transition(list, &mut self.swap_images[i], D3D12_RESOURCE_STATE_PRESENT);
        }
        unsafe {
            list.Close()?;
            self.gpu.queue.ExecuteCommandLists(&[Some(list.cast()?)]);
        }
        let mut presented = false;
        if index.is_some() {
            let s = self.swapchain.as_ref().unwrap();
            let r = unsafe { s.Present(u32::from(self.vsync), DXGI_PRESENT(0)) };
            if r.is_err() {
                // Lost (the device or the window): made again next time.
                self.needs_rebuild = true;
                r.ok()?;
            }
            presented = true;
        }
        self.fence_value += 1;
        unsafe { self.gpu.queue.Signal(&self.fence, self.fence_value)? };
        self.frames[f].fence_value = self.fence_value;
        self.frames[f].retired_images.append(&mut self.trash_images);
        self.frames[f].retired_buffers.append(&mut self.trash_buffers);
        self.frames[f].retired_spare.append(&mut self.trash_spare);
        if let Some((path, mut buf, w, h, pitch)) = readback {
            self.wait_fence(self.fence_value)?;
            let s = buf.bytes();
            let pixels: Vec<u8> = (0..h as usize).flat_map(|y| s[y * pitch..][..w as usize * 4].to_vec()).collect();
            render::save_png(&path, w, h, &pixels)?;
        }
        Ok(presented)
    }
}

fn validation_wanted() -> bool {
    cfg!(debug_assertions) || std::env::var_os("NSMBW_VALIDATION").is_some()
}

/// The EFB's colour: RGBA8 seen as UNORM (draws, copies) and as UINT
/// (logic-op draws).
fn create_efb_color(device: &ID3D12Device, srv: &mut Heap, rtv: &mut Heap, w: u32, h: u32) -> Res<Image> {
    let desc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
        Width: u64::from(w),
        Height: h,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT_R8G8B8A8_TYPELESS,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,
        ..Default::default()
    };
    unsafe {
        let mut res: Option<ID3D12Resource> = None;
        device.CreateCommittedResource(&heap_props(D3D12_HEAP_TYPE_DEFAULT), D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_COMMON, None, &mut res)?;
        let res = res.ok_or("no EFB image")?;
        let s = srv.alloc()?;
        let view = D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: COLOR,
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2D,
            Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 { Texture2D: D3D12_TEX2D_SRV { MipLevels: 1, ..Default::default() } },
        };
        device.CreateShaderResourceView(&res, Some(&view), srv.cpu(s));
        let mut views = [0u32; 2];
        for (v, format) in views.iter_mut().zip([COLOR, DXGI_FORMAT_R8G8B8A8_UINT]) {
            *v = rtv.alloc()?;
            let d = D3D12_RENDER_TARGET_VIEW_DESC { Format: format, ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2D, ..Default::default() };
            device.CreateRenderTargetView(&res, Some(&d), rtv.cpu(*v));
        }
        Ok(Image { res, w, h, state: D3D12_RESOURCE_STATE_COMMON, srv: Some(s), rtv: Some(views[0]), rtv_uint: Some(views[1]), dsv: None })
    }
}

/// A 2D image in the default heap with a view (and a render target view),
/// in the copy destination state.
#[allow(clippy::too_many_arguments)]
fn create_image(device: &ID3D12Device, srv: &mut Heap, rtv: &mut Heap, w: u32, h: u32, levels: u32, format: DXGI_FORMAT, target: bool) -> Res<Image> {
    let desc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
        Width: u64::from(w),
        Height: h,
        DepthOrArraySize: 1,
        MipLevels: levels as u16,
        Format: format,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Flags: if target { D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET } else { D3D12_RESOURCE_FLAG_NONE },
        ..Default::default()
    };
    unsafe {
        let mut res: Option<ID3D12Resource> = None;
        device.CreateCommittedResource(&heap_props(D3D12_HEAP_TYPE_DEFAULT), D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_COMMON, None, &mut res)?;
        let res = res.ok_or("no image")?;
        let s = srv.alloc()?;
        device.CreateShaderResourceView(&res, None, srv.cpu(s));
        let r = if target {
            let r = rtv.alloc()?;
            device.CreateRenderTargetView(&res, None, rtv.cpu(r));
            Some(r)
        } else {
            None
        };
        Ok(Image { res, w, h, state: D3D12_RESOURCE_STATE_COMMON, srv: Some(s), rtv: r, rtv_uint: None, dsv: None })
    }
}

fn copy_buffer_to_image(list: &ID3D12GraphicsCommandList, buffer: &ID3D12Resource, fp: D3D12_PLACED_SUBRESOURCE_FOOTPRINT, image: &ID3D12Resource, level: u32, x: u32, y: u32) {
    let src = D3D12_TEXTURE_COPY_LOCATION { pResource: borrowed(buffer), Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT, Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { PlacedFootprint: fp } };
    let dst = D3D12_TEXTURE_COPY_LOCATION { pResource: borrowed(image), Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX, Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { SubresourceIndex: level } };
    unsafe { list.CopyTextureRegion(&dst, x, y, 0, &src, None) };
}

fn copy_image_to_buffer(list: &ID3D12GraphicsCommandList, image: &ID3D12Resource, fp: D3D12_PLACED_SUBRESOURCE_FOOTPRINT, buffer: &ID3D12Resource, area: (u32, u32, u32, u32)) {
    let src = D3D12_TEXTURE_COPY_LOCATION { pResource: borrowed(image), Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX, Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { SubresourceIndex: 0 } };
    let dst = D3D12_TEXTURE_COPY_LOCATION { pResource: borrowed(buffer), Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT, Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { PlacedFootprint: fp } };
    let b = D3D12_BOX { left: area.0, top: area.1, front: 0, right: area.0 + area.2, bottom: area.1 + area.3, back: 1 };
    unsafe { list.CopyTextureRegion(&dst, 0, 0, 0, &src, Some(&b)) };
}

impl Drop for Renderer {
    fn drop(&mut self) {
        if let Some(w) = self.prewarm.take() {
            w.stop.store(true, Ordering::Relaxed);
            for t in w.workers {
                let _ = t.join();
            }
            for (key, p) in w.built.try_iter() {
                self.pipelines.entry(key).or_insert(p);
            }
        }
        let _ = self.wait_idle();
        drain_messages(&self.gpu);
        if let Some(path) = &self.keys_path {
            render::save_keys(path, self.pipelines.keys().copied());
        }
        unsafe {
            let _ = CloseHandle(self.event);
        }
    }
}

impl Backend for Renderer {
    fn api(&self) -> Api {
        Api::D3d12
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
        Renderer::set_vsync(self, on)
    }
    fn resized(&mut self) {
        Renderer::resized(self)
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
