//! Joining a course in progress (docs/PORT_PLAN.md, section 12): a player
//! whose Wii Remote joined online appears in the running course in a
//! bubble, the way a player who lost a life comes back.
//!
//! At the end of `dBgGm_c::execute` (in the frame, where actors may be
//! created) the joiner's slot is marked as in the game (`0x80354E50`), given
//! a character no one else plays, flagged to start in a bubble (bit 4 of the
//! character's flags at `0x80354E80`, which `daPyMng_c::isCreateBalloon`
//! reads and `checkCorrectCreateInfo` clears again at the next course start),
//! and the CPU is sent into `daPyMng_c::create(slot, &pos, 0, 0)` with a
//! stack frame of its own holding `pos`, then into
//! `dAcPy_c::setBalloonInDamage` for the new player. When that returns to
//! the same `blr`, the flag and the registers are put back and `execute`
//! returns as usual.
//! Every machine does this at the same instruction, so the game stays the
//! same on all of them.

use guest_cpu::Cpu;
use guest_mem::snap::{Snap, Snapshot};
use game_version::E1;
use guest_mem::Mem;

/// `daPyMng_c::create(int, mVec3_c*, int, u8)`.
const CREATE: E1 = E1(0x8005_EEE0);
const SDA: u32 = 0x8042_F680;
/// The game mode: 0 in a course played normally, 2 in the title's demo.
const GAME_MODE: E1 = E1(SDA - 0x547C);
const PLAYER_IDS: E1 = E1(0x8035_4E10);
const IN_GAME: E1 = E1(0x8035_4E50);
const CHARACTER: E1 = E1(0x8035_4E60);
const CHAR_FLAGS: E1 = E1(0x8035_4E80);
const IN_BUBBLE: u32 = 4;
/// `dRemoconMng_c`: per remote, a `dConnect_c` whose byte at +0x54 says the
/// remote may stay connected; in a course the game disconnects any other
/// remote that connects (`dConnect_c::executeState_Setup`).
const REMOCON_MNG: E1 = E1(SDA - 0x5658);
const CONNECT_ALLOWED: u32 = 0x54;
/// The stack frame given to the call: back chain, LR save word, `pos`.
const FRAME: u32 = 0x20;

fn read32(mem: &Mem, a: u32) -> Option<u32> {
    mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()))
}

/// `dAcPy_c::setBalloonInDamage`: the bubble a player comes back in after
/// losing a life (demo state "none" 0, bubble type 0). `create` puts a
/// player that starts in a bubble into the course-start kind instead
/// (`setBalloonInNextgoto`: demo "none" 1, type 2), which stays where it
/// is; calling this right after makes the joiner's bubble float to the
/// others like a revived player's.
const BALLOON_IN_DAMAGE: E1 = E1(0x8013_8E60);

/// A call made for a join, and what to put back when it returns.
#[derive(Clone, Copy, Default)]
struct Call {
    r1: u32,
    r3: u32,
    lr: u32,
    /// 0: `create`; 1: `setBalloonInDamage`.
    step: u32,
    slot: u32,
    /// The character flags word and its value before.
    flags_at: u32,
    flags: u32,
}

impl Snapshot for Call {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.r1, self.r3, self.lr, self.step, self.slot, self.flags_at, self.flags);
    }
}

#[derive(Default)]
pub struct Joins {
    /// Slots waiting to be brought into a course.
    pending: Vec<usize>,
    /// A call in flight.
    call: Option<Call>,
    /// Players brought in (for the exit report).
    pub joined: u64,
}

impl Joins {
    /// Brings player `slot` (0-based) into the next course frame, unless it
    /// is in the game already by then.
    pub fn request(&mut self, slot: usize) {
        if slot < 4 && !self.pending.contains(&slot) {
            self.pending.push(slot);
        }
    }

