//! Background layers in players' own windows (docs/PORT_PLAN.md, section
//! 12). The game places each background layer for its camera. With a box
//! around far-apart players it does so for the box: scaled up with the
//! box and moved for the box's centre and height. A player's window cut
//! from the box would show the layers enlarged and out of place.
//!
//! Each layer (named by its view-space depth) is learned from the fields
//! the game draws, on the host only:
//! - its scale is linear in the camera's width (1-1's far hills: 0.502 +
//!   0.00122 x width, so 1.107 at the normal 496);
//! - divided by the zoom (scale / normal scale), its x translation is a
//!   constant minus a share of the camera's x (13.8% for those hills): its
//!   parallax;
//! - divided by the zoom, its y translation moves with the camera's height,
//!   at a rate learned per height from consecutive fields (none for layers
//!   fixed in the world); it is summed from the box's height down to the
//!   normal height.
//!
//! In a player's window a layer is drawn at its normal scale, at the height
//! it has with a normal camera, and moved by its parallax share of the
//! window's offset from the box's centre.

use std::collections::{BTreeMap, HashMap};

use crate::{xf, Layer, World};

#[derive(Clone, Debug, Default)]
struct Learned {
    /// Least-squares sums for scale = a + b x width: n, Σw, Σs, Σw², Σws.
    fit: [f64; 5],
    /// The share of the camera's x motion the layer follows in view space.
    parallax: Option<f32>,
    /// How the pieces' y translation over the zoom changes per unit of
    /// camera height, by height (in steps of [`STEP`]): recent estimates,
    /// whose median is used.
    slopes: BTreeMap<i32, Vec<f32>>,
}

/// Height step for [`Learned::slopes`].
const STEP: f32 = 16.0;

impl Learned {
    fn add(&mut self, w: f32, s: f32) {
        let (w, s) = (f64::from(w), f64::from(s));
        let f = &mut self.fit;
        *f = [f[0] + 1.0, f[1] + w, f[2] + s, f[3] + w * w, f[4] + w * s];
    }

    /// The scale at camera width `w`, once two widths have been seen.
    fn scale_at(&self, w: f32) -> Option<f32> {
        let [n, sw, ss, sww, sws] = self.fit;
        let var = n * sww - sw * sw;
        if n < 2.0 || var <= 1e-3 * n * n {
            return None;
        }
        let b = (n * sws - sw * ss) / var;
        let a = (ss - b * sw) / n;
        Some((a + b * f64::from(w)) as f32).filter(|s| *s > 0.0)
    }

    /// How much the pieces' y over the zoom changes from camera height
    /// `from` to `to`: the learned rates summed in steps (each step takes
    /// the nearest height learned).
    fn y_change(&self, from: f32, to: f32) -> f32 {
        if self.slopes.is_empty() || (to - from).abs() < 1e-3 {
            return 0.0;
        }
        let steps = ((to - from).abs() / STEP).ceil().max(1.0);
        let dh = (to - from) / steps;
        (0..steps as i32)
            .map(|i| {
                let h = from + dh * (i as f32 + 0.5);
                let key = (h / STEP).round() as i32;
                let rate = self.slopes.range(..=key).next_back().map(|(k, v)| (key - k, v)).into_iter().chain(self.slopes.range(key..).next().map(|(k, v)| (k - key, v))).min_by_key(|(d, _)| *d).and_then(|(_, v)| median(v.clone())).unwrap_or(0.0);
                rate * dh
            })
            .sum()
    }
}

/// What is known about the background layers, kept by each window's owner.
#[derive(Clone, Default)]
pub struct Backdrop {
    layers: HashMap<i32, Learned>,
    /// The last field: its camera centre and size, and each layer's
    /// pieces' translations over the zoom, in draw order.
    prev: Option<([f32; 2], [f32; 2], HashMap<i32, Vec<[f32; 2]>>)>,
    /// The field heights are measured from: a reference kept until the
    /// height has moved [`STEP`] from it (or the camera moved vertically).
    href: Option<([f32; 2], [f32; 2], HashMap<i32, Vec<[f32; 2]>>)>,
}

fn median(mut v: Vec<f32>) -> Option<f32> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f32::total_cmp);
    Some(v[v.len() / 2])
}

