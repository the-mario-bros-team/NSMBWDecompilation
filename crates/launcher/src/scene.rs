//! The picture behind the launcher, drawn by code from a version's theme
//! (`versions.toml`, `theme.scene`): a sky, a sun or moon, stars, an aurora,
//! clouds, lines of hills or peaks drifting at different speeds, and motes
//! of light. Nothing here is taken from any game.
//!
//! Two renderers share one description: [`paint`] draws it live with egui
//! shapes (the background), and [`render`] draws one still frame into an
//! image (the cover art on cards, which need rounded corners).

use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{pos2, vec2, Color32, ColorImage, Painter, Pos2, Rect, Shape, TextureId};

use crate::versions::{Ridge, SceneSpec};

fn hash(n: u32) -> f32 {
    let mut x = n.wrapping_mul(0x9E37_79B9) ^ 0x85EB_CA6B;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    x = x.wrapping_mul(0x297A_2D39);
    x ^= x >> 15;
    (x >> 8) as f32 / (1u32 << 24) as f32
}

/// Value noise in 0..1; `smooth` rounds the corners between its points.
fn noise(x: f32, seed: u32, smooth: bool) -> f32 {
    let i = x.floor();
    let f = x - i;
    let t = if smooth { f * f * (3.0 - 2.0 * f) } else { f };
    let at = |k: f32| hash((k as i64 as u32).wrapping_mul(7919).wrapping_add(seed.wrapping_mul(104_729)));
    at(i) + (at(i + 1.0) - at(i)) * t
}

/// How far up the picture (0..1 from the bottom) ridge `index` reaches at
/// `x` (in picture heights).
fn ridge_top(r: &Ridge, index: usize, x: f32) -> f32 {
    let seed = index as u32 * 31 + 5;
    let u = x / r.width.max(0.05);
    let rolling = 0.62 * noise(u * 1.3, seed, true) + 0.28 * noise(u * 2.9, seed + 1, true) + 0.10 * noise(u * 6.1, seed + 2, true);
    let jagged = 0.52 * noise(u * 1.6, seed + 3, false) + 0.27 * noise(u * 3.7, seed + 4, false) + 0.14 * noise(u * 8.3, seed + 5, false) + 0.07 * noise(u * 17.0, seed + 6, false);
    let n = rolling + (jagged - rolling) * r.rough.clamp(0.0, 1.0);
    r.height + r.amp * (n * 2.0 - 1.0)
}

pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let l = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t).round() as u8;
    Color32::from_rgba_premultiplied(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()), l(a.a(), b.a()))
}

/// The sky's colour at `v` (0 = the top of the picture, 1 = the bottom).
pub fn sky_at(spec: &SceneSpec, v: f32) -> Color32 {
    const MID: f32 = 0.40;
    const HORIZON: f32 = 0.72;
    if v < MID {
        mix(spec.sky[0].0, spec.sky[1].0, v / MID)
    } else {
        mix(spec.sky[1].0, spec.sky[2].0, (v - MID) / (HORIZON - MID))
    }
}

struct Puff {
    /// Centre, x in picture heights and y in 0..1 from the top; radius in
    /// picture heights.
    x: f32,
    y: f32,
    r: f32,
    color: Color32,
}

/// The clouds' circles at `time`, far ones first.
fn clouds(spec: &SceneSpec, time: f32, aspect: f32) -> Vec<Puff> {
    const SHAPE: [(f32, f32, f32); 6] = [(-1.5, 0.30, 0.62), (-0.75, -0.05, 0.92), (0.1, -0.38, 1.12), (0.95, -0.02, 0.88), (1.65, 0.32, 0.60), (0.1, 0.34, 0.74)];
    let count = (spec.clouds.amount * 7.0).round() as u32;
    let mut order: Vec<(f32, u32)> = (0..count).map(|i| (hash(i * 11 + 3), i)).collect();
    order.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut puffs = Vec::new();
    for (near, i) in order {
        let size = 0.020 + 0.040 * near;
        let margin = 0.4;
        let span = aspect + 2.0 * margin;
        let x = ((hash(i * 11 + 1) + time * (0.003 + 0.010 * near) / span).fract()) * span - margin;
        let y = 0.07 + 0.36 * hash(i * 11 + 2);
        let color = mix(sky_at(spec, y), spec.clouds.color.0, 0.55 + 0.45 * near);
        for (dx, dy, r) in SHAPE {
            puffs.push(Puff { x: x + dx * size, y: y + dy * size, r: r * size, color });
        }
    }
    puffs
}

