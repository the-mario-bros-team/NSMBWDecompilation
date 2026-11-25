// GPU vertex processing (vertex pulling): appended to the ubershader by
// renderers that decode and transform vertices themselves
// (`gx::Output::pull`). `vs_pulled` reads each vertex in its GX format from
// the draw's bytes and arrays as the game wrote them (`gx::pull`: a format
// word per attribute; `read_pulled` is the CPU's reading), then does what
// `gx::xf` does on the CPU (`transform_one`, `light_channel`): position and
// normal matrices, projection and viewport, the two lighting channels and
// texture coordinate generation, from the XF state blocks
// (`gx::xf::block`; the offsets below mirror it).
// `gx::pull::transform_pulled` is the CPU reference.

// The field's pulled bytes: vertices and array copies (a byte buffer).
@group(0) @binding(6) var<storage, read> pull: array<u32>;
// XF state blocks.
@group(0) @binding(7) var<storage, read> xfb: array<u32>;
// Per state: setup block offset, matrix block offset, spare, flags (1: the
// matrix block holds all of matrix memory).
@group(0) @binding(8) var<storage, read> pstates: array<vec4<u32>>;
// Per GX draw (`gx::pull::DRAW_WORDS` words): where its vertices start in
// `pull`, its format, its state, spare, per array the offset of element 0,
// the arrays' strides (a byte each).
@group(0) @binding(9) var<storage, read> pdraws: array<u32>;
// Vertex formats (`gx::pull::FORMAT_WORDS` words): size and attribute
// count, then a word per attribute.
@group(0) @binding(10) var<storage, read> pformats: array<u32>;

const DRAW_WORDS: u32 = 20u;
const DRAW_ARRAYS: u32 = 4u;
const DRAW_STRIDES: u32 = 16u;
const FORMAT_WORDS: u32 = 22u;

fn byte(a: u32) -> u32 {
    return (pull[a >> 2u] >> ((a & 3u) * 8u)) & 0xFFu;
}

fn be16(a: u32) -> u32 {
    return byte(a) << 8u | byte(a + 1u);
}

fn be32(a: u32) -> u32 {
    return byte(a) << 24u | byte(a + 1u) << 16u | byte(a + 2u) << 8u | byte(a + 3u);
}

// Component `i` at `a` in format `fmt` (u8, s8, u16, s16, f32), unscaled.
fn comp(a: u32, fmt: u32, i: u32) -> f32 {
    switch fmt {
        case 0u: {
            return f32(byte(a + i));
        }
        case 1u: {
            return f32(bitcast<i32>(byte(a + i) << 24u) >> 24u);
        }
        case 2u: {
            return f32(be16(a + 2u * i));
        }
        case 3u: {
            return f32(bitcast<i32>(be16(a + 2u * i) << 16u) >> 16u);
        }
        case 4u: {
            return bitcast<f32>(be32(a + 4u * i));
        }
        default: {
            return 0.0;
        }
    }
}

// A colour at `a` in format `fmt` (RGB565, RGB888, RGB888x, RGBA4444,
// RGBA6, RGBA8888), as `gx::vertex::color`.
fn gx_color(a: u32, fmt: u32) -> vec4<f32> {
    var c = vec4<u32>(0u);
    switch fmt {
        case 0u: {
            let v = be16(a);
            c = vec4<u32>(((v >> 11u) & 31u) * 255u / 31u, ((v >> 5u) & 63u) * 255u / 63u, (v & 31u) * 255u / 31u, 255u);
        }
        case 1u, 2u: {
            c = vec4<u32>(byte(a), byte(a + 1u), byte(a + 2u), 255u);
        }
        case 3u: {
            let v = be16(a);
            c = vec4<u32>(((v >> 12u) & 15u) * 17u, ((v >> 8u) & 15u) * 17u, ((v >> 4u) & 15u) * 17u, (v & 15u) * 17u);
        }
        case 4u: {
            let v = byte(a) << 16u | byte(a + 1u) << 8u | byte(a + 2u);
            c = vec4<u32>(((v >> 18u) & 63u) * 255u / 63u, ((v >> 12u) & 63u) * 255u / 63u, ((v >> 6u) & 63u) * 255u / 63u, (v & 63u) * 255u / 63u);
        }
        case 5u: {
            c = vec4<u32>(byte(a), byte(a + 1u), byte(a + 2u), byte(a + 3u));
        }
        default: {}
    }
    return vec4<f32>(c) / 255.0;
}

