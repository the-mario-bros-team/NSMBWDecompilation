//! Skips `EGG::LightTextureManager::drawAndCaptureTexture` (0x802CD4B0)
//! when nothing it reads has changed since its last call, after the same
//! hook in WitherBlitz's NSMB-Wii-Recompilation
//! (`runtime/src/hle/gx/light_texture_cache.cpp`).
//!
//! Every frame, courses and the world map redraw their light textures (the
//! normal-sphere maps their models are lit with) into a corner of the EFB
//! and copy each one out, around a save and restore of the EFB they draw
//! over: interpreted EGG code, GX quads and copies, for the same pictures
//! while the lights hold still.
//!
//! The key covers what the call reads: its arguments, the manager, its light
//! textures, their light set (light objects and ambient colours), and the EGG
//! statics it uses. Objects are hashed whole as their heap blocks (the
//! expanded heap's 16-byte header before each block gives the size), so no
//! field layout is assumed. When the key matches the manager's last call, the
//! EFB copies that call made are still the textures at their addresses (the
//! GX texture cache keeps a copy until its address is copied to again or its
//! RAM changes), so the skip only sets the manager's drawn-this-frame bit,
//! which is all the call leaves behind in what the key covers.
//!
//! `NSMBW_LIGHTTEX=off` always runs the game's code.

use guest_cpu::Cpu;
use game_version::E1;
use guest_mem::Mem;

pub const DRAW_AND_CAPTURE: E1 = E1(0x802C_D4B0);

/// `EGG::Frustum::sGlobalOffset` .. `EGG::StateGX::s_optimize`. Not
/// `EGG::TextureBuffer`'s list after it: the call borrows a scratch buffer
/// to save and restore the EFB, which turns the list round every call, and
/// which buffer it is makes no difference.
const STATICS: (E1, E1) = (E1(0x8042_AEC0), E1(0x8042_AF58));
const MAX_BLOCK: u32 = 0x10000;
/// What is hashed of an object without a heap header.
const UNKNOWN_BLOCK: u32 = 0x100;

#[derive(Clone, Copy, Default)]
struct State {
    manager: u32,
    key: Option<u64>,
}

pub struct LightTextures {
    on: bool,
    /// Per manager (one per scene layer).
    states: [State; 4],
    pub calls: u64,
    pub skips: u64,
}

impl Default for LightTextures {
    fn default() -> Self {
        Self { on: std::env::var("NSMBW_LIGHTTEX").map_or(true, |v| v != "off"), states: [State::default(); 4], calls: 0, skips: 0 }
    }
}

fn read32(mem: &Mem, a: u32) -> Option<u32> {
    mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()))
}

fn read16(mem: &Mem, a: u32) -> Option<u16> {
    mem.ram(a, 2).map(|b| u16::from_be_bytes(b.try_into().unwrap()))
}

