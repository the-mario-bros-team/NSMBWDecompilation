//! Frame interpolation, after the design in WitherBlitz's
//! NSMB-Wii-Recompilation (aurora `lib/gx/frame_interpolation.cpp`): each
//! draw of a frame is paired with the same draw of the frame before (same
//! identity, nearest transform), and its vertices are placed again with the
//! position matrices interpolated rigidly (rotation by quaternion, scale and
//! translation linearly) and the projection blended. Draws without a
//! partner, lines and points keep their positions; colours and texture
//! coordinates always come from the newer frame.

use std::collections::HashMap;

use crate::xf::{apply, project};
use crate::{Output, Vertex};

type Mat = [[f32; 4]; 3];

/// A paired draw may move at most this far on screen (in NDC units, 2 =
/// the full width) between frames; farther means a cut or a teleport.
const MAX_SCREEN_STEP: f32 = 0.5;
/// Groups of identical draws larger than this pair in order instead of by
/// distance.
const MAX_GROUP: usize = 256;
/// Draws of a group that come in the same order and each move less than
/// this (NDC units) are paired in order, without searching.
const IN_ORDER_STEP: f32 = 0.05;

/// For each record of `cur`, the record of `prev` it continues, if any.
pub fn pair(prev: &Output, cur: &Output) -> Vec<Option<u32>> {
    // Records by identity: sorted (identity, index) lists, walked together.
    // The world map has over 10,000 small draws a field.
    let keyed = |out: &Output| {
        let mut v: Vec<(u64, u32)> = out.records.iter().enumerate().filter(|(_, r)| !r.fixed).map(|(i, r)| (r.identity, i as u32)).collect();
        v.sort_unstable();
        v
    };
    let (kp, kc) = (keyed(prev), keyed(cur));
    // Each record's first vertex on screen, once per frame.
    let (sp, sc) = (screen_points(prev), screen_points(cur));
    let step = |p: usize, c: usize| step(prev, p, &sp, cur, c, &sc);
    let mut pairs = vec![None; cur.records.len()];
    let mut used = vec![false; prev.records.len()];
    let (mut prevs, mut curs, mut edges) = (Vec::new(), Vec::new(), Vec::new());
    let (mut i, mut j) = (0, 0);
    while i < kp.len() && j < kc.len() {
        let id = kp[i].0.min(kc[j].0);
        prevs.clear();
        while i < kp.len() && kp[i].0 == id {
            prevs.push(kp[i].1 as usize);
            i += 1;
        }
        curs.clear();
        while j < kc.len() && kc[j].0 == id {
            curs.push(kc[j].1 as usize);
            j += 1;
        }
        if prevs.is_empty() || curs.is_empty() {
            continue;
        }
        // Usually a group's draws come in the same order and barely move:
        // pair them in order when every such pair stays this close.
        if prevs.len() == curs.len() && curs.iter().zip(&prevs).all(|(&c, &p)| step(p, c).is_some_and(|d| d <= IN_ORDER_STEP * IN_ORDER_STEP)) {
            for (&c, &p) in curs.iter().zip(&prevs) {
                pairs[c] = Some(p as u32);
            }
            continue;
        }
        if prevs.len() > MAX_GROUP || curs.len() > MAX_GROUP {
            for (&c, &p) in curs.iter().zip(&prevs) {
                if step(p, c).is_some() {
                    pairs[c] = Some(p as u32);
                }
            }
            continue;
        }
        edges.clear();
        for &c in &curs {
            for &p in &prevs {
                if let Some(d) = step(p, c) {
                    edges.push((d, c, p));
                }
            }
        }
        edges.sort_by(|a, b| a.0.total_cmp(&b.0));
        for &(_, c, p) in &edges {
            if pairs[c].is_none() && !used[p] {
                pairs[c] = Some(p as u32);
                used[p] = true;
            }
        }
    }
    pairs
}