// Setup block (`gx::xf::block`).
const B_PROJECTION: u32 = 0u;
const B_ORTHO: u32 = 6u;
const B_VIEWPORT: u32 = 7u;
const B_CHANS: u32 = 13u;
const B_CTL: u32 = 14u;
const B_MAT: u32 = 18u;
const B_AMB: u32 = 26u;
const B_GENS: u32 = 34u;
const B_DUAL: u32 = 35u;
const B_NEED_NORMAL: u32 = 36u;
const B_NEED_NRM_MTX: u32 = 37u;
const B_GEN_INFO: u32 = 38u;
const B_POST: u32 = 46u;
const B_POST_NORM: u32 = 142u;
const B_LIGHTS: u32 = 143u;
// Matrix block.
const B_SCREEN: u32 = 0u;
const B_POS: u32 = 2u;
const B_NRM: u32 = 14u;
const B_TEX: u32 = 23u;
const B_MEM: u32 = 2u;
const B_NRM_MEM: u32 = 258u;
const B_DEFAULTS: u32 = 362u;

fn xf1(i: u32) -> f32 {
    return bitcast<f32>(xfb[i]);
}

fn xf3(i: u32) -> vec3<f32> {
    return vec3<f32>(xf1(i), xf1(i + 1u), xf1(i + 2u));
}

fn xf4(i: u32) -> vec4<f32> {
    return vec4<f32>(xf1(i), xf1(i + 1u), xf1(i + 2u), xf1(i + 3u));
}

// `gx::xf::normalize`: unchanged when its length is zero.
fn norm3(v: vec3<f32>) -> vec3<f32> {
    let l = sqrt(dot(v, v));
    if l > 0.0 {
        return v / l;
    }
    return v;
}

// A matrix block's matrices for one vertex.
struct Mats {
    // Block offset; whether it holds all of matrix memory.
    mb: u32,
    full: bool,
    pnmtx: u32,
}

// Row `r` of the matrix at row index `idx` in a full block's memory.
fn mem_row(mb: u32, idx: u32, r: u32) -> vec4<f32> {
    let at = (idx * 4u + r * 4u) & 0xFFu;
    return vec4<f32>(xf1(mb + B_MEM + at), xf1(mb + B_MEM + ((at + 1u) & 0xFFu)), xf1(mb + B_MEM + ((at + 2u) & 0xFFu)), xf1(mb + B_MEM + ((at + 3u) & 0xFFu)));
}

fn pos_row(m: Mats, r: u32) -> vec4<f32> {
    if m.full {
        return mem_row(m.mb, m.pnmtx, r);
    }
    return xf4(m.mb + B_POS + 4u * r);
}

fn nrm_row(m: Mats, r: u32) -> vec3<f32> {
    if m.full {
        return xf3(m.mb + B_NRM_MEM + (m.pnmtx & 31u) * 3u + 3u * r);
    }
    return xf3(m.mb + B_NRM + 3u * r);
}

