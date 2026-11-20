// GX ubershader. The CPU (`gx` crate) has already transformed and lit the
// vertices; the fragment stage runs indirect texturing, the TEV, the alpha
// test, Z textures and fog from the packed BP state in `shadings[draw]`
// (word offsets mirror `gx::sh`). Integer maths follows the hardware's 8-bit
// inputs and 11-bit signed registers; fog and indirect texturing follow
// Dolphin's pixel shader so FIFO-log comparisons line up.

enable wgpu_binding_array;
enable dual_source_blending;

const GENMODE: u32 = 0u;
const TREF: u32 = 1u;
const ENV: u32 = 9u;
const KSEL: u32 = 41u;
const TEV_COLOR: u32 = 49u;
const TEV_KONST: u32 = 57u;
const ALPHA_TEST: u32 = 65u;
const PE_CONTROL: u32 = 66u;
const CONST_ALPHA: u32 = 67u;
const TEX: u32 = 72u;
const SAMPLER: u32 = 80u;
const IND_CMD: u32 = 88u;
const IREF: u32 = 104u;
const IND_SCALE: u32 = 105u;
const IND_MTX: u32 = 107u;
const TEX_SIZE: u32 = 116u;
const SU_SIZE: u32 = 124u;
const FOG: u32 = 132u;
const FOG_RANGE: u32 = 137u;
const FOG_WIDTH: u32 = 143u;
const ZTEX: u32 = 144u;
const FLAGS: u32 = 146u;
const CLEAR_COLOR: u32 = 147u;
const ZPLANE: u32 = 148u;
const SCALE: u32 = 151u;
const FLAG_CLEAR: u32 = 1u;
const FLAG_ZFREEZE: u32 = 2u;
const NONE: u32 = 0xFFFFFFFFu;

struct Shading {
    w: array<vec4<u32>, 40>,
}

@group(0) @binding(0) var<storage, read> shadings: array<Shading>;
@group(0) @binding(1) var textures: binding_array<texture_2d<f32>, 4096>;
@group(0) @binding(2) var samplers: binding_array<sampler, 256>;

// Frame interpolation on the GPU (`gx::interp::Moves`): for frames shown
// between two fields, a paired draw's vertices are placed again from their
// model positions with the two fields' matrices blended at `t` (rotation by
// quaternion, scale and translation linearly), as `gx::interp::interpolate_into`
// does on the CPU.
struct Interp {
    t: f32,
    on: u32,
}

struct MatPair {
    a0: vec4<f32>,
    a1: vec4<f32>,
    a2: vec4<f32>,
    b0: vec4<f32>,
    b1: vec4<f32>,
    b2: vec4<f32>,
    qa: vec4<f32>,
    qb: vec4<f32>,
    // w: 1 for a rigid blend, 0 for a linear one.
    sa: vec4<f32>,
    // w: index into `recs` (bits).
    sb: vec4<f32>,
}

struct RecPair {
    p0: vec4<f32>,
    p1: vec4<f32>,
    p2: vec4<f32>,
    v0: vec4<f32>,
    v1: vec4<f32>,
    v2: vec4<f32>,
    // Screen offset x, y; ortho (1/0).
    s: vec4<f32>,
}

var<immediate> interp: Interp;
// Per vertex: model position, and in w the bits of an index into `mats`.
@group(0) @binding(3) var<storage, read> model: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read> mats: array<MatPair>;
@group(0) @binding(5) var<storage, read> recs: array<RecPair>;

const EFB_W: f32 = 640.0;
const EFB_H: f32 = 528.0;

// `gx::xf::project`: a view-space position to Vulkan clip space, with the
// projection's six values in `pa`, `pb` and the viewport's (wd, ht,
// zRange, xOrig, yOrig, farZ) in `va`, `vb`.
fn project(p: vec3<f32>, pa: vec4<f32>, pb: vec2<f32>, ortho: bool, va: vec4<f32>, vb: vec2<f32>, screen: vec2<f32>) -> vec4<f32> {
    var clip: vec4<f32>;
    if ortho {
        clip = vec4<f32>(pa.x * p.x + pa.y, pa.z * p.y + pa.w, pb.x * p.z + pb.y, 1.0);
    } else {
        clip = vec4<f32>(pa.x * p.x + pa.y * p.z, pa.z * p.y + pa.w * p.z, pb.x * p.z + pb.y, -p.z);
    }
    let w = clip.w;
    let cx = vb.x - screen.x;
    let cy = vb.y - screen.y;
    return vec4<f32>(
        w * (cx / (EFB_W / 2.0) - 1.0) + clip.x * (va.x / (EFB_W / 2.0)),
        w * (cy / (EFB_H / 2.0) - 1.0) + clip.y * (va.y / (EFB_H / 2.0)),
        (w * va.w + clip.z * va.z) / 16777216.0,
        w,
    );
}

