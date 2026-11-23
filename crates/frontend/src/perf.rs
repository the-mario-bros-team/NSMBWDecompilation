//! `--perf` (and, in the Android app, a file named `perf` in its files):
//! what each field costs, to find where a slow device spends its time.
//!
//! Each field is a row of a CSV file: the game thread's time (and of it,
//! waiting for the GX thread), the GX threads' busy time (decoding, and
//! transforming), the render thread's time for the field, the GPU's time
//! when the renderer measures it, the time since the field before, and the
//! field's draws. Every window of fields, and at the end, the log gets the
//! median, 99th percentile and worst of each.
//!
//! `--perf-skip` names draw categories to leave out, one per window in turn
//! (`none` draws everything): comparing a category's windows with `none`'s
//! shows what drawing it costs. The picture is wrong while one is left out.
//!
//! Guest functions: `--profile-guest` samples where the game spends its
//! time, by function.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{Duration, Instant};

use crate::emu::GameFrame;

/// A category of draws `--perf-skip` leaves out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Skip {
    None,
    All,
    /// Draws that blend with the EFB.
    Blend,
    /// Draws with the depth test on, and with it off.
    Depth,
    NoDepth,
    /// Two or more TEV stages.
    Tev2,
    /// Two or more textures.
    Tex2,
    /// Indirect texturing.
    Indirect,
    /// EFB copies to textures (and the patches drawn from them).
    Copies,
}

const SKIPS: [(&str, Skip); 9] =
    [("none", Skip::None), ("all", Skip::All), ("blend", Skip::Blend), ("depth", Skip::Depth), ("nodepth", Skip::NoDepth), ("tev2", Skip::Tev2), ("tex2", Skip::Tex2), ("indirect", Skip::Indirect), ("copies", Skip::Copies)];

impl Skip {
    /// A comma-separated list of category names.
    pub fn parse_list(s: &str) -> Result<Vec<Skip>, String> {
        s.split(',').filter(|n| !n.is_empty()).map(|n| SKIPS.iter().find(|k| k.0 == n).map(|k| k.1).ok_or_else(|| format!("--perf-skip: no category {n} (one of {})", SKIPS.map(|k| k.0).join(", ")))).collect()
    }

    pub fn name(self) -> &'static str {
        SKIPS.iter().find(|k| k.1 == self).map_or("?", |k| k.0)
    }

    fn drops(self, out: &gx::Output, c: &gx::Cmd) -> bool {
        use gx::sh;
        let d = match c {
            gx::Cmd::Draw(d) => d,
            gx::Cmd::CopyTex(_) | gx::Cmd::Patch(_) => return self == Skip::Copies,
            _ => return false,
        };
        let Some(s) = out.shadings.get(d.shading as usize) else { return false };
        let genmode = s.0[sh::GENMODE];
        match self {
            Skip::None | Skip::Copies => false,
            Skip::All => true,
            Skip::Blend => d.raster.blend & 1 != 0,
            Skip::Depth => d.raster.zmode & 1 != 0,
            Skip::NoDepth => d.raster.zmode & 1 == 0,
            Skip::Tev2 => (genmode >> 10) & 0xF >= 1,
            Skip::Tex2 => s.0[sh::TEX..sh::TEX + 8].iter().filter(|&&t| t != gx::NO_TEXTURE).count() >= 2,
            Skip::Indirect => (genmode >> 16) & 7 != 0,
        }
    }
}

/// `--perf`, `--perf-skip` and `--perf-window`.
#[derive(Clone, Debug)]
pub struct Options {
    pub csv: Option<std::path::PathBuf>,
    pub skip: Vec<Skip>,
    pub window: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self { csv: None, skip: Vec::new(), window: 300 }
    }
}

/// One field's numbers (milliseconds, then counts).
#[derive(Clone, Copy, Default)]
struct Row {
    ms: [f32; MS.len()],
    /// Draw commands, and the game's draws merged into them.
    draws: u32,
    draw_ops: u32,
    /// Texture bytes hashed.
    hashed: u64,
}

/// The timed columns, in CSV order.
const MS: [&str; 8] = ["emu", "gx_wait", "out_wait", "decode", "transform", "render", "gpu", "interval"];

pub struct Perf {
    csv: Option<BufWriter<File>>,
    cycle: Vec<Skip>,
    /// Fields per window.
    window: u64,
    /// Index in `cycle`, and fields so far in this window.
    phase: usize,
    in_window: u64,
    rows: Vec<Row>,
    /// Every field so far, by the category left out.
    all: Vec<(Skip, Row)>,
    last: Option<Instant>,
}

fn ms(d: Duration) -> f32 {
    d.as_secs_f32() * 1000.0
}

impl Perf {
    pub fn open(o: &Options) -> Result<Self, String> {
        Self::new(o.csv.as_deref(), o.skip.clone(), o.window)
    }