fn mix(mut h: u64, bytes: &[u8]) -> u64 {
    h ^= (bytes.len() as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mut chunks = bytes.chunks_exact(8);
    for c in &mut chunks {
        h = (h ^ u64::from_le_bytes(c.try_into().unwrap())).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        h ^= h >> 29;
    }
    let mut last = [0u8; 8];
    last[..chunks.remainder().len()].copy_from_slice(chunks.remainder());
    h = (h ^ u64::from_le_bytes(last)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^ (h >> 31)
}

/// Folds the heap block at `object` (or [`UNKNOWN_BLOCK`] bytes of it) into `h`.
fn hash_object(mem: &Mem, object: u32, h: &mut u64) -> Option<()> {
    if object < 0x8000_0010 {
        return None;
    }
    let mut size = UNKNOWN_BLOCK;
    if read16(mem, object - 0x10) == Some(0x5544) {
        // 'UD': a used block of the expanded heap; its size follows.
        if let Some(s) = read32(mem, object - 0xC).filter(|&s| s != 0 && s <= MAX_BLOCK) {
            size = s;
        }
    }
    *h = mix(*h ^ u64::from(object), mem.ram(object, size as usize)?);
    Some(())
}

/// The key of a call with manager `manager` and float arguments `args`, or
/// `None` when something is not plain guest memory (never skipped).
fn key(mem: &Mem, manager: u32, args: [u64; 4]) -> Option<u64> {
    let mut h = 0;
    for a in args {
        h = mix(h, &a.to_le_bytes());
    }
    let (from, to) = (STATICS.0.va(), STATICS.1.va());
    h = mix(h, mem.ram(from, (to - from) as usize)?);
    hash_object(mem, manager, &mut h)?;
    let screen = read32(mem, manager + 0xC)?;
    hash_object(mem, screen, &mut h)?;
    let count = read16(mem, manager + 6)?;
    let textures = read32(mem, manager + 8)?;
    if count > 64 {
        return None;
    }
    hash_object(mem, textures, &mut h)?;
    let mut last_set = 0;
    for i in 0..u32::from(count) {
        let texture = read32(mem, textures + 4 * i)?;
        hash_object(mem, texture, &mut h)?;
        let set = read32(mem, texture + 0x30)?;
        if set == last_set {
            continue; // the textures usually share one light set
        }
        last_set = set;
        hash_object(mem, set, &mut h)?;
        let light_count = read16(mem, set + 0x14)?;
        let lights = read32(mem, set + 0x18)?;
        if light_count > 256 {
            return None;
        }
        hash_object(mem, lights, &mut h)?;
        let ambient_owner = read32(mem, set + 0xC)?;
        hash_object(mem, ambient_owner, &mut h)?;
        let ambient = read32(mem, ambient_owner + 0xC)?;
        hash_object(mem, ambient, &mut h)?;
        for l in 0..u32::from(light_count) {
            let light = read32(mem, lights + 4 * l)?;
            if light != 0 {
                hash_object(mem, light, &mut h)?;
            }
        }
    }
    Some(h)
}

/// The call's own early-outs (light textures off, already drawn this frame,
/// none to draw) return false: then it does nothing worth skipping.
/// Otherwise sets the manager's drawn-this-frame bit, as the call does first.
fn mark_drawn(mem: &mut Mem, manager: u32) -> Option<()> {
    let flags = read32(mem, manager + 4)?;
    if (flags >> 24) & 0x10 == 0 {
        return None;
    }
    let screen = read32(mem, manager + 0xC)?;
    let screen_id = u32::from(mem.ram(screen + 0x1D, 1)?[0]);
    let bit = if screen_id < 32 { 1 << screen_id } else { 0 };
    let drawn = read32(mem, manager + 0x10)?;
    if drawn & bit != 0 || read16(mem, manager + 6)? == 0 {
        return None;
    }
    mem.write_u32(manager + 0x10, drawn | bit).ok()
}

impl LightTextures {
    fn state(&mut self, manager: u32) -> &mut State {
        let i = self.states.iter().position(|s| s.manager == manager || s.manager == 0).unwrap_or(3);
        self.states[i].manager = manager;
        &mut self.states[i]
    }

    /// At the call's first instruction: true if it was answered here (the
    /// caller returns to `lr`), false to run the game's code.
    pub fn skip(&mut self, cpu: &Cpu, mem: &mut Mem) -> bool {
        if !self.on {
            return false;
        }
        self.calls += 1;
        let manager = cpu.gpr[3];
        let args = std::array::from_fn(|i| cpu.fpr[1 + i][0].to_bits());
        let key = key(mem, manager, args);
        let state = self.state(manager);
        let repeat = key.is_some() && state.key == key;
        state.key = key;
        if repeat && mark_drawn(mem, manager).is_some() {
            self.skips += 1;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANAGER: u32 = 0x8000_1000;
    const SCREEN: u32 = 0x8000_1200;
    const TEXTURES: u32 = 0x8000_1400;
    const TEXTURE: u32 = 0x8000_1600;
    const SET: u32 = 0x8000_1800;
    const AMBIENT_OWNER: u32 = 0x8000_1A00;
    const AMBIENT: u32 = 0x8000_1C00;
    const LIGHTS: u32 = 0x8000_1E00;
    const LIGHT: u32 = 0x8000_2000;

    /// A manager with one light texture lit by one light, on screen 2; the
    /// manager is a heap block ('UD' header with its size).
    fn scene() -> (Mem, Cpu) {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        let w = |m: &mut Mem, a: u32, v: u32| m.write_u32(a, v).unwrap();
        w(&mut mem, MANAGER - 0x10, 0x5544_0000);
        w(&mut mem, MANAGER - 0xC, 0x20);
        w(&mut mem, MANAGER + 4, 0x1000_0001); // enabled; one texture
        w(&mut mem, MANAGER + 8, TEXTURES);
        w(&mut mem, MANAGER + 0xC, SCREEN);
        mem.ram_mut(SCREEN + 0x1D, 1).unwrap()[0] = 2;
        w(&mut mem, TEXTURES, TEXTURE);
        w(&mut mem, TEXTURE + 0x30, SET);
        w(&mut mem, SET + 0xC, AMBIENT_OWNER);
        w(&mut mem, SET + 0x14, 0x0001_0000); // one light
        w(&mut mem, SET + 0x18, LIGHTS);
        w(&mut mem, AMBIENT_OWNER + 0xC, AMBIENT);
        w(&mut mem, LIGHTS, LIGHT);
        w(&mut mem, LIGHT, 0x3F80_0000);
        let mut cpu = Cpu::new();
        cpu.gpr[3] = MANAGER;
        (mem, cpu)
    }

    #[test]
    fn repeats_are_skipped_until_a_light_changes() {
        let (mut mem, cpu) = scene();
        let mut lt = LightTextures { on: true, ..Default::default() };
        let drawn = |m: &Mem| read32(m, MANAGER + 0x10).unwrap();
        assert!(!lt.skip(&cpu, &mut mem), "first call draws");
        assert!(lt.skip(&cpu, &mut mem), "same inputs: skipped");
        assert_eq!(drawn(&mem), 1 << 2, "a skip marks the screen drawn");
        assert!(!lt.skip(&cpu, &mut mem), "already drawn this frame: the game's own early-out");
        mem.write_u32(MANAGER + 0x10, 0).unwrap(); // next frame
        mem.write_u32(LIGHT, 0x4000_0000).unwrap(); // the light moved
        assert!(!lt.skip(&cpu, &mut mem), "changed light: drawn again");
        assert_eq!(drawn(&mem), 0);
        assert!(lt.skip(&cpu, &mut mem));
        assert_eq!((lt.calls, lt.skips), (5, 2));
    }
}

/// The managers and their keys; `on` stays the loading machine's (a net
/// session checks both sides agree) and the counts are statistics.
impl guest_mem::snap::Snapshot for LightTextures {
    fn snap(&mut self, s: &mut guest_mem::snap::Snap) {
        for st in &mut self.states {
            guest_mem::snap!(s; st.manager, st.key);
        }
    }
}

impl LightTextures {
    /// Forgets every key, so each manager's next call redraws.
    pub fn forget(&mut self) {
        for st in &mut self.states {
            st.key = None;
        }
    }

    pub fn on(&self) -> bool {
        self.on
    }
}