/// Each record's first vertex projected with its own matrix: (x/w, y/w,
/// w > 0), or `None` when it cannot be placed.
fn screen_points(out: &Output) -> Vec<Option<(f32, f32, bool)>> {
    out.records
        .iter()
        .map(|r| {
            let (pos, slot) = *out.model.get(r.first as usize)?;
            let m = &r.matrices.iter().find(|m| m.0 == slot)?.1;
            let a = project(apply(m, pos), &r.projection, r.ortho, &r.viewport, r.screen);
            (a[3].abs() >= 1e-9).then(|| (a[0] / a[3], a[1] / a[3], a[3] > 0.0))
        })
        .collect()
}

/// How far (squared, in NDC) record `c`'s first vertex moved since record
/// `p`, or `None` if they cannot be the same draw.
fn step(prev: &Output, p: usize, sp: &[Option<(f32, f32, bool)>], cur: &Output, c: usize, sc: &[Option<(f32, f32, bool)>]) -> Option<f32> {
    let (rp, rc) = (&prev.records[p], &cur.records[c]);
    if rp.ortho != rc.ortho || rp.count != rc.count || rp.matrices.len() != rc.matrices.len() {
        return None;
    }
    let ((ax, ay, aw), (bx, by, bw)) = (sp[p]?, sc[c]?);
    if aw != bw {
        return None;
    }
    let (dx, dy) = (ax - bx, ay - by);
    let d = dx * dx + dy * dy;
    (d.is_finite() && d <= MAX_SCREEN_STEP * MAX_SCREEN_STEP).then_some(d)
}

/// `cur`'s vertices with every paired draw placed at fraction `t` (0 =
/// `prev`, 1 = `cur`) of the way between the two frames.
pub fn interpolate(prev: &Output, cur: &Output, pairs: &[Option<u32>], t: f32, out: &mut Vec<Vertex>) {
    out.clear();
    out.resize(cur.vertices.len(), Vertex::default());
    interpolate_into(prev, cur, pairs, t, out);
}

/// [`interpolate`] into `out` (exactly `cur.vertices.len()` long), which is
/// only written, so it may be mapped GPU memory.
pub fn interpolate_into(prev: &Output, cur: &Output, pairs: &[Option<u32>], t: f32, out: &mut [Vertex]) {
    out.copy_from_slice(&cur.vertices);
    let lerp6 = |a: &[f32; 6], b: &[f32; 6]| -> [f32; 6] { std::array::from_fn(|i| a[i] + (b[i] - a[i]) * t) };
    let mut matrices: Vec<(u8, Mat)> = Vec::new();
    for (rc, pair) in cur.records.iter().zip(pairs) {
        let Some(p) = pair else { continue };
        let rp = &prev.records[*p as usize];
        matrices.clear();
        matrices.extend(rc.matrices.iter().filter_map(|(slot, mc)| {
            let mp = &rp.matrices.iter().find(|m| m.0 == *slot)?.1;
            Some((*slot, interpolate_matrix(mp, mc, t)))
        }));
        let projection = lerp6(&rp.projection, &rc.projection);
        let viewport = lerp6(&rp.viewport, &rc.viewport);
        let range = rc.first as usize..(rc.first + rc.count) as usize;
        for (v, &(pos, slot)) in out[range.clone()].iter_mut().zip(&cur.model[range]) {
            if let Some((_, m)) = matrices.iter().find(|m| m.0 == slot) {
                v.pos = project(apply(m, pos), &projection, rc.ortho, &viewport, rc.screen);
            }
        }
    }
}

/// No matrix: the vertex keeps `cur`'s position.
pub const NO_MOVE: u32 = u32::MAX;