// The vertex at model position `p` under matrix pair `k`, at `t`.
fn moved(p: vec3<f32>, k: u32, t: f32) -> vec4<f32> {
    let m = mats[k];
    var r0 = mix(m.a0, m.b0, t);
    var r1 = mix(m.a1, m.b1, t);
    var r2 = mix(m.a2, m.b2, t);
    if m.sa.w != 0.0 {
        let qm = mix(m.qa, m.qb, t);
        let l = length(qm);
        var q = vec4<f32>(0.0, 0.0, 0.0, 1.0);
        if l > 0.0 {
            q = qm / l;
        }
        let s = mix(m.sa.xyz, m.sb.xyz, t);
        let x = q.x;
        let y = q.y;
        let z = q.z;
        let qw = q.w;
        r0 = vec4<f32>(vec3<f32>(1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - z * qw), 2.0 * (x * z + y * qw)) * s, r0.w);
        r1 = vec4<f32>(vec3<f32>(2.0 * (x * y + z * qw), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - x * qw)) * s, r1.w);
        r2 = vec4<f32>(vec3<f32>(2.0 * (x * z - y * qw), 2.0 * (y * z + x * qw), 1.0 - 2.0 * (x * x + y * y)) * s, r2.w);
    }
    let world = vec3<f32>(dot(r0.xyz, p) + r0.w, dot(r1.xyz, p) + r1.w, dot(r2.xyz, p) + r2.w);
    let r = recs[bitcast<u32>(m.sb.w)];
    // Projection a = (p0, p1.xy), b = (p1.zw, p2); viewports likewise.
    let pr = mix(r.p0, vec4<f32>(r.p1.zw, r.p2.xy), t);
    let pr2 = mix(r.p1.xy, r.p2.zw, t);
    let vp = mix(r.v0, vec4<f32>(r.v1.zw, r.v2.xy), t);
    let vp2 = mix(r.v1.xy, r.v2.zw, t);
    // `project` takes (wd, ht, zRange, farZ) and (xOrig, yOrig).
    return project(world, pr, pr2, r.s.z != 0.0, vec4<f32>(vp.x, vp.y, vp.z, vp2.y), vec2<f32>(vp.w, vp2.x), r.s.xy);
}

struct VIn {
    @location(0) pos: vec4<f32>,
    @location(1) c0: vec4<f32>,
    @location(2) c1: vec4<f32>,
    @location(3) t0: vec3<f32>,
    @location(4) t1: vec3<f32>,
    @location(5) t2: vec3<f32>,
    @location(6) t3: vec3<f32>,
    @location(7) t4: vec3<f32>,
    @location(8) t5: vec3<f32>,
    @location(9) t6: vec3<f32>,
    @location(10) t7: vec3<f32>,
}

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) c0: vec4<f32>,
    @location(1) c1: vec4<f32>,
    @location(2) t0: vec3<f32>,
    @location(3) t1: vec3<f32>,
    @location(4) t2: vec3<f32>,
    @location(5) t3: vec3<f32>,
    @location(6) t4: vec3<f32>,
    @location(7) t5: vec3<f32>,
    @location(8) t6: vec3<f32>,
    @location(9) t7: vec3<f32>,
    @location(10) @interpolate(flat) draw: u32,
}

// Colour written to the EFB (alpha may be the constant destination alpha),
// and the alpha blending uses.
struct FOut {
    @location(0) @blend_src(0) color: vec4<f32>,
    @location(0) @blend_src(1) blend: vec4<f32>,
    @builtin(frag_depth) depth: f32,
}

// The same without a depth output, for draws whose depth is the
// rasterised one (no z-freeze or Z texture): the GPU can then test depth
// before shading.
struct FOutEarly {
    @location(0) @blend_src(0) color: vec4<f32>,
    @location(0) @blend_src(1) blend: vec4<f32>,
}