    /// At the end of `dBgGm_c::execute`, when one of our calls returned
    /// there: makes the next call, or puts the registers back (the `blr`
    /// then runs as usual). True if this was such a return.
    pub fn returned(&mut self, cpu: &mut Cpu, mem: &mut Mem) -> bool {
        let Some(mut c) = self.call.take() else { return false };
        if c.step == 0 {
            if let Some(actor) = crate::camera::player_actor(mem, c.slot as usize) {
                c.step = 1;
                self.call = Some(c);
                cpu.gpr[3] = actor;
                cpu.lr = cpu.pc;
                cpu.pc = BALLOON_IN_DAMAGE.va();
                return true;
            }
        }
        let _ = mem.write_u32(c.flags_at, c.flags);
        cpu.gpr[1] = c.r1;
        cpu.gpr[3] = c.r3;
        cpu.lr = c.lr;
        true
    }

    /// At the end of `dBgGm_c::execute`, with the CPU at its `blr`: starts
    /// bringing in a waiting player below `near` (a player the camera
    /// follows), under the zone where it waits out of sight, as a player
    /// who lost a life does, until its bubble comes in from the screen edge.
    /// Returns the slot if the CPU now runs `daPyMng_c::create`; that remote
    /// may then connect (the caller lets it page the Wii).
    pub fn start(&mut self, cpu: &mut Cpu, mem: &mut Mem, near: Option<[f32; 2]>) -> Option<usize> {
        if self.pending.is_empty() || read32(mem, GAME_MODE.va()) != Some(0) {
            return None;
        }
        let Some([x, y]) = near else { return None };
        // Already in the game (joined on the map with Add/Drop Players, or
        // created at the course start): nothing to do.
        self.pending.retain(|&s| read32(mem, IN_GAME.va() + 4 * s as u32) == Some(0) && read32(mem, PLAYER_IDS.va() + 4 * s as u32) == Some(0));
        let Some(slot) = self.pending.first().copied() else { return None };
        self.pending.remove(0);
        // A character no player in the game has.
        let taken: Vec<u32> = (0..4).filter(|&j| j != slot && read32(mem, IN_GAME.va() + 4 * j as u32).is_some_and(|v| v != 0)).filter_map(|j| read32(mem, CHARACTER.va() + 4 * j as u32)).collect();
        let mine = read32(mem, CHARACTER.va() + 4 * slot as u32).filter(|c| *c < 4 && !taken.contains(c));
        let Some(ch) = mine.or_else(|| (0..4).find(|c| !taken.contains(c))) else { return None };
        let flags = read32(mem, CHAR_FLAGS.va() + 4 * ch).unwrap_or(0);
        let r1 = cpu.gpr[1];
        let sp = r1 - FRAME;
        let ok = [
            mem.write_u32(IN_GAME.va() + 4 * slot as u32, 1),
            mem.write_u32(CHARACTER.va() + 4 * slot as u32, ch),
            mem.write_u32(CHAR_FLAGS.va() + 4 * ch, flags | IN_BUBBLE),
            mem.write_u32(sp, r1),
            mem.write_u32(sp + 8, x.to_bits()),
            mem.write_u32(sp + 12, crate::camera::zone_bottom(mem).map_or(y, |b| b - 64.0).to_bits()),
            mem.write_u32(sp + 16, 0),
        ];
        if ok.iter().any(Result::is_err) {
            return None;
        }
        self.call = Some(Call { r1, r3: cpu.gpr[3], lr: cpu.lr, step: 0, slot: slot as u32, flags_at: CHAR_FLAGS.va() + 4 * ch, flags });
        cpu.gpr[1] = sp;
        cpu.gpr[3] = slot as u32;
        cpu.gpr[4] = sp + 8;
        cpu.gpr[5] = 0;
        cpu.gpr[6] = 0;
        cpu.lr = cpu.pc;
        cpu.pc = CREATE.va();
        if let Some(c) = read32(mem, REMOCON_MNG.va()).and_then(|m| read32(mem, m + 4 + 4 * slot as u32)).filter(|&c| c >= 0x8000_0000) {
            let _ = mem.write_u8(c + CONNECT_ALLOWED, 1);
        }
        self.joined += 1;
        Some(slot)
    }
}

/// Waiting slots and a call in flight are guest state: every machine must
/// see them alike, and a state saved mid-call must finish it.
impl Snapshot for Joins {
    fn snap(&mut self, s: &mut Snap) {
        guest_mem::snap!(s; self.pending, self.call);
    }
}