/// What the GPU needs to place `cur`'s paired draws anywhere between `prev`
/// and `cur` ([`interpolate_into`] done by the vertex shader, so frames
/// shown in between cost no CPU work per vertex). Built once per pair of
/// fields; the shader reads it with the fraction `t` alone.
#[derive(Default)]
pub struct Moves {
    /// Per vertex of `cur`: model-space position, and in `w` the bits of an
    /// index into `mats` ([`NO_MOVE`]: keep the vertex as it is).
    pub model: Vec<[f32; 4]>,
    /// Per matrix pair, ten vec4s: rows of the two matrices (a0..a2,
    /// b0..b2), both rotations as quaternions (qb on qa's side), and the
    /// scales (sa.w: 1 for a rigid blend, 0 for a linear one; sb.w: the bits
    /// of the index into `recs`).
    pub mats: Vec<[[f32; 4]; 10]>,
    /// Per paired draw, seven vec4s: both projections, both viewports (six
    /// floats each, back to back), then screen offset x, y and ortho (1/0).
    pub recs: Vec<[[f32; 4]; 7]>,
}

/// [`Moves`] for `cur` after `prev`, with `pairs` from [`pair`].
///
/// A draw whose matrices, projection and viewport did not change is left
/// out: its vertices are already where every in-between frame puts them.
/// Draws share a few cameras and many share matrices, so each distinct
/// camera pair and matrix pair is built once.
pub fn moves(prev: &Output, cur: &Output, pairs: &[Option<u32>]) -> Moves {
    let mut m = Moves { model: Vec::with_capacity(cur.vertices.len()), ..Default::default() };
    let none = f32::from_bits(NO_MOVE);
    m.model.extend(cur.model.iter().map(|&(p, _)| [p[0], p[1], p[2], none]));
    // Records cover the vertices; anything else stays put.
    m.model.resize(cur.vertices.len(), [0.0, 0.0, 0.0, none]);
    let mut rec_ids: HashMap<[u32; 27], u32, Fx> = HashMap::default();
    let mut mat_ids: HashMap<([u32; 12], [u32; 12], u32), u32, Fx> = HashMap::default();
    let bits = |m: &Mat| -> [u32; 12] { std::array::from_fn(|i| m[i / 4][i % 4].to_bits()) };
    let mut matched: Vec<(u8, &Mat, &Mat)> = Vec::new();
    let mut slots: Vec<(u8, u32)> = Vec::new();
    for (rc, pair) in cur.records.iter().zip(pairs) {
        let Some(p) = pair else { continue };
        let rp = &prev.records[*p as usize];
        matched.clear();
        let mut moving = rp.projection != rc.projection || rp.viewport != rc.viewport;
        for (slot, mc) in &rc.matrices {
            let Some((_, mp)) = rp.matrices.iter().find(|m| m.0 == *slot) else { continue };
            moving |= mp != mc;
            matched.push((*slot, mp, mc));
        }
        if !moving || matched.is_empty() {
            continue;
        }
        let (pa, pb, va, vb) = (&rp.projection, &rc.projection, &rp.viewport, &rc.viewport);
        let ortho = if rc.ortho { 1.0 } else { 0.0 };
        let entry = [
            [pa[0], pa[1], pa[2], pa[3]],
            [pa[4], pa[5], pb[0], pb[1]],
            [pb[2], pb[3], pb[4], pb[5]],
            [va[0], va[1], va[2], va[3]],
            [va[4], va[5], vb[0], vb[1]],
            [vb[2], vb[3], vb[4], vb[5]],
            [rc.screen.offset_x, rc.screen.offset_y, ortho, 0.0],
        ];
        let key: [u32; 27] = std::array::from_fn(|i| entry[i / 4][i % 4].to_bits());
        let rec = *rec_ids.entry(key).or_insert_with(|| {
            m.recs.push(entry);
            m.recs.len() as u32 - 1
        });
        slots.clear();
        for &(slot, mp, mc) in &matched {
            let k = *mat_ids.entry((bits(mp), bits(mc), rec)).or_insert_with(|| {
                m.mats.push(matrix_pair(mp, mc, rec));
                m.mats.len() as u32 - 1
            });
            slots.push((slot, k));
        }
        let range = rc.first as usize..(rc.first + rc.count) as usize;
        for (v, &(_, slot)) in m.model[range.clone()].iter_mut().zip(&cur.model[range]) {
            if let Some(&(_, k)) = slots.iter().find(|s| s.0 == slot) {
                v[3] = f32::from_bits(k);
            }
        }
    }
    m
}