@vertex
fn vs(v: VIn, @builtin(instance_index) inst: u32, @builtin(vertex_index) vi: u32) -> VOut {
    var o: VOut;
    o.pos = v.pos;
    if interp.on != 0u {
        let m = model[vi];
        let k = bitcast<u32>(m.w);
        if k != NONE {
            o.pos = moved(m.xyz, k, interp.t);
        }
    }
    o.c0 = v.c0;
    o.c1 = v.c1;
    o.t0 = v.t0;
    o.t1 = v.t1;
    o.t2 = v.t2;
    o.t3 = v.t3;
    o.t4 = v.t4;
    o.t5 = v.t5;
    o.t6 = v.t6;
    o.t7 = v.t7;
    o.draw = inst;
    return o;
}

var<private> sidx: u32;

fn word(i: u32) -> u32 {
    return shadings[sidx].w[i >> 2u][i & 3u];
}

fn s11(v: u32) -> i32 {
    return i32(v << 21u) >> 21u;
}

// TEV register pair at `base + 2i`: (R | A << 12, B | G << 12).
fn tev_reg(base: u32, i: u32) -> vec4<i32> {
    let lo = word(base + 2u * i);
    let hi = word(base + 2u * i + 1u);
    return vec4<i32>(s11(lo & 0x7FFu), s11((hi >> 12u) & 0x7FFu), s11(hi & 0x7FFu), s11((lo >> 12u) & 0x7FFu));
}

fn swap(c: vec4<i32>, table: u32) -> vec4<i32> {
    let rg = word(KSEL + 2u * table);
    let ba = word(KSEL + 2u * table + 1u);
    return vec4<i32>(c[rg & 3u], c[(rg >> 2u) & 3u], c[ba & 3u], c[(ba >> 2u) & 3u]);
}

// 255, 223, 191, 159, 128, 96, 64, 32 (without a local array, which GPUs
// keep in slow memory when indexed at run time).
fn konst_fraction(sel: u32) -> i32 {
    let s = i32(sel & 7u);
    return select(255 - 32 * s, 256 - 32 * s, s >= 4);
}

// The TEV registers and konst colours as four values each, picked by
// switch rather than by indexing an array.
struct Regs {
    r0: vec4<i32>,
    r1: vec4<i32>,
    r2: vec4<i32>,
    r3: vec4<i32>,
}

fn reg(r: Regs, i: u32) -> vec4<i32> {
    switch i {
        case 0u: { return r.r0; }
        case 1u: { return r.r1; }
        case 2u: { return r.r2; }
        default: { return r.r3; }
    }
}

// One of the eight texture coordinates (or their derivatives).
struct Coords {
    c0: vec2<f32>,
    c1: vec2<f32>,
    c2: vec2<f32>,
    c3: vec2<f32>,
    c4: vec2<f32>,
    c5: vec2<f32>,
    c6: vec2<f32>,
    c7: vec2<f32>,
}

fn coord_of(c: Coords, i: u32) -> vec2<f32> {
    switch i {
        case 0u: { return c.c0; }
        case 1u: { return c.c1; }
        case 2u: { return c.c2; }
        case 3u: { return c.c3; }
        case 4u: { return c.c4; }
        case 5u: { return c.c5; }
        case 6u: { return c.c6; }
        default: { return c.c7; }
    }
}

fn konst_color(sel: u32, k: Regs) -> vec3<i32> {
    if (sel < 8u) {
        return vec3<i32>(konst_fraction(sel));
    }
    if (sel >= 12u && sel < 16u) {
        return reg(k, sel - 12u).rgb;
    }
    if (sel >= 16u) {
        let kk = reg(k, (sel - 16u) & 3u);
        return vec3<i32>(kk[(sel - 16u) >> 2u]);
    }
    return vec3<i32>(0);
}

fn konst_alpha(sel: u32, k: Regs) -> i32 {
    if (sel < 8u) {
        return konst_fraction(sel);
    }
    if (sel >= 16u) {
        let kk = reg(k, (sel - 16u) & 3u);
        return kk[(sel - 16u) >> 2u];
    }
    return 0;
}