/// A star: x and y in 0..1 of the picture, size in 0..1, twinkle phase.
fn star(i: u32) -> (f32, f32, f32, f32) {
    (hash(i * 5 + 101), hash(i * 5 + 102) * 0.66, hash(i * 5 + 103), hash(i * 5 + 104) * 6.283)
}

fn star_count(spec: &SceneSpec) -> u32 {
    (spec.stars.clamp(0.0, 3.0) * 150.0) as u32
}

/// An aurora curtain at `x` (picture heights): its top and height (0..1 of
/// the picture) and its strength (0..1).
fn curtain(band: usize, x: f32, time: f32) -> (f32, f32, f32) {
    let b = band as f32;
    let top = 0.06 + 0.055 * b + 0.045 * (x * 1.7 + time * 0.21 + b * 1.9).sin() + 0.025 * (x * 4.3 - time * 0.33 + b).sin();
    let height = 0.26 + 0.07 * (x * 2.3 + time * 0.27 + b * 2.4).sin();
    let strength = 0.30 + 0.22 * (x * 3.1 + time * 0.43 + b * 1.3).sin() + 0.10 * (x * 7.7 - time * 0.6).sin();
    (top, height, strength.clamp(0.04, 0.7))
}

/// How bright the sun or moon's wide and near halos are.
const HALO: [f32; 2] = [0.26, 0.30];

fn additive(c: Color32, strength: f32) -> Color32 {
    let s = |v: u8| (f32::from(v) * strength.clamp(0.0, 1.0)) as u8;
    Color32::from_rgba_premultiplied(s(c.r()), s(c.g()), s(c.b()), 0)
}

fn quad(mesh: &mut Mesh, center: Pos2, half: f32, color: Color32) {
    mesh.add_rect_with_uv(Rect::from_center_size(center, vec2(half * 2.0, half * 2.0)), Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), color);
}