// `light_channel`: channel `c`'s colour or alpha under control word `ctl`.
fn light_channel(sb: u32, ctl: u32, vtx: vec4<f32>, c: u32, pos: vec3<f32>, normal: vec3<f32>) -> vec4<f32> {
    var mat = xf4(sb + B_MAT + 4u * c);
    if (ctl & 1u) != 0u {
        mat = vtx;
    }
    if (ctl & 2u) == 0u {
        return mat;
    }
    var amb = xf4(sb + B_AMB + 4u * c);
    if (ctl & 0x40u) != 0u {
        amb = vtx;
    }
    let diffuse_fn = (ctl >> 7u) & 3u;
    let spot = ((ctl >> 9u) & 1u) != 0u;
    let attn_on = ((ctl >> 10u) & 1u) != 0u;
    let mask = ((ctl >> 2u) & 0xFu) | (((ctl >> 11u) & 0xFu) << 4u);
    var sum = amb;
    for (var l = 0u; l < 8u; l++) {
        if (mask & (1u << l)) == 0u {
            continue;
        }
        let lb = sb + B_LIGHTS + 16u * l;
        let lc = xf4(lb);
        let cos_a = xf3(lb + 4u);
        let dist_k = xf3(lb + 7u);
        let lpos = xf3(lb + 10u);
        let ldir = xf3(lb + 13u);
        var dir: vec3<f32>;
        var attn: f32;
        if !attn_on {
            dir = norm3(lpos - pos);
            attn = 1.0;
        } else if spot {
            let dv = lpos - pos;
            let dist2 = dot(dv, dv);
            let dist = sqrt(dist2);
            dir = dv;
            if dist > 0.0 {
                dir = dv / dist;
            }
            let cs = max(dot(dir, ldir), 0.0);
            let num = max(cos_a.x + cos_a.y * cs + cos_a.z * cs * cs, 0.0);
            let den = dist_k.x + dist_k.y * dist + dist_k.z * dist2;
            attn = 0.0;
            if den != 0.0 {
                attn = num / den;
            }
        } else {
            // Specular: the light "position" holds its direction and
            // "direction" the half-angle vector.
            dir = norm3(lpos);
            var s = 0.0;
            if dot(normal, dir) >= 0.0 {
                s = max(dot(normal, ldir), 0.0);
            }
            let num = max(cos_a.x + cos_a.y * s + cos_a.z * s * s, 0.0);
            let den = dist_k.x + dist_k.y * s + dist_k.z * s * s;
            attn = 0.0;
            if den != 0.0 {
                attn = num / den;
            }
        }
        let nd = dot(normal, dir);
        var diffuse = 1.0;
        if diffuse_fn == 1u {
            diffuse = nd;
        } else if diffuse_fn >= 2u {
            diffuse = max(nd, 0.0);
        }
        sum += attn * diffuse * lc;
    }
    return mat * clamp(sum, vec4<f32>(0.0), vec4<f32>(1.0));
}