fn color_in(i: u32, r: Regs, tex: vec4<i32>, ras: vec4<i32>, kc: vec3<i32>) -> vec3<i32> {
    switch i {
        case 0u: { return r.r0.rgb; }
        case 1u: { return vec3<i32>(r.r0.a); }
        case 2u: { return r.r1.rgb; }
        case 3u: { return vec3<i32>(r.r1.a); }
        case 4u: { return r.r2.rgb; }
        case 5u: { return vec3<i32>(r.r2.a); }
        case 6u: { return r.r3.rgb; }
        case 7u: { return vec3<i32>(r.r3.a); }
        case 8u: { return tex.rgb; }
        case 9u: { return vec3<i32>(tex.a); }
        case 10u: { return ras.rgb; }
        case 11u: { return vec3<i32>(ras.a); }
        case 12u: { return vec3<i32>(255); }
        case 13u: { return vec3<i32>(128); }
        case 14u: { return kc; }
        default: { return vec3<i32>(0); }
    }
}

fn alpha_in(i: u32, r: Regs, tex: vec4<i32>, ras: vec4<i32>, ka: i32) -> i32 {
    switch i {
        case 0u: { return r.r0.a; }
        case 1u: { return r.r1.a; }
        case 2u: { return r.r2.a; }
        case 3u: { return r.r3.a; }
        case 4u: { return tex.a; }
        case 5u: { return ras.a; }
        case 6u: { return ka; }
        default: { return 0; }
    }
}

fn bias_of(b: u32) -> i32 {
    if (b == 1u) {
        return 128;
    }
    if (b == 2u) {
        return -128;
    }
    return 0;
}

fn scaled(v: vec4<i32>, shift: u32) -> vec4<i32> {
    if (shift == 1u) {
        return v * 2;
    }
    if (shift == 2u) {
        return v * 4;
    }
    if (shift == 3u) {
        return v >> vec4<u32>(1u);
    }
    return v;
}

// d +/- lerp(a, b, c) + bias, scaled; per channel (alpha in .w).
fn combine(a: vec4<i32>, b: vec4<i32>, c: vec4<i32>, d: vec4<i32>, bias: i32, op: u32, shift: u32) -> vec4<i32> {
    let a8 = a & vec4<i32>(255);
    let b8 = b & vec4<i32>(255);
    var c8 = c & vec4<i32>(255);
    c8 = c8 + (c8 >> vec4<u32>(7u));
    var lerp = (a8 << vec4<u32>(8u)) + (b8 - a8) * c8;
    lerp = (lerp + vec4<i32>(select(128, 127, op == 1u))) >> vec4<u32>(8u);
    var r = select(d + lerp, d - lerp, op == 1u);
    return scaled(r + vec4<i32>(bias), shift);
}

fn packed(v: vec3<i32>, n: u32) -> i32 {
    let m = v & vec3<i32>(255);
    if (n == 0u) {
        return m.r;
    }
    if (n == 1u) {
        return (m.g << 8u) | m.r;
    }
    return (m.b << 16u) | (m.g << 8u) | m.r;
}

fn compare(x: i32, y: i32, eq: bool) -> bool {
    return select(x > y, x == y, eq);
}

fn alpha_pass(a: i32, r: i32, f: u32) -> bool {
    switch f {
        case 0u: { return false; }
        case 1u: { return a < r; }
        case 2u: { return a == r; }
        case 3u: { return a <= r; }
        case 4u: { return a > r; }
        case 5u: { return a != r; }
        case 6u: { return a >= r; }
        default: { return true; }
    }
}

// Texture size of map `m` in texels.
fn tex_size(m: u32) -> vec2<f32> {
    let s = word(TEX_SIZE + m);
    return vec2<f32>(f32(max(s & 0xFFFFu, 1u)), f32(max(s >> 16u, 1u)));
}

// Texcoord scale of coordinate `c` (BP 0x30 + 2c).
fn su_size(c: u32) -> vec2<f32> {
    let s = word(SU_SIZE + c);
    return vec2<f32>(f32(s & 0xFFFFu), f32(s >> 16u));
}

fn sample(m: u32, uv: vec2<f32>, dx: vec2<f32>, dy: vec2<f32>) -> vec4<f32> {
    let slot = word(TEX + m);
    if (slot == NONE) {
        return vec4<f32>(1.0);
    }
    return textureSampleGrad(textures[slot], samplers[word(SAMPLER + m)], uv, dx, dy);
}

// 1.11.8 fog float (sign, exponent, 11-bit mantissa) to f32.
fn fog_float(v: u32) -> f32 {
    return bitcast<f32>(((v >> 19u) & 1u) << 31u | ((v >> 11u) & 0xFFu) << 23u | (v & 0x7FFu) << 12u);
}