/// Draws the scene live into `rect` (clipped by the painter). `opacity`
/// fades the whole picture; `shift` slides the layers sideways, near ones
/// further (parallax); `soft` is a round, soft-edged white texture.
pub fn paint(spec: &SceneSpec, p: &Painter, rect: Rect, time: f32, opacity: f32, shift: f32, soft: TextureId) {
    if opacity <= 0.003 {
        return;
    }
    let (w, h) = (rect.width(), rect.height());
    let aspect = w / h;
    let fade = |c: Color32| c.gamma_multiply(opacity);
    let at = |x: f32, v: f32| pos2(rect.left() + x * h, rect.top() + v * h);

    // Sky.
    let mut sky = Mesh::default();
    for v in [0.0, 0.40, 0.72, 1.0] {
        let c = fade(sky_at(spec, v));
        sky.colored_vertex(pos2(rect.left(), rect.top() + v * h), c);
        sky.colored_vertex(pos2(rect.right(), rect.top() + v * h), c);
    }
    for i in 0..3 {
        sky.add_triangle(i * 2, i * 2 + 1, i * 2 + 2);
        sky.add_triangle(i * 2 + 1, i * 2 + 3, i * 2 + 2);
    }
    p.add(Shape::mesh(sky));

    // Stars.
    let mut lights = Mesh::with_texture(soft);
    for i in 0..star_count(spec) {
        let (x, y, size, phase) = star(i);
        let twinkle = 0.55 + 0.45 * (time * (0.6 + size * 1.7) + phase).sin();
        quad(&mut lights, at(x * aspect, y), 1.0 + 2.2 * size, additive(Color32::WHITE, opacity * twinkle * (0.45 + 0.55 * size)));
    }

    // Aurora.
    if !spec.aurora.is_empty() {
        let mut m = Mesh::default();
        let step = 0.03;
        let columns = (aspect / step).ceil() as u32 + 1;
        for (band, color) in spec.aurora.iter().enumerate() {
            let base = m.vertices.len() as u32;
            for k in 0..columns {
                let x = k as f32 * step;
                let (top, height, strength) = curtain(band, x + shift * 0.05, time);
                m.colored_vertex(at(x, top), Color32::TRANSPARENT);
                m.colored_vertex(at(x, top + height * 0.72), additive(color.0, strength * opacity));
                m.colored_vertex(at(x, top + height), Color32::TRANSPARENT);
            }
            for k in 0..columns - 1 {
                let a = base + k * 3;
                for row in 0..2 {
                    m.add_triangle(a + row, a + row + 3, a + row + 1);
                    m.add_triangle(a + row + 1, a + row + 3, a + row + 4);
                }
            }
        }
        p.add(Shape::mesh(m));
    }

    // The sun or moon: a halo, then the body.
    let g = &spec.glow;
    let center = at(g.x * aspect + shift * 0.02, g.y);
    quad(&mut lights, center, g.size * h, additive(g.color.0, HALO[0] * opacity));
    quad(&mut lights, center, g.size * h * 0.42, additive(g.color.0, HALO[1] * opacity));
    p.add(Shape::mesh(lights));
    if g.disc > 0.0 {
        p.circle_filled(center, g.disc * h, fade(mix(g.color.0, Color32::WHITE, 0.45)));
    }

    // Clouds.
    for puff in clouds(spec, time, aspect) {
        p.circle_filled(at(puff.x + shift * 0.04, puff.y), puff.r * h, fade(puff.color));
    }

    // Ridges, far to near.
    let step = (6.0 / h).max(0.004);
    let columns = (aspect / step).ceil() as u32 + 1;
    let feather = 1.2;
    let count = spec.ridge.len().max(1) as f32;
    for (index, r) in spec.ridge.iter().enumerate() {
        let depth = (index as f32 + 1.0) / count;
        let scroll = time * r.speed + shift * 0.12 * depth * depth;
        let (top, bottom) = (fade(r.top.0), fade(r.bottom.0));
        let mut m = Mesh::default();
        for k in 0..columns {
            let x = k as f32 * step;
            let y = rect.bottom() - ridge_top(r, index, x + scroll) * h;
            let px = rect.left() + x * h;
            m.vertices.push(Vertex { pos: pos2(px, y - feather), uv: WHITE_UV, color: Color32::TRANSPARENT });
            m.vertices.push(Vertex { pos: pos2(px, y), uv: WHITE_UV, color: top });
            m.vertices.push(Vertex { pos: pos2(px, rect.bottom()), uv: WHITE_UV, color: bottom });
        }
        for k in 0..columns - 1 {
            let a = k * 3;
            for row in 0..2 {
                m.add_triangle(a + row, a + row + 3, a + row + 1);
                m.add_triangle(a + row + 1, a + row + 3, a + row + 4);
            }
        }
        p.add(Shape::mesh(m));
    }

    // Motes of light.
    let mut motes = Mesh::with_texture(soft);
    for i in 0..(spec.particles.amount.clamp(0.0, 4.0) * 46.0) as u32 {
        let (hx, hy, hs) = (hash(i * 3 + 201), hash(i * 3 + 202), hash(i * 3 + 203));
        let travel = time * (0.010 + 0.028 * hs);
        let y = if spec.particles.fall { (hy + travel).fract() } else { (hy - travel).rem_euclid(1.0) };
        let x = (hx + 0.03 * (time * 0.4 + hs * 40.0).sin()).rem_euclid(1.0);
        let glow = 0.30 + 0.30 * (time * (0.8 + hs) + hx * 30.0).sin();
        quad(&mut motes, pos2(rect.left() + x * w + shift * 30.0 * hs, rect.top() + y * h), 2.0 + 4.5 * hs, additive(spec.particles.color.0, glow * opacity));
    }
    p.add(Shape::mesh(motes));
}