@vertex
fn vs_pulled(@builtin(vertex_index) vi: u32, @builtin(instance_index) inst: u32) -> VOut {
    // The index names the GX draw (high bits) and its vertex.
    let db = (vi >> 16u) * DRAW_WORDS;
    let k = vi & 0xFFFFu;
    let st = pstates[pdraws[db + 2u]];
    let sb = st.x;
    var m = Mats(st.y, (st.w & 1u) != 0u, 0u);
    let fb = pdraws[db + 1u] * FORMAT_WORDS;
    let head = pformats[fb];
    let at = pdraws[db] + k * (head & 0xFFu);

    // The vertex, over the defaults: zero, colours white, the default
    // matrices (all of matrix memory) or the draw's own.
    var texmtx = array<u32, 8>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
    if m.full {
        let d0 = xfb[m.mb + B_DEFAULTS];
        let d1 = xfb[m.mb + B_DEFAULTS + 1u];
        m.pnmtx = d0 & 255u;
        texmtx = array<u32, 8>((d0 >> 8u) & 255u, (d0 >> 16u) & 255u, d0 >> 24u, d1 & 255u, (d1 >> 8u) & 255u, (d1 >> 16u) & 255u, d1 >> 24u, xfb[m.mb + B_DEFAULTS + 2u] & 255u);
    }
    var p = vec3<f32>(0.0);
    var nbt = array<f32, 9>(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    var vc0 = vec4<f32>(1.0);
    var vc1 = vec4<f32>(1.0);
    var has0 = false;
    var has1 = false;
    var vt = array<vec2<f32>, 8>(vec2<f32>(0.0), vec2<f32>(0.0), vec2<f32>(0.0), vec2<f32>(0.0), vec2<f32>(0.0), vec2<f32>(0.0), vec2<f32>(0.0), vec2<f32>(0.0));
    let steps = (head >> 8u) & 0xFFu;
    for (var s = 0u; s < steps; s++) {
        let w = pformats[fb + 1u + s];
        let dst = w & 7u;
        let idx = (w >> 3u) & 7u;
        let off = (w >> 6u) & 0xFFu;
        let desc = (w >> 14u) & 3u;
        let arr = (w >> 16u) & 15u;
        let fmt = (w >> 20u) & 7u;
        let count = (w >> 23u) & 15u;
        // 2^-shift.
        let scale = bitcast<f32>((127u - (w >> 27u)) << 23u);
        var src = at + off;
        if desc >= 2u {
            var i = byte(at + off);
            if desc == 3u {
                i = be16(at + off);
            }
            let stride = (pdraws[db + DRAW_STRIDES + arr / 4u] >> (8u * (arr % 4u))) & 0xFFu;
            src = pdraws[db + DRAW_ARRAYS + arr] + i * stride;
        }
        switch dst {
            case 0u: {
                m.pnmtx = byte(src) & 0x3Fu;
            }
            case 1u: {
                texmtx[idx] = byte(src) & 0x3Fu;
            }
            case 2u: {
                for (var c = 0u; c < min(count, 3u); c++) {
                    p[c] = comp(src, fmt, c) * scale;
                }
            }
            case 3u: {
                for (var c = 0u; c < min(count, 9u); c++) {
                    nbt[c] = comp(src, fmt, c) * scale;
                }
            }
            case 4u: {
                if idx == 0u {
                    vc0 = gx_color(src, fmt);
                    has0 = true;
                } else {
                    vc1 = gx_color(src, fmt);
                    has1 = true;
                }
            }
            default: {
                var t = vec2<f32>(0.0);
                for (var c = 0u; c < min(count, 2u); c++) {
                    t[c] = comp(src, fmt, c) * scale;
                }
                vt[idx] = t;
            }
        }
    }
    let n0 = vec3<f32>(nbt[0], nbt[1], nbt[2]);
    let n1 = vec3<f32>(nbt[3], nbt[4], nbt[5]);
    let n2 = vec3<f32>(nbt[6], nbt[7], nbt[8]);

    // Position, normal and projection.
    let r0 = pos_row(m, 0u);
    let r1 = pos_row(m, 1u);
    let r2 = pos_row(m, 2u);
    let world = vec3<f32>(dot(r0.xyz, p) + r0.w, dot(r1.xyz, p) + r1.w, dot(r2.xyz, p) + r2.w);
    let need_nrm_mtx = xfb[sb + B_NEED_NRM_MTX] != 0u;
    var nm0 = vec3<f32>(0.0);
    var nm1 = vec3<f32>(0.0);
    var nm2 = vec3<f32>(0.0);
    if need_nrm_mtx {
        nm0 = nrm_row(m, 0u);
        nm1 = nrm_row(m, 1u);
        nm2 = nrm_row(m, 2u);
    }
    var normal = vec3<f32>(0.0);
    if xfb[sb + B_NEED_NORMAL] != 0u {
        normal = norm3(vec3<f32>(dot(nm0, n0), dot(nm1, n0), dot(nm2, n0)));
    }
    let mb = m.mb;
    let screen = vec2<f32>(xf1(mb + B_SCREEN), xf1(mb + B_SCREEN + 1u));
    // `project` takes (wd, ht, zRange, farZ) and (xOrig, yOrig).
    let vb = sb + B_VIEWPORT;
    var o: VOut;
    o.pos = project(world, xf4(sb + B_PROJECTION), vec2<f32>(xf1(sb + B_PROJECTION + 4u), xf1(sb + B_PROJECTION + 5u)), xfb[sb + B_ORTHO] != 0u, vec4<f32>(xf1(vb), xf1(vb + 1u), xf1(vb + 2u), xf1(vb + 5u)), vec2<f32>(xf1(vb + 3u), xf1(vb + 4u)), screen);

    // Lighting channels.
    let chans = xfb[sb + B_CHANS];
    var color = array<vec4<f32>, 2>(vec4<f32>(0.0), vec4<f32>(0.0));
    for (var c = 0u; c < 2u; c++) {
        if c >= chans {
            break;
        }
        var vc = vec4<f32>(1.0);
        if c == 0u && has0 {
            vc = vc0;
        }
        if c == 1u && has1 {
            vc = vc1;
        }
        let cc = xfb[sb + B_CTL + 2u * c];
        let ac = xfb[sb + B_CTL + 2u * c + 1u];
        let rgb = light_channel(sb, cc, vc, c, world, normal);
        var a = rgb;
        if ac != cc {
            a = light_channel(sb, ac, vc, c, world, normal);
        }
        color[c] = vec4<f32>(rgb.xyz, a.w);
    }

    // Texture coordinate generation.
    let gens = xfb[sb + B_GENS];
    let dual = xfb[sb + B_DUAL] != 0u;
    let post_norm = xfb[sb + B_POST_NORM];
    var tc = array<vec3<f32>, 8>(vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));
    for (var i = 0u; i < 8u; i++) {
        if i >= gens {
            break;
        }
        let info = xfb[sb + B_GEN_INFO + i];
        let stq = ((info >> 1u) & 1u) != 0u;
        let ab11 = ((info >> 2u) & 1u) == 0u;
        let kind = (info >> 4u) & 7u;
        let src_row = (info >> 7u) & 31u;
        var src = vec4<f32>(0.0, 0.0, 1.0, 1.0);
        if src_row == 0u {
            src = vec4<f32>(p, 1.0);
        } else if src_row == 1u {
            src = vec4<f32>(n0, 1.0);
        } else if src_row == 2u {
            src = vec4<f32>(color[0].xyz, 1.0);
        } else if src_row == 3u {
            src = vec4<f32>(n1, 1.0);
        } else if src_row == 4u {
            src = vec4<f32>(n2, 1.0);
        } else if src_row <= 12u {
            let t = vt[src_row - 5u];
            src = vec4<f32>(t, 1.0, 1.0);
        }
        if ab11 {
            src.z = 1.0;
        }
        var c: vec3<f32>;
        if kind == 2u {
            c = vec3<f32>(color[0].x, color[0].y, 1.0);
        } else if kind == 3u {
            c = vec3<f32>(color[1].x, color[1].y, 1.0);
        } else if kind == 1u {
            // Emboss: the source texgen's coordinates moved by the light
            // direction in the surface's tangent space.
            let s = (info >> 12u) & 7u;
            let light = (info >> 15u) & 7u;
            var base = vec3<f32>(0.0, 0.0, 1.0);
            if s < i {
                base = tc[s];
            }
            let ldir = norm3(xf3(sb + B_LIGHTS + 16u * light + 10u) - world);
            let tangent = norm3(vec3<f32>(dot(nm0, n1), dot(nm1, n1), dot(nm2, n1)));
            let binormal = norm3(vec3<f32>(dot(nm0, n2), dot(nm1, n2), dot(nm2, n2)));
            c = vec3<f32>(base.x + dot(ldir, tangent), base.y + dot(ldir, binormal), base.z);
        } else {
            var t0: vec4<f32>;
            var t1: vec4<f32>;
            var t2: vec4<f32>;
            if m.full {
                t0 = mem_row(mb, texmtx[i], 0u);
                t1 = mem_row(mb, texmtx[i], 1u);
                t2 = mem_row(mb, texmtx[i], 2u);
            } else {
                t0 = xf4(mb + B_TEX + 12u * i);
                t1 = xf4(mb + B_TEX + 12u * i + 4u);
                t2 = xf4(mb + B_TEX + 12u * i + 8u);
            }
            c = vec3<f32>(dot(t0, src), dot(t1, src), 1.0);
            if stq {
                c.z = dot(t2, src);
            }
        }
        if dual && kind == 0u {
            if ((post_norm >> i) & 1u) != 0u {
                c = norm3(c);
            }
            let pb = sb + B_POST + 12u * i;
            let q0 = xf4(pb);
            let q1 = xf4(pb + 4u);
            let q2 = xf4(pb + 8u);
            var z = 1.0;
            if stq {
                z = dot(q2.xyz, c) + q2.w;
            }
            c = vec3<f32>(dot(q0.xyz, c) + q0.w, dot(q1.xyz, c) + q1.w, z);
        }
        if c.z == 0.0 {
            c.z = 1.0;
        }
        tc[i] = c;
    }

    o.c0 = color[0];
    o.c1 = color[1];
    o.t0 = tc[0];
    o.t1 = tc[1];
    o.t2 = tc[2];
    o.t3 = tc[3];
    o.t4 = tc[4];
    o.t5 = tc[5];
    o.t6 = tc[6];
    o.t7 = tc[7];
    o.draw = inst;
    return o;
}