// A wrapped indirect texcoord (fixed point, 7 fractional bits).
fn ind_wrap(v: i32, w: u32) -> i32 {
    if (w == 0u) {
        return v;
    }
    if (w >= 6u) {
        return 0;
    }
    return v % ((256 >> (w - 1u)) << 7u);
}

@fragment
fn fs(i: VOut) -> FOut {
    return shade(i);
}

@fragment
fn fs_early(i: VOut) -> FOutEarly {
    let s = shade(i);
    var o: FOutEarly;
    o.color = s.color;
    o.blend = s.blend;
    return o;
}

fn shade(i: VOut) -> FOut {
    sidx = i.draw;
    var o: FOut;
    let flags = word(FLAGS);
    if ((flags & FLAG_CLEAR) != 0u) {
        let c = unpack4x8unorm(word(CLEAR_COLOR));
        o.color = c;
        o.blend = c;
        o.depth = i.pos.z;
        return o;
    }
    let uv = Coords(i.t0.xy / i.t0.z, i.t1.xy / i.t1.z, i.t2.xy / i.t2.z, i.t3.xy / i.t3.z, i.t4.xy / i.t4.z, i.t5.xy / i.t5.z, i.t6.xy / i.t6.z, i.t7.xy / i.t7.z);
    // Derivatives in uniform control flow, before the stage loop.
    let dx = Coords(dpdx(uv.c0), dpdx(uv.c1), dpdx(uv.c2), dpdx(uv.c3), dpdx(uv.c4), dpdx(uv.c5), dpdx(uv.c6), dpdx(uv.c7));
    let dy = Coords(dpdy(uv.c0), dpdy(uv.c1), dpdy(uv.c2), dpdy(uv.c3), dpdy(uv.c4), dpdy(uv.c5), dpdy(uv.c6), dpdy(uv.c7));

    let genmode = word(GENMODE);
    let scale = bitcast<f32>(word(SCALE));

    // Indirect lookups: each indirect stage samples its map at its scaled
    // texcoord; the result is (s, t, u) = texel (alpha, blue, green).
    let ind_stages = extractBits(genmode, 16u, 3u);
    var iind = array<vec3<i32>, 4>(vec3<i32>(0), vec3<i32>(0), vec3<i32>(0), vec3<i32>(0));
    for (var n = 0u; n < ind_stages; n = n + 1u) {
        let iref = word(IREF) >> (6u * n);
        let m = iref & 7u;
        let c = (iref >> 3u) & 7u;
        let sc = word(IND_SCALE + n / 2u) >> (8u * (n & 1u));
        let shift = vec2<u32>(sc & 0xFu, (sc >> 4u) & 0xFu);
        let fix = vec2<i32>(coord_of(uv, c) * su_size(c) * 128.0) >> shift;
        let k = su_size(c) / tex_size(m) / vec2<f32>(f32(1u << shift.x), f32(1u << shift.y));
        let t = sample(m, vec2<f32>(fix) / (128.0 * tex_size(m)), coord_of(dx, c) * k, coord_of(dy, c) * k);
        let ti = vec4<i32>(round(t * 255.0));
        iind[n] = vec3<i32>(ti.a, ti.b, ti.g);
    }

    var r = Regs(tev_reg(TEV_COLOR, 0u), tev_reg(TEV_COLOR, 1u), tev_reg(TEV_COLOR, 2u), tev_reg(TEV_COLOR, 3u));
    let k = Regs(tev_reg(TEV_KONST, 0u), tev_reg(TEV_KONST, 1u), tev_reg(TEV_KONST, 2u), tev_reg(TEV_KONST, 3u));
    let ras0 = vec4<i32>(round(clamp(i.c0, vec4<f32>(0.0), vec4<f32>(1.0)) * 255.0));
    let ras1 = vec4<i32>(round(clamp(i.c1, vec4<f32>(0.0), vec4<f32>(1.0)) * 255.0));

    let stages = extractBits(genmode, 10u, 4u) + 1u;
    var cdest = 0u;
    var adest = 0u;
    var tevcoord = vec2<i32>(0);
    var alphabump = 0;
    var last_tex = vec4<i32>(0);
    for (var s = 0u; s < stages; s = s + 1u) {
        let tref = word(TREF + s / 2u) >> (12u * (s & 1u));
        let map = tref & 7u;
        let coord = (tref >> 3u) & 7u;
        let chan = (tref >> 7u) & 7u;
        let cenv = word(ENV + 2u * s);
        let aenv = word(ENV + 2u * s + 1u);

        // Indirect texturing for this stage.
        let ic = word(IND_CMD + s);
        let bt = ic & 3u;
        let ifmt = (ic >> 2u) & 3u;
        let ibias = (ic >> 4u) & 7u;
        let bs = (ic >> 7u) & 3u;
        let mid = (ic >> 9u) & 0xFu;
        let sw = (ic >> 13u) & 7u;
        let tw = (ic >> 16u) & 7u;
        let addprev = ((ic >> 20u) & 1u) == 1u;
        let has_ind = (bs != 0u || mid != 0u) && bt < ind_stages;
        let tuv = coord_of(uv, coord);
        let fix = vec2<i32>(tuv * su_size(coord) * 128.0);
        let fancy = has_ind || sw != 0u || tw != 0u || addprev;
        var indtrans = vec2<i32>(0);
        if (has_ind) {
            var masks = array<i32, 4>(0xFF, 0x1F, 0x0F, 0x07);
            var amasks = array<i32, 4>(0xF8, 0xE0, 0xF0, 0xF8);
            var crd = iind[bt];
            if (bs != 0u) {
                alphabump = crd[bs - 1u] & amasks[ifmt];
            }
            crd = crd & vec3<i32>(masks[ifmt]);
            let b = select(1, -128, ifmt == 0u);
            if ((ibias & 1u) != 0u) { crd.x = crd.x + b; }
            if ((ibias & 2u) != 0u) { crd.y = crd.y + b; }
            if ((ibias & 4u) != 0u) { crd.z = crd.z + b; }
            let mi = (mid & 3u) - 1u;
            if ((mid & 3u) != 0u) {
                let c0 = word(IND_MTX + 3u * mi);
                let c1 = word(IND_MTX + 3u * mi + 1u);
                let c2 = word(IND_MTX + 3u * mi + 2u);
                let mscale = i32(((c0 >> 22u) & 3u) | (((c1 >> 22u) & 3u) << 2u) | (((c2 >> 22u) & 3u) << 4u));
                let shift = 17 - mscale;
                switch mid >> 2u {
                    case 0u: {
                        let row0 = vec3<i32>(s11(c0), s11(c1), s11(c2));
                        let row1 = vec3<i32>(s11(c0 >> 11u), s11(c1 >> 11u), s11(c2 >> 11u));
                        indtrans = vec2<i32>(dot(row0, crd), dot(row1, crd)) >> vec2<u32>(3u);
                    }
                    case 1u: { indtrans = (fix * crd.xx) >> vec2<u32>(8u); }
                    default: { indtrans = (fix * crd.yy) >> vec2<u32>(8u); }
                }
                if (shift >= 0) {
                    indtrans = indtrans >> vec2<u32>(u32(shift));
                } else {
                    indtrans = indtrans << vec2<u32>(u32(-shift));
                }
            }
        }
        if (fancy) {
            let wrapped = vec2<i32>(ind_wrap(fix.x, sw), ind_wrap(fix.y, tw));
            tevcoord = select(vec2<i32>(0), tevcoord, addprev) + wrapped + indtrans;
        } else {
            tevcoord = fix;
        }

        var tex = vec4<i32>(255);
        if (((tref >> 6u) & 1u) == 1u) {
            let ks = su_size(coord) / tex_size(map);
            var st = tuv * ks;
            if (fancy) {
                st = vec2<f32>(tevcoord) / (128.0 * tex_size(map));
            }
            let t = sample(map, st, coord_of(dx, coord) * ks, coord_of(dy, coord) * ks);
            tex = swap(vec4<i32>(round(t * 255.0)), (aenv >> 2u) & 3u);
            last_tex = tex;
        }
        var ras = vec4<i32>(0);
        switch chan {
            case 0u: { ras = ras0; }
            case 1u: { ras = ras1; }
            case 5u: { ras = vec4<i32>(alphabump); }
            case 6u: { ras = vec4<i32>(alphabump | (alphabump >> 5u)); }
            default: {}
        }
        ras = swap(ras, aenv & 3u);

        let ksel = word(KSEL + s / 2u) >> (10u * (s & 1u));
        let kc = konst_color((ksel >> 4u) & 31u, k);
        let ka = konst_alpha((ksel >> 9u) & 31u, k);

        let ca = color_in(extractBits(cenv, 12u, 4u), r, tex, ras, kc);
        let cb = color_in(extractBits(cenv, 8u, 4u), r, tex, ras, kc);
        let cc = color_in(extractBits(cenv, 4u, 4u), r, tex, ras, kc);
        let cd = color_in(extractBits(cenv, 0u, 4u), r, tex, ras, kc);
        let aa = alpha_in(extractBits(aenv, 13u, 3u), r, tex, ras, ka);
        let ab = alpha_in(extractBits(aenv, 10u, 3u), r, tex, ras, ka);
        let ac = alpha_in(extractBits(aenv, 7u, 3u), r, tex, ras, ka);
        let ad = alpha_in(extractBits(aenv, 4u, 3u), r, tex, ras, ka);

        // Colour combiner.
        var cout: vec3<i32>;
        let cbias = extractBits(cenv, 16u, 2u);
        let cop = extractBits(cenv, 18u, 1u);
        let cshift = extractBits(cenv, 20u, 2u);
        if (cbias == 3u) {
            if (cshift == 3u) {
                let a8 = ca & vec3<i32>(255);
                let b8 = cb & vec3<i32>(255);
                let hit = select(a8 > b8, a8 == b8, vec3<bool>(cop == 1u));
                cout = cd + select(vec3<i32>(0), cc & vec3<i32>(255), hit);
            } else {
                let hit = compare(packed(ca, cshift), packed(cb, cshift), cop == 1u);
                cout = cd + select(vec3<i32>(0), cc & vec3<i32>(255), hit);
            }
        } else {
            cout = combine(vec4<i32>(ca, 0), vec4<i32>(cb, 0), vec4<i32>(cc, 0), vec4<i32>(cd, 0), bias_of(cbias), cop, cshift).rgb;
        }
        if (extractBits(cenv, 19u, 1u) == 1u) {
            cout = clamp(cout, vec3<i32>(0), vec3<i32>(255));
        } else {
            cout = clamp(cout, vec3<i32>(-1024), vec3<i32>(1023));
        }

        // Alpha combiner.
        var aout: i32;
        let abias = extractBits(aenv, 16u, 2u);
        let aop = extractBits(aenv, 18u, 1u);
        let ashift = extractBits(aenv, 20u, 2u);
        if (abias == 3u) {
            var hit: bool;
            if (ashift == 3u) {
                hit = compare(aa & 255, ab & 255, aop == 1u);
            } else {
                hit = compare(packed(ca, ashift), packed(cb, ashift), aop == 1u);
            }
            aout = ad + select(0, ac & 255, hit);
        } else {
            aout = combine(vec4<i32>(aa), vec4<i32>(ab), vec4<i32>(ac), vec4<i32>(ad), bias_of(abias), aop, ashift).a;
        }
        if (extractBits(aenv, 19u, 1u) == 1u) {
            aout = clamp(aout, 0, 255);
        } else {
            aout = clamp(aout, -1024, 1023);
        }

        cdest = extractBits(cenv, 22u, 2u);
        adest = extractBits(aenv, 22u, 2u);
        switch cdest {
            case 0u: { r.r0 = vec4<i32>(cout, r.r0.a); }
            case 1u: { r.r1 = vec4<i32>(cout, r.r1.a); }
            case 2u: { r.r2 = vec4<i32>(cout, r.r2.a); }
            default: { r.r3 = vec4<i32>(cout, r.r3.a); }
        }
        switch adest {
            case 0u: { r.r0.a = aout; }
            case 1u: { r.r1.a = aout; }
            case 2u: { r.r2.a = aout; }
            default: { r.r3.a = aout; }
        }
    }

    var rgb = clamp(reg(r, cdest).rgb, vec3<i32>(0), vec3<i32>(255));
    let a = clamp(reg(r, adest).a, 0, 255);

    let at = word(ALPHA_TEST);
    let p0 = alpha_pass(a, i32(at & 255u), extractBits(at, 16u, 3u));
    let p1 = alpha_pass(a, i32((at >> 8u) & 255u), extractBits(at, 19u, 3u));
    var ok: bool;
    switch extractBits(at, 22u, 2u) {
        case 0u: { ok = p0 && p1; }
        case 1u: { ok = p0 || p1; }
        case 2u: { ok = p0 != p1; }
        default: { ok = p0 == p1; }
    }
    if (!ok) {
        discard;
    }

    // Depth: the z-freeze plane, then Z textures, in 24-bit units.
    let px = i.pos.xy / scale;
    var depth = i.pos.z;
    var zcoord = i32(clamp(i.pos.z, 0.0, 1.0) * 16777216.0);
    var zchanged = false;
    if ((flags & FLAG_ZFREEZE) != 0u) {
        let pa = bitcast<f32>(word(ZPLANE));
        let pb = bitcast<f32>(word(ZPLANE + 1u));
        let pc = bitcast<f32>(word(ZPLANE + 2u));
        zcoord = i32(clamp(pa * px.x + pb * px.y + pc, 0.0, 16777215.0));
        zchanged = true;
    }
    let ztex = word(ZTEX + 1u);
    let zop = (ztex >> 2u) & 3u;
    if (zop != 0u) {
        var zt: i32;
        switch ztex & 3u {
            case 0u: { zt = last_tex.a; }
            case 1u: { zt = last_tex.r + (last_tex.a << 8u); }
            default: { zt = (last_tex.r << 16u) + (last_tex.g << 8u) + last_tex.b; }
        }
        zt = zt + i32(word(ZTEX) & 0xFFFFFFu);
        zcoord = select(zt, zcoord + zt, zop == 1u) & 0xFFFFFF;
        zchanged = true;
    }
    if (zchanged) {
        depth = f32(zcoord) / 16777216.0;
    }

    // Fog.
    let f3 = word(FOG + 3u);
    let fsel = (f3 >> 21u) & 7u;
    if (fsel != 0u) {
        let fa = fog_float(word(FOG));
        let fc = fog_float(f3 & 0xFFFFFu);
        var ze: f32;
        if (((f3 >> 20u) & 1u) == 0u) {
            ze = (fa * 16777216.0) / f32(i32(word(FOG + 1u) & 0xFFFFFFu) - (zcoord >> (word(FOG + 2u) & 31u)));
        } else {
            ze = fa * f32(zcoord) / 16777216.0;
        }
        let range = word(FOG_RANGE);
        if (((range >> 10u) & 1u) == 1u) {
            let width = max(bitcast<f32>(word(FOG_WIDTH)), 1.0);
            let offset = (px.x - (f32(range & 0x3FFu) - 342.0)) / width;
            let fi = clamp(9.0 - abs(offset) * 9.0, 0.0, 9.0);
            let lo = u32(fi);
            let hi = min(lo + 1u, 9u);
            let klo = f32((word(FOG_RANGE + 1u + lo / 2u) >> (12u * (lo & 1u))) & 0xFFFu) / 256.0;
            let khi = f32((word(FOG_RANGE + 1u + hi / 2u) >> (12u * (hi & 1u))) & 0xFFFu) / 256.0;
            let kk = max(mix(klo, khi, fract(fi)), 1.0 / 256.0);
            ze = ze * sqrt(offset * offset + kk * kk) / kk;
        }
        var fog = clamp(ze - fc, 0.0, 1.0);
        switch fsel {
            case 4u: { fog = 1.0 - exp2(-8.0 * fog); }
            case 5u: { fog = 1.0 - exp2(-8.0 * fog * fog); }
            case 6u: { fog = exp2(-8.0 * (1.0 - fog)); }
            case 7u: { fog = exp2(-8.0 * (1.0 - fog) * (1.0 - fog)); }
            default: {}
        }
        let ifog = i32(round(fog * 256.0));
        let fcol = word(FOG + 4u);
        let fogc = vec3<i32>(i32((fcol >> 16u) & 255u), i32((fcol >> 8u) & 255u), i32(fcol & 255u));
        rgb = (rgb * (256 - ifog) + fogc * ifog) >> vec3<u32>(8u);
    }

    let colour = vec3<f32>(rgb) / 255.0;
    let fa = f32(a) / 255.0;
    // Destination alpha: the EFB gets the constant alpha, blending still
    // uses the TEV's.
    let ca = word(CONST_ALPHA);
    var written = fa;
    if (((ca >> 8u) & 1u) == 1u) {
        written = f32(ca & 255u) / 255.0;
    }
    if ((word(PE_CONTROL) & 7u) != 1u) {
        written = 1.0;
    }
    o.color = vec4<f32>(colour, written);
    o.blend = vec4<f32>(colour, fa);
    o.depth = depth;
    return o;
}