impl Backdrop {
    /// Learns from a field (call once per field, before drawing it).
    pub fn learn(&mut self, world: &World) {
        if world.layers.is_empty() {
            self.prev = None;
            self.href = None;
            return;
        }
        let cam = world.camera;
        let mut by_depth: HashMap<i32, Vec<&Layer>> = HashMap::new();
        for l in &world.layers {
            by_depth.entry(l.z).or_default().push(l);
        }
        let mut norm: HashMap<i32, Vec<[f32; 2]>> = HashMap::new();
        for (z, pieces) in &by_depth {
            let learned = self.layers.entry(*z).or_default();
            learned.add(cam.size[0], pieces[0].scale);
            // Over the zoom: as if drawn with the normal camera size.
            let normal = learned.scale_at(cam.normal_size[0]).or(cam.normal.then_some(pieces[0].scale));
            let Some(zoom) = normal.map(|n| pieces[0].scale / n).filter(|z| *z > 1e-3) else { continue };
            norm.insert(*z, pieces.iter().map(|l| [l.t[0] / zoom, l.t[1] / zoom]).collect());
        }
        if let Some((was, size, before)) = &self.prev {
            let moved = cam.centre[0] - was[0];
            if moved.abs() >= 0.5 {
                for (z, ts) in &norm {
                    let Some(old) = before.get(z) else { continue };
                    // The k-th piece of a layer is the same from field to field.
                    let est = median(ts.iter().zip(old).map(|(t, o)| -(t[0] - o[0]) / moved).collect());
                    if let (Some(est), Some(l)) = (est, self.layers.get_mut(z)) {
                        let est = est.clamp(-0.5, 1.5);
                        l.parallax = Some(l.parallax.map_or(est, |p| 0.8 * p + 0.2 * est));
                    }
                }
            }
        }
        // Height: from the reference field, if the camera has not moved
        // vertically since, once the height has changed by a step.
        match &self.href {
            Some((was, size, before)) if (cam.centre[1] - was[1]).abs() < 0.25 => {
                let grew = cam.size[1] - size[1];
                if grew.abs() >= STEP {
                    let key = (((cam.size[1] + size[1]) / 2.0) / STEP).round() as i32;
                    for (z, ts) in &norm {
                        let Some(old) = before.get(z).filter(|o| o.len() == ts.len()) else { continue };
                        let est = median(ts.iter().zip(old).map(|(t, o)| (t[1] - o[1]) / grew).collect());
                        if let (Some(est), Some(l)) = (est, self.layers.get_mut(z)) {
                            let v = l.slopes.entry(key).or_default();
                            v.push(est.clamp(-0.6, 0.6));
                            if v.len() > 9 {
                                v.remove(0);
                            }
                        }
                    }
                    self.href = Some((cam.centre, cam.size, norm.clone()));
                }
            }
            _ => self.href = Some((cam.centre, cam.size, norm.clone())),
        }
        self.prev = Some((cam.centre, cam.size, norm));
    }

    /// The clip-space affine (per axis, X' = a X + b W) that draws layer `l`
    /// of the field `world` in `view`. `None` if the
    /// layer's normal scale is not known yet.
    pub fn layer_affine(&self, world: &World, l: &Layer, view: &xf::View) -> Option<[(f32, f32); 2]> {
        let learned = self.layers.get(&l.z)?;
        let cam = world.camera;
        let normal = learned.scale_at(view.size[0]).or(cam.normal.then_some(l.scale))?;
        if l.scale <= 1e-6 {
            return None;
        }
        let ratio = normal / l.scale;
        let share = learned.parallax.unwrap_or(0.0);
        // Fixed in the world and never rescaled (the castle at the start of
        // 1-1): drawn exactly like the course.
        if (share - 1.0).abs() < 0.05 && (ratio - 1.0).abs() < 0.01 {
            return Some(world.world_affine(view));
        }
        // Where it sits at the window's (normal) height rather than the
        // camera's, over the zoom.
        let dy = learned.y_change(cam.size[1], view.size[1]);
        let shift = [-share * view.offset[0], dy - share * view.offset[1]];
        Some(std::array::from_fn(|i| {
            let (s, o) = (world.map[2 * i], world.map[2 * i + 1]);
            let a = ratio * cam.size[i] / view.size[i];
            (a, o * (1.0 - a) + s * 2.0 * shift[i] / view.size[i])
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WorldCamera, LAYER_Z};

    /// A field with one layer piece drawn the way 1-1's far hills are: scale
    /// 0.502 + 0.00122 w, x over the zoom 72.66 - 0.138 cx, and y over the
    /// zoom 0.2 h - 50.
    fn field(w: f32, cx: f32) -> World {
        let h = w * 0.548;
        let normal = 0.502 + 0.00122 * 496.0;
        let s = 0.502 + 0.00122 * w;
        let zoom = s / normal;
        let t = [zoom * (72.66 - 0.138 * cx), zoom * (0.2 * h - 50.0)];
        let camera = WorldCamera { size: [w, h], centre: [cx, -505.0], normal: w == 496.0, normal_size: [496.0, 496.0 * 0.548] };
        World { ranges: vec![(0, 4)], layers: vec![Layer { first: 0, count: 4, z: LAYER_Z as i32 - 6000, scale: s, t }], camera, map: [1.0, 0.0, 1.0, 0.0] }
    }

    #[test]
    fn learns_scale_parallax_and_height() {
        let mut b = Backdrop::default();
        // The box grows from its left edge at x 528.
        for i in 0..40 {
            let w = 600.0 + 40.0 * i as f32;
            b.learn(&field(w, 528.0 + w / 2.0));
        }
        let l = &b.layers[&(LAYER_Z as i32 - 6000)];
        assert!((l.scale_at(496.0).unwrap() - (0.502 + 0.00122 * 496.0)).abs() < 1e-4);
        assert!((l.parallax.unwrap() - 0.138).abs() < 1e-3, "{:?}", l.parallax);
        // A window the normal size, 300 right of the box's centre: the
        // layer at its normal scale, at its normal-height y, 0.138 x 300 left.
        let w = field(2160.0, 528.0 + 1080.0);
        b.learn(&w);
        let view = xf::View { offset: [300.0, 0.0], size: [496.0, 496.0 * 0.548] };
        let a = b.layer_affine(&w, &w.layers[0], &view).unwrap();
        let normal = 0.502 + 0.00122 * 496.0;
        assert!((a[0].0 - normal / w.layers[0].scale * 2160.0 / 496.0).abs() < 1e-3, "{a:?}");
        assert!((a[0].1 - 2.0 * -0.138 * 300.0 / 496.0).abs() < 1e-3, "{a:?}");
        let dy = (0.2 * view.size[1] - 50.0) - (0.2 * w.camera.size[1] - 50.0);
        assert!((a[1].1 - 2.0 * dy / view.size[1]).abs() < 1e-2, "{a:?} {dy}");
    }
}
