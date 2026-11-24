//! GX captures, for timing decoding and transform away from the game
//! (`examples/gx_bench.rs`, which also runs on a phone from `adb shell`).
//!
//! With `NSMBW_GX_CAPTURE=<file>,<first field>,<fields>` set, the threaded
//! GX port saves the decoder's registers and guest RAM (MEM1 and MEM2) as
//! the first field starts, then each field's stream as the GX thread got
//! it (display lists already copied in, indexed XF loads resolved) and the
//! game camera changes, and writes the file once the last field is done.
//! RAM is the first field's: later fields read vertex arrays and textures
//! from it as they were then, which is right for timing, not for pictures.
//! A capture holds game data (name it `*.gxcap`, which git ignores): keep
//! it out of the repository and under the data folder, never uploaded.

use crate::WorldCamera;

const MAGIC: &[u8; 8] = b"NSGXCAP1";

/// One step of a captured field.
#[derive(Clone, Debug, PartialEq)]
pub enum Rec {
    /// Commands, as [`crate::Gpu::feed`] takes them.
    Data(Vec<u8>),
    /// [`crate::Gpu::set_world`].
    World(Option<WorldCamera>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Capture {
    /// [`crate::Gpu::snap_regs`]'s saved bytes.
    pub regs: Vec<u8>,
    pub mem1: Vec<u8>,
    pub mem2: Vec<u8>,
    pub fields: Vec<Vec<Rec>>,
}

/// The capture asked for by `NSMBW_GX_CAPTURE`: file, first field and
/// number of fields.
pub(crate) fn requested() -> Option<(std::path::PathBuf, u64, usize)> {
    let v = std::env::var("NSMBW_GX_CAPTURE").ok()?;
    let mut parts = v.rsplitn(3, ',');
    let count = parts.next()?.trim().parse().ok()?;
    let first = parts.next()?.trim().parse().ok()?;
    Some((parts.next()?.into(), first, count))
}

fn put(out: &mut Vec<u8>, b: &[u8]) {
    out.extend_from_slice(&(b.len() as u32).to_le_bytes());
    out.extend_from_slice(b);
}

impl Capture {
    pub fn write(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        put(&mut out, &self.regs);
        put(&mut out, &self.mem1);
        put(&mut out, &self.mem2);
        out.extend_from_slice(&(self.fields.len() as u32).to_le_bytes());
        for f in &self.fields {
            out.extend_from_slice(&(f.len() as u32).to_le_bytes());
            for r in f {
                match r {
                    Rec::Data(d) => {
                        out.push(0);
                        put(&mut out, d);
                    }
                    Rec::World(c) => {
                        out.push(1);
                        let c = c.as_ref();
                        out.push(u8::from(c.is_some()));
                        let c = c.copied().unwrap_or_default();
                        for v in [c.size[0], c.size[1], c.centre[0], c.centre[1], c.normal_size[0], c.normal_size[1]] {
                            out.extend_from_slice(&v.to_le_bytes());
                        }
                        out.push(u8::from(c.normal));
                    }
                }
            }
        }
        out
    }

    pub fn parse(b: &[u8]) -> Result<Self, String> {
        let mut r = Reader { b, at: 0 };
        if r.take(8)? != MAGIC {
            return Err("not a GX capture".into());
        }
        let regs = r.block()?;
        let mem1 = r.block()?;
        let mem2 = r.block()?;
        let nf = r.u32()?;
        let mut fields = Vec::with_capacity(nf);
        for _ in 0..nf {
            let nr = r.u32()?;
            let mut recs = Vec::with_capacity(nr);
            for _ in 0..nr {
                match r.take(1)?[0] {
                    0 => recs.push(Rec::Data(r.block()?)),
                    1 => {
                        let some = r.take(1)?[0] != 0;
                        let mut f = [0.0f32; 6];
                        for v in &mut f {
                            *v = f32::from_le_bytes(r.take(4)?.try_into().unwrap());
                        }
                        let normal = r.take(1)?[0] != 0;
                        let c = WorldCamera { size: [f[0], f[1]], centre: [f[2], f[3]], normal, normal_size: [f[4], f[5]] };
                        recs.push(Rec::World(some.then_some(c)));
                    }
                    t => return Err(format!("unknown record {t}")),
                }
            }
            fields.push(recs);
        }
        Ok(Self { regs, mem1, mem2, fields })
    }
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let s = self.b.get(self.at..self.at + n).ok_or_else(|| format!("truncated at {}", self.at))?;
        self.at += n;
        Ok(s)
    }

    fn u32(&mut self) -> Result<usize, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()) as usize)
    }

    fn block(&mut self) -> Result<Vec<u8>, String> {
        let n = self.u32()?;
        Ok(self.take(n)?.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let c = Capture {
            regs: vec![1, 2, 3],
            mem1: vec![4; 5],
            mem2: vec![6],
            fields: vec![vec![Rec::Data(vec![0x61, 0, 0, 0, 0]), Rec::World(None)], vec![Rec::World(Some(WorldCamera { size: [1.0, 2.0], centre: [3.0, 4.0], normal: true, normal_size: [5.0, 6.0] }))]],
        };
        assert_eq!(Capture::parse(&c.write()).unwrap(), c);
        assert!(Capture::parse(b"nope").is_err());
    }
}