/// A small fast hasher for the keys above (bits of floats; no untrusted input).
type Fx = std::hash::BuildHasherDefault<FxHasher>;

#[derive(Default)]
struct FxHasher(u64);

impl std::hash::Hasher for FxHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for c in bytes.chunks(8) {
            let mut b = [0u8; 8];
            b[..c.len()].copy_from_slice(c);
            self.0 = (self.0.rotate_left(5) ^ u64::from_le_bytes(b)).wrapping_mul(0x517C_C1B7_2722_0A95);
        }
    }
}

/// One entry of [`Moves::mats`]: [`interpolate_matrix`] split into what the
/// shader blends.
fn matrix_pair(a: &Mat, b: &Mat, rec: u32) -> [[f32; 4]; 10] {
    let mut e = [[0.0; 4]; 10];
    e[..3].copy_from_slice(a);
    e[3..6].copy_from_slice(b);
    e[9][3] = f32::from_bits(rec);
    if a == b {
        return e;
    }
    if let (Some((ra, sa)), Some((rb, sb))) = (decompose(a), decompose(b)) {
        let (qa, mut qb) = (quat(&ra), quat(&rb));
        if dot4(qa, qb) < 0.0 {
            qb = qb.map(|v| -v);
        }
        e[6] = qa;
        e[7] = qb;
        e[8] = [sa[0], sa[1], sa[2], 1.0];
        e[9][..3].copy_from_slice(&sb);
    }
    e
}

/// Rigid interpolation of two affine transforms: the 3x3 part as rotation
/// times per-column scale. Shears and degenerate matrices blend linearly.
pub fn interpolate_matrix(a: &Mat, b: &Mat, t: f32) -> Mat {
    let linear = || -> Mat { std::array::from_fn(|r| std::array::from_fn(|c| a[r][c] + (b[r][c] - a[r][c]) * t)) };
    if a == b {
        return *a;
    }
    let (Some((ra, sa)), Some((rb, sb))) = (decompose(a), decompose(b)) else {
        return linear();
    };
    let (qa, mut qb) = (quat(&ra), quat(&rb));
    if dot4(qa, qb) < 0.0 {
        qb = qb.map(|v| -v);
    }
    let q = normalize4(std::array::from_fn(|i| qa[i] + (qb[i] - qa[i]) * t));
    let r = rotation(q);
    let s: [f32; 3] = std::array::from_fn(|i| sa[i] + (sb[i] - sa[i]) * t);
    std::array::from_fn(|row| std::array::from_fn(|col| if col < 3 { r[row][col] * s[col] } else { a[row][3] + (b[row][3] - a[row][3]) * t }))
}

/// Rotation and per-column scale of `m`'s 3x3 part, if it is close to one.
fn decompose(m: &Mat) -> Option<([[f32; 3]; 3], [f32; 3])> {
    let mut s = [0.0f32; 3];
    let mut r = [[0.0f32; 3]; 3];
    for c in 0..3 {
        let len = (m[0][c] * m[0][c] + m[1][c] * m[1][c] + m[2][c] * m[2][c]).sqrt();
        if !(len > 1e-8 && len.is_finite()) {
            return None;
        }
        s[c] = len;
        for row in 0..3 {
            r[row][c] = m[row][c] / len;
        }
    }
    let det = r[0][0] * (r[1][1] * r[2][2] - r[1][2] * r[2][1]) - r[0][1] * (r[1][0] * r[2][2] - r[1][2] * r[2][0]) + r[0][2] * (r[1][0] * r[2][1] - r[1][1] * r[2][0]);
    if det < 0.0 {
        s[0] = -s[0];
        for row in r.iter_mut() {
            row[0] = -row[0];
        }
    }
    // Columns must be near-orthogonal for this to be a rotation.
    for (i, j) in [(0, 1), (0, 2), (1, 2)] {
        if (r[0][i] * r[0][j] + r[1][i] * r[1][j] + r[2][i] * r[2][j]).abs() > 0.02 {
            return None;
        }
    }
    Some((r, s))
}

