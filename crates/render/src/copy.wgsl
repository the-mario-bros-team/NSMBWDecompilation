// EFB copy pass: reads a rectangle of the EFB (colour or depth), applies
// the copy's 2x2 box filter, converts to the copy format the way the
// texture decoder would read the bytes back, and writes one destination
// texel per fragment. Used for texture copies (scaled and native for the
// RAM writeback) and for CPU EFB peeks.

struct Params {
    // Source rectangle in EFB-image pixels: x, y, w, h.
    src: vec4<f32>,
    // Destination size in texels.
    dst: vec2<f32>,
    // Copy format 0..12 (`gx::CopyTex::format`).
    format: u32,
    // 1 intensity, 2 depth source, 4 the EFB has alpha.
    flags: u32,
}

var<immediate> pc: Params;

@group(0) @binding(0) var efb_color: texture_2d<f32>;
@group(0) @binding(1) var efb_depth: texture_depth_2d;

@vertex
fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
}

fn q(v: f32, bits: u32) -> f32 {
    let x = u32(round(clamp(v, 0.0, 1.0) * 255.0)) >> (8u - bits);
    var e: u32;
    switch bits {
        case 3u: { e = (x << 5u) | (x << 2u) | (x >> 1u); }
        case 4u: { e = x * 17u; }
        case 5u: { e = (x << 3u) | (x >> 2u); }
        default: { e = (x << 2u) | (x >> 4u); }
    }
    return f32(e) / 255.0;
}

fn load_color(at: vec2<f32>) -> vec4<f32> {
    let size = vec2<f32>(textureDimensions(efb_color));
    return textureLoad(efb_color, vec2<i32>(clamp(at, vec2<f32>(0.0), size - 1.0)), 0);
}

@fragment
fn fs(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    // The source region this texel covers, sampled at its quarter points:
    // a 2x2 box filter for half-size copies, the nearest texel otherwise.
    let cell = pc.src.zw / pc.dst;
    let base = pc.src.xy + floor(p.xy) * cell;
    var c: vec4<f32>;
    if ((pc.flags & 2u) != 0u) {
        let size = vec2<f32>(textureDimensions(efb_depth));
        let at = vec2<i32>(clamp(base + cell * 0.5, vec2<f32>(0.0), size - 1.0));
        // Depth is stored as z24 / 2^24 throughout.
        let z = min(u32(round(clamp(textureLoad(efb_depth, at, 0), 0.0, 1.0) * 16777216.0)), 0xFFFFFFu);
        let hi = f32((z >> 16u) & 255u) / 255.0;
        let mid = f32((z >> 8u) & 255u) / 255.0;
        let lo = f32(z & 255u) / 255.0;
        switch pc.format {
            case 3u: { return vec4<f32>(mid, mid, mid, hi); }
            case 6u: { return vec4<f32>(hi, mid, lo, 1.0); }
            // RG8 from depth: the higher byte is the alpha, as in Z16
            // (NSMBW's depth of field reads its RG8 copy's alpha as depth).
            case 11u: { return vec4<f32>(mid, mid, mid, hi); }
            default: { c = vec4<f32>(hi, mid, lo, 1.0); }
        }
    } else {
        c = 0.25 * (load_color(base + cell * vec2<f32>(0.25, 0.25)) + load_color(base + cell * vec2<f32>(0.75, 0.25))
            + load_color(base + cell * vec2<f32>(0.25, 0.75)) + load_color(base + cell * vec2<f32>(0.75, 0.75)));
        if ((pc.flags & 4u) == 0u) {
            c.a = 1.0;
        }
        let f = pc.format;
        if ((pc.flags & 1u) != 0u && (f <= 3u || f == 8u)) {
            c.r = clamp(dot(c.rgb, vec3<f32>(0.257, 0.504, 0.098)) + 16.0 / 255.0, 0.0, 1.0);
        }
    }
    switch pc.format {
        case 0u: { let v = q(c.r, 4u); return vec4<f32>(v, v, v, v); }
        case 1u, 8u: { return c.rrrr; }
        case 2u: { let v = q(c.r, 4u); return vec4<f32>(v, v, v, q(c.a, 4u)); }
        case 3u: { return vec4<f32>(c.rrr, c.a); }
        case 4u: { return vec4<f32>(q(c.r, 5u), q(c.g, 6u), q(c.b, 5u), 1.0); }
        case 5u: {
            if (c.a >= 224.0 / 255.0) {
                return vec4<f32>(q(c.r, 5u), q(c.g, 5u), q(c.b, 5u), 1.0);
            }
            return vec4<f32>(q(c.r, 4u), q(c.g, 4u), q(c.b, 4u), q(c.a, 3u));
        }
        case 7u: { return c.aaaa; }
        case 9u: { return c.gggg; }
        case 10u: { return c.bbbb; }
        case 11u: { return vec4<f32>(c.rrr, c.g); }
        case 12u: { return vec4<f32>(c.ggg, c.b); }
        default: { return c; }
    }
}