/// One still frame of the scene as an image, `w` by `h` pixels.
pub fn render(spec: &SceneSpec, w: usize, h: usize) -> ColorImage {
    let (wf, hf) = (w as f32, h as f32);
    let aspect = wf / hf;
    let time = 40.0;
    let rgb = |c: Color32| [f32::from(c.r()), f32::from(c.g()), f32::from(c.b())];
    let mut px = vec![[0.0f32; 3]; w * h];

    // Sky, aurora, and the sun or moon's halo.
    let g = &spec.glow;
    let (gx, gy, glow) = (g.x * aspect * hf, g.y * hf, rgb(g.color.0));
    let curtains: Vec<Vec<(f32, f32, f32)>> = (0..spec.aurora.len()).map(|band| (0..w).map(|x| curtain(band, x as f32 / hf, time)).collect()).collect();
    for y in 0..h {
        let v = y as f32 / hf;
        let sky = rgb(sky_at(spec, v));
        for x in 0..w {
            let mut c = sky;
            for (band, color) in spec.aurora.iter().enumerate() {
                let (top, height, strength) = curtains[band][x];
                let t = (v - top) / height;
                if (0.0..1.0).contains(&t) {
                    let k = strength * if t < 0.72 { t / 0.72 } else { (1.0 - t) / 0.28 };
                    let a = rgb(color.0);
                    for i in 0..3 {
                        c[i] += a[i] * k;
                    }
                }
            }
            let d = ((x as f32 - gx).powi(2) + (y as f32 - gy).powi(2)).sqrt() / hf;
            for (radius, strength) in [(g.size, HALO[0]), (g.size * 0.42, HALO[1])] {
                let f = (1.0 - (d / radius).powi(2)).max(0.0).powi(2) * strength;
                for i in 0..3 {
                    c[i] += glow[i] * f;
                }
            }
            px[y * w + x] = c;
        }
    }

    // Filled circles with soft edges, for the stars, the body and the clouds.
    let mut circle = |cx: f32, cy: f32, r: f32, color: [f32; 3], strength: f32, add: bool| {
        let (x0, x1) = (((cx - r - 1.0).floor().max(0.0)) as usize, ((cx + r + 1.0).ceil().min(wf - 1.0)).max(0.0) as usize);
        let (y0, y1) = (((cy - r - 1.0).floor().max(0.0)) as usize, ((cy + r + 1.0).ceil().min(hf - 1.0)).max(0.0) as usize);
        for y in y0..=y1.min(h.saturating_sub(1)) {
            for x in x0..=x1.min(w.saturating_sub(1)) {
                let d = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                let cover = (r - d + 0.5).clamp(0.0, 1.0) * strength;
                let p = &mut px[y * w + x];
                for i in 0..3 {
                    p[i] = if add { p[i] + color[i] * cover } else { p[i] + (color[i] - p[i]) * cover };
                }
            }
        }
    };
    for i in 0..star_count(spec) {
        let (x, y, size, phase) = star(i);
        let twinkle = 0.55 + 0.45 * (time * (0.6 + size * 1.7) + phase).sin();
        circle(x * wf, y * hf, 0.4 + 0.9 * size, [255.0; 3], twinkle * (0.45 + 0.55 * size), true);
    }
    if g.disc > 0.0 {
        circle(gx, gy, g.disc * hf, rgb(mix(g.color.0, Color32::WHITE, 0.45)), 1.0, false);
    }
    for puff in clouds(spec, time, aspect) {
        circle(puff.x * hf, puff.y * hf, puff.r * hf, rgb(puff.color), 1.0, false);
    }

    // Ridges.
    for (index, r) in spec.ridge.iter().enumerate() {
        let (top, bottom) = (rgb(r.top.0), rgb(r.bottom.0));
        for x in 0..w {
            let edge = hf - ridge_top(r, index, x as f32 / hf + time * r.speed) * hf;
            for y in (edge.floor().max(0.0) as usize)..h {
                let cover = (y as f32 - edge + 0.5).clamp(0.0, 1.0);
                let t = ((y as f32 - edge) / (hf - edge).max(1.0)).clamp(0.0, 1.0);
                let p = &mut px[y * w + x];
                for i in 0..3 {
                    let c = top[i] + (bottom[i] - top[i]) * t;
                    p[i] += (c - p[i]) * cover;
                }
            }
        }
    }

    let pixels = px.iter().map(|c| Color32::from_rgb(c[0].min(255.0) as u8, c[1].min(255.0) as u8, c[2].min(255.0) as u8)).collect();
    ColorImage::new([w, h], pixels)
}

/// A round white texture, opaque in the middle and fading to nothing at the
/// edge: halos, stars and motes are drawn with it.
pub fn soft_dot() -> ColorImage {
    const N: usize = 64;
    let mut pixels = Vec::with_capacity(N * N);
    for y in 0..N {
        for x in 0..N {
            let d = (vec2(x as f32 + 0.5, y as f32 + 0.5) - vec2(N as f32, N as f32) / 2.0).length() / (N as f32 / 2.0);
            let a = ((1.0 - d * d).max(0.0).powi(2) * 255.0) as u8;
            pixels.push(Color32::from_rgba_premultiplied(a, a, a, a));
        }
    }
    ColorImage::new([N, N], pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ridges_stay_within_their_band() {
        let spec = &crate::versions::load(std::path::Path::new("no such folder")).0[1].theme.scene;
        for (i, r) in spec.ridge.iter().enumerate() {
            for k in 0..2000 {
                let top = ridge_top(r, i, k as f32 * 0.013 - 7.0);
                assert!((r.height - r.amp - 1e-4..=r.height + r.amp + 1e-4).contains(&top), "ridge {i} at {k}: {top}");
            }
        }
    }

    #[test]
    fn a_still_frame_is_drawn() {
        for v in crate::versions::load(std::path::Path::new("no such folder")).0 {
            let image = render(&v.theme.scene, 64, 48);
            assert_eq!(image.size, [64, 48]);
            // The sky at the top, the nearest ridge at the bottom.
            assert_ne!(image.pixels[0], image.pixels[64 * 47]);
        }
    }
}