fn dot4(a: [f32; 4], b: [f32; 4]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

fn normalize4(q: [f32; 4]) -> [f32; 4] {
    let l = dot4(q, q).sqrt();
    if l > 0.0 {
        q.map(|v| v / l)
    } else {
        [0.0, 0.0, 0.0, 1.0]
    }
}

/// Quaternion (x, y, z, w) of a rotation matrix.
fn quat(m: &[[f32; 3]; 3]) -> [f32; 4] {
    let tr = m[0][0] + m[1][1] + m[2][2];
    let q = if tr > 0.0 {
        let s = (tr + 1.0).sqrt() * 2.0;
        [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s]
    } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
        let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
        [0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    } else if m[1][1] > m[2][2] {
        let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
        [(m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    } else {
        let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
        [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s]
    };
    normalize4(q)
}

fn rotation(q: [f32; 4]) -> [[f32; 3]; 3] {
    let [x, y, z, w] = q;
    [
        [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y - z * w), 2.0 * (x * z + y * w)],
        [2.0 * (x * y + z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z - x * w)],
        [2.0 * (x * z - y * w), 2.0 * (y * z + x * w), 1.0 - 2.0 * (x * x + y * y)],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xf::Screen;
    use crate::DrawRecord;

    fn rot_z(deg: f32, tx: f32, scale: f32) -> Mat {
        let (s, c) = deg.to_radians().sin_cos();
        [[c * scale, -s * scale, 0.0, tx], [s * scale, c * scale, 0.0, 0.0], [0.0, 0.0, scale, -10.0]]
    }

    #[test]
    fn rotation_stays_rigid() {
        let m = interpolate_matrix(&rot_z(0.0, 0.0, 2.0), &rot_z(90.0, 4.0, 2.0), 0.5);
        let want = rot_z(45.0, 2.0, 2.0);
        for r in 0..3 {
            for c in 0..4 {
                assert!((m[r][c] - want[r][c]).abs() < 1e-5, "{m:?} vs {want:?}");
            }
        }
    }

    fn frame(m: Mat) -> Output {
        let rec = DrawRecord {
            first: 0,
            count: 3,
            identity: 7,
            projection: [1.0, 0.0, 1.0, 0.0, -1.0, -1.0],
            ortho: false,
            viewport: [320.0, -240.0, 16_777_215.0, 662.0, 582.0, 16_777_215.0],
            screen: Screen { offset_x: 342.0, offset_y: 342.0 },
            matrices: vec![(0, m)],
            fixed: false,
        };
        let model = vec![([0.0, 0.0, 0.0], 0), ([1.0, 0.0, 0.0], 0), ([0.0, 1.0, 0.0], 0)];
        let vertices = model.iter().map(|(p, _)| Vertex { pos: project(apply(&m, *p), &rec.projection, false, &rec.viewport, rec.screen), ..Default::default() }).collect();
        Output { vertices, model, records: vec![rec], ..Default::default() }
    }

    #[test]
    fn paired_draws_move_halfway() {
        let (a, b) = (frame(rot_z(0.0, 0.0, 1.0)), frame(rot_z(0.0, 1.0, 1.0)));
        let pairs = pair(&a, &b);
        assert_eq!(pairs, vec![Some(0)]);
        let mut out = Vec::new();
        interpolate(&a, &b, &pairs, 0.5, &mut out);
        let mid = frame(rot_z(0.0, 0.5, 1.0));
        for (v, w) in out.iter().zip(&mid.vertices) {
            for k in 0..4 {
                assert!((v.pos[k] - w.pos[k]).abs() < 1e-5);
            }
        }
        interpolate(&a, &b, &pairs, 1.0, &mut out);
        assert_eq!(out, b.vertices);
    }

    /// `moved` in vk-render's shader.wgsl, line for line.
    fn shader(m: &Moves, vertex: usize, cur: &Vertex, t: f32) -> [f32; 4] {
        let mix = |a: f32, b: f32| a + (b - a) * t;
        let mix4 = |a: [f32; 4], b: [f32; 4]| -> [f32; 4] { std::array::from_fn(|i| mix(a[i], b[i])) };
        let [px, py, pz, kw] = m.model[vertex];
        let k = kw.to_bits();
        if k == NO_MOVE {
            return cur.pos;
        }
        let e = &m.mats[k as usize];
        let mut r = [mix4(e[0], e[3]), mix4(e[1], e[4]), mix4(e[2], e[5])];
        if e[8][3] != 0.0 {
            let q = normalize4(mix4(e[6], e[7]));
            let s: [f32; 3] = std::array::from_fn(|i| mix(e[8][i], e[9][i]));
            let rot = rotation(q);
            for (row, rr) in r.iter_mut().enumerate() {
                *rr = [rot[row][0] * s[0], rot[row][1] * s[1], rot[row][2] * s[2], rr[3]];
            }
        }
        let world = std::array::from_fn(|i| r[i][0] * px + r[i][1] * py + r[i][2] * pz + r[i][3]);
        let rec = &m.recs[e[9][3].to_bits() as usize];
        let pa = [rec[0][0], rec[0][1], rec[0][2], rec[0][3], rec[1][0], rec[1][1]];
        let pb = [rec[1][2], rec[1][3], rec[2][0], rec[2][1], rec[2][2], rec[2][3]];
        let va = [rec[3][0], rec[3][1], rec[3][2], rec[3][3], rec[4][0], rec[4][1]];
        let vb = [rec[4][2], rec[4][3], rec[5][0], rec[5][1], rec[5][2], rec[5][3]];
        let pr = std::array::from_fn(|i| mix(pa[i], pb[i]));
        let vp = std::array::from_fn(|i| mix(va[i], vb[i]));
        project(world, &pr, rec[6][2] != 0.0, &vp, Screen { offset_x: rec[6][0], offset_y: rec[6][1] })
    }

    #[test]
    fn gpu_moves_match_the_cpu() {
        for (ma, mb) in [(rot_z(0.0, 0.0, 1.0), rot_z(0.0, 1.0, 1.0)), (rot_z(10.0, 0.0, 2.0), rot_z(50.0, 0.4, 2.5)), (rot_z(5.0, 0.0, 1.0), rot_z(5.0, 0.0, 1.0))] {
            let (a, b) = (frame(ma), frame(mb));
            let pairs = pair(&a, &b);
            let m = moves(&a, &b, &pairs);
            for t in [0.0, 0.3, 0.5, 1.0] {
                let mut want = Vec::new();
                interpolate(&a, &b, &pairs, t, &mut want);
                for (i, w) in want.iter().enumerate() {
                    let got = shader(&m, i, &b.vertices[i], t);
                    for k in 0..4 {
                        assert!((got[k] - w.pos[k]).abs() < 1e-4, "t {t} vertex {i}: {got:?} vs {:?}", w.pos);
                    }
                }
            }
        }
    }

    #[test]
    fn far_jumps_do_not_pair() {
        let (a, b) = (frame(rot_z(0.0, 0.0, 1.0)), frame(rot_z(0.0, 9.0, 1.0)));
        assert_eq!(pair(&a, &b), vec![None]);
    }
}