    /// Writes rows to `csv` (if given), leaving out each of `cycle` in turn
    /// for `window` fields.
    pub fn new(csv: Option<&Path>, cycle: Vec<Skip>, window: u64) -> Result<Self, String> {
        let csv = match csv {
            Some(p) => {
                let mut w = BufWriter::new(File::create(p).map_err(|e| format!("{}: {e}", p.display()))?);
                let _ = writeln!(w, "field,skip,{},draws,gx_draws,vertices,uploads,hashed_kb", MS.map(|c| format!("{c}_ms")).join(","));
                Some(w)
            }
            None => None,
        };
        let cycle = if cycle.is_empty() { vec![Skip::None] } else { cycle };
        if cycle.len() > 1 || cycle[0] != Skip::None {
            println!("perf: leaving out {} in turn, {window} fields each (the picture is wrong meanwhile)", cycle.iter().map(|s| s.name()).collect::<Vec<_>>().join(", "));
        }
        Ok(Self { csv, cycle, window: window.max(1), phase: 0, in_window: 0, rows: Vec::new(), all: Vec::new(), last: None })
    }

    /// The category left out now.
    pub fn skip(&self) -> Skip {
        self.cycle[self.phase]
    }

    /// Leaves the category left out now out of `out`.
    pub fn filter(&self, out: &mut gx::Output) {
        let skip = self.skip();
        if skip != Skip::None {
            let cmds = std::mem::take(&mut out.cmds);
            out.cmds = cmds.into_iter().filter(|c| !skip.drops(out, c)).collect();
        }
    }

    /// A field done: the render thread took `render` for it, and the GPU
    /// `gpu` (if measured).
    pub fn field(&mut self, f: &GameFrame, render: Duration, gpu: Option<Duration>) {
        let now = Instant::now();
        let interval = self.last.map_or(0.0, |l| ms(now - l));
        self.last = Some(now);
        let out = &f.out;
        let draws = out.cmds.iter().filter(|c| matches!(c, gx::Cmd::Draw(_))).count() as u32;
        let row = Row { ms: [ms(f.emu), ms(f.gx_wait), ms(f.output_wait), ms(out.busy[0]), ms(out.busy[1]), ms(render), gpu.map_or(0.0, ms), interval], draws, draw_ops: out.draw_ops, hashed: out.hashed };
        let skip = self.skip().name();
        if let Some(w) = &mut self.csv {
            let cols: Vec<String> = row.ms.iter().map(|v| format!("{v:.3}")).collect();
            let _ = writeln!(w, "{},{skip},{},{draws},{},{},{},{}", f.index, cols.join(","), out.draw_ops, out.vertices.len(), out.new_textures.len(), out.hashed >> 10);
        }
        self.rows.push(row);
        self.all.push((self.skip(), row));
        self.in_window += 1;
        if self.in_window >= self.window {
            println!("perf fields {}-{} [{}]: {}", f.index + 1 - self.in_window, f.index, self.skip().name(), summary(&self.rows));
            if let Some(w) = &mut self.csv {
                let _ = w.flush();
            }
            self.rows.clear();
            self.in_window = 0;
            self.phase = (self.phase + 1) % self.cycle.len();
        }
    }

    /// The whole run's summary, per category left out.
    pub fn finish(&mut self) {
        if let Some(w) = &mut self.csv {
            let _ = w.flush();
        }
        for skip in self.cycle.clone() {
            let rows: Vec<Row> = self.all.iter().filter(|r| r.0 == skip).map(|r| r.1).collect();
            if !rows.is_empty() {
                println!("perf, {} fields [{}]: {}", rows.len(), skip.name(), summary(&rows));
            }
        }
    }
}

/// Median, 99th percentile and worst of each column with any time in it.
fn summary(rows: &[Row]) -> String {
    let mut parts = Vec::new();
    for (i, name) in MS.iter().enumerate() {
        let mut v: Vec<f32> = rows.iter().map(|r| r.ms[i]).collect();
        if v.iter().all(|&x| x == 0.0) {
            continue;
        }
        v.sort_by(f32::total_cmp);
        let at = |p: usize| v[(v.len() * p).div_ceil(100).saturating_sub(1).min(v.len() - 1)];
        parts.push(format!("{name} {:.2}/{:.2}/{:.2}", at(50), at(99), v[v.len() - 1]));
    }
    let n = rows.len().max(1) as u64;
    let draws = rows.iter().map(|r| u64::from(r.draws)).sum::<u64>() / n;
    let ops = rows.iter().map(|r| u64::from(r.draw_ops)).sum::<u64>() / n;
    let kb = rows.iter().map(|r| r.hashed).sum::<u64>() / n >> 10;
    format!("{} ms (p50/p99/worst); a field: {draws} draws ({ops} before merging), {kb} KB of textures hashed", parts.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentiles() {
        let rows: Vec<Row> = (1..=100).map(|i| Row { ms: [i as f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], draws: 2, draw_ops: 3, hashed: 2048 }).collect();
        assert_eq!(summary(&rows), "emu 50.00/99.00/100.00 ms (p50/p99/worst); a field: 2 draws (3 before merging), 2 KB of textures hashed");
        assert_eq!(Skip::parse_list("none,blend").unwrap(), vec![Skip::None, Skip::Blend]);
        assert!(Skip::parse_list("bogus").is_err());
    }
}
