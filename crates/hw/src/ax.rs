//! The AX audio microcode (Wii version), done on the host: it runs the
//! command lists the game's AX library sends the DSP every 3 ms frame
//! (96 samples at 32 kHz).
//!
//! The layout follows what NSMBW sends (found by dumping its lists and
//! parameter blocks) and Dolphin's `AXWii` HLE:
//!
//! | Command | Arguments | Here |
//! |---|---|---|
//! | `0x00` setup | buffer init address | zeroes the buses |
//! | `0x01`/`0x02` add/subtract to L/R | address of 96 `s32` | the last frame's surround, folded into L and R |
//! | `0x03` add/subtract to L/R | two addresses | first added, second subtracted |
//! | `0x04` process | head of the parameter-block list | every running voice |
//! | `0x05`–`0x07` aux A/B/C | volume, write address, read address | the bus goes to the CPU's effect callback; its return is mixed back only with [`Ax::aux_return`] |
//! | `0x0A` compressor | threshold, frames, table | ignored (as in Dolphin) |
//! | `0x0B`/`0x0C` output | volume, surround address, L/R address | the 16-bit stereo frame the AI DMA plays |
//! | `0x0D` Wii Remote speakers | four addresses | silent |
//! | `0x0E` end | | |
//!
//! A voice is decoded (ADPCM, PCM16, PCM8) by the accelerator model, resampled
//! to 32 kHz, scaled by its volume envelope, filtered (one-pole low-pass,
//! biquad), and added to the buses its mixer-control word selects, with
//! per-sample volume ramps.

use game_version::E1;
use guest_mem::Mem;

/// Samples per frame (3 ms at 32 kHz).
pub const FRAME: usize = 96;

/// Byte offsets in a Wii AX parameter block (0x140 bytes).
pub mod pb {
    pub const NEXT: u32 = 0x00;
    pub const SRC_TYPE: u32 = 0x08;
    pub const MIXER_CONTROL: u32 = 0x0C;
    pub const RUNNING: u32 = 0x10;
    pub const IS_STREAM: u32 = 0x12;
    /// Volume and delta pairs: main L, R, aux A L, R, aux B L, R, aux C L,
    /// R, then main S, aux A S, aux B S, aux C S.
    pub const MIXER: u32 = 0x14;
    /// Last mixed sample per bus channel: L of main, A, B, C; R of the
    /// same; then S of the same.
    pub const DPOP: u32 = 0x52;
    pub const VOL_ENV: u32 = 0x6A;
    pub const LOOPING: u32 = 0x6E;
    pub const FORMAT: u32 = 0x70;
    pub const LOOP_ADDR: u32 = 0x72;
    pub const END_ADDR: u32 = 0x76;
    pub const CUR_ADDR: u32 = 0x7A;
    pub const COEFS: u32 = 0x7E;
    pub const PRED_SCALE: u32 = 0xA0;
    pub const YN1: u32 = 0xA2;
    pub const YN2: u32 = 0xA4;
    pub const RATIO: u32 = 0xA6;
    pub const FRAC: u32 = 0xAA;
    pub const LAST_SAMPLES: u32 = 0xAC;
    pub const LOOP_PRED_SCALE: u32 = 0xB4;
    pub const LOOP_YN1: u32 = 0xB6;
    pub const LOOP_YN2: u32 = 0xB8;
    pub const LPF: u32 = 0xBA;
    pub const BIQUAD: u32 = 0xC2;
    pub const SIZE: usize = 0x140;
}

pub const FMT_ADPCM: u16 = 0x00;
pub const FMT_PCM16: u16 = 0x0A;
pub const FMT_PCM8: u16 = 0x19;

/// Mixer-control bit of the first bus channel per bus (main, aux A, B, C);
/// within a bus: +0 L, +1 R, +2 L/R ramp, +3 S, +4 S ramp.
const BUS_BITS: [u32; 4] = [0, 16, 21, 26];
/// Parameter blocks walked per frame at most (the SDK has 96).
const MAX_VOICES: usize = 128;

/// One parameter block, as 16-bit words.
struct Pb([u16; pb::SIZE / 2]);

impl Pb {
    fn get(&self, off: u32) -> u16 {
        self.0[off as usize / 2]
    }
    fn get_i(&self, off: u32) -> i16 {
        self.get(off) as i16
    }
    fn get32(&self, off: u32) -> u32 {
        u32::from(self.get(off)) << 16 | u32::from(self.get(off + 2))
    }
    fn set(&mut self, off: u32, v: u16) {
        self.0[off as usize / 2] = v;
    }
    fn set32(&mut self, off: u32, v: u32) {
        self.set(off, (v >> 16) as u16);
        self.set(off + 2, v as u16);
    }
}

fn read_pb(mem: &Mem, addr: u32) -> Option<Pb> {
    let b = mem.ram(addr, pb::SIZE)?;
    Some(Pb(std::array::from_fn(|i| u16::from_be_bytes([b[2 * i], b[2 * i + 1]]))))
}

fn write_pb(mem: &mut Mem, addr: u32, p: &Pb) {
    if let Some(b) = mem.ram_mut(addr, pb::SIZE) {
        for (i, w) in p.0.iter().enumerate() {
            b[2 * i..2 * i + 2].copy_from_slice(&w.to_be_bytes());
        }
    }
}

/// Effective address of a DSP accelerator byte (physical MEM1 or MEM2).
fn accel_byte(mem: &Mem, phys: u32) -> u8 {
    mem.ram(0x8000_0000 | (phys & 0x1FFF_FFFF), 1).map_or(0, |b| b[0])
}

/// The DSP's sample accelerator for one voice: reads samples at the
/// voice's current address, decoding ADPCM, and loops or stops at the end
/// address (the last sample played).
struct Accel {
    format: u16,
    looping: bool,
    stream: bool,
    loop_addr: u32,
    end: u32,
    cur: u32,
    coefs: [i16; 16],
    pred_scale: u16,
    yn1: i16,
    yn2: i16,
    loop_pred_scale: u16,
    loop_yn1: i16,
    loop_yn2: i16,
    /// A one-shot voice played its last sample.
    ended: bool,
}

impl Accel {
    fn load(p: &Pb) -> Self {
        Self {
            format: p.get(pb::FORMAT),
            looping: p.get(pb::LOOPING) != 0,
            stream: p.get(pb::IS_STREAM) != 0,
            loop_addr: p.get32(pb::LOOP_ADDR),
            end: p.get32(pb::END_ADDR),
            cur: p.get32(pb::CUR_ADDR),
            coefs: std::array::from_fn(|i| p.get_i(pb::COEFS + 2 * i as u32)),
            pred_scale: p.get(pb::PRED_SCALE),
            yn1: p.get_i(pb::YN1),
            yn2: p.get_i(pb::YN2),
            loop_pred_scale: p.get(pb::LOOP_PRED_SCALE),
            loop_yn1: p.get_i(pb::LOOP_YN1),
            loop_yn2: p.get_i(pb::LOOP_YN2),
            ended: false,
        }
    }

    fn store(&self, p: &mut Pb) {
        p.set32(pb::CUR_ADDR, self.cur);
        p.set(pb::PRED_SCALE, self.pred_scale);
        p.set(pb::YN1, self.yn1 as u16);
        p.set(pb::YN2, self.yn2 as u16);
    }

    fn read(&mut self, mem: &Mem) -> i16 {
        if self.ended {
            return 0;
        }
        let at = self.cur;
        let sample = match self.format {
            FMT_ADPCM => {
                let mut at = at;
                if at & 15 == 0 {
                    // A frame's first byte is its predictor and scale.
                    self.pred_scale = u16::from(accel_byte(mem, at >> 1));
                    at += 2;
                    self.cur = at;
                }
                let byte = accel_byte(mem, at >> 1);
                let nibble = i32::from(if at & 1 != 0 { byte & 0xF } else { byte >> 4 });
                let nibble = if nibble >= 8 { nibble - 16 } else { nibble };
                let scale = 1 << (self.pred_scale & 0xF);
                let k = usize::from((self.pred_scale >> 4) & 7);
                let (c1, c2) = (i32::from(self.coefs[2 * k]), i32::from(self.coefs[2 * k + 1]));
                let v = (scale * nibble) + ((0x400 + c1 * i32::from(self.yn1) + c2 * i32::from(self.yn2)) >> 11);
                let v = v.clamp(-0x7FFF, 0x7FFF) as i16;
                self.yn2 = self.yn1;
                self.yn1 = v;
                v
            }
            FMT_PCM16 => {
                let b = at.wrapping_mul(2);
                i16::from_be_bytes([accel_byte(mem, b), accel_byte(mem, b + 1)])
            }
            FMT_PCM8 => i16::from(accel_byte(mem, at) as i8) << 8,
            _ => 0,
        };
        let at = self.cur;
        if at == self.end {
            // The accelerator always goes back to the loop address; the
            // microcode then stops a voice that does not loop. NW4R relies on
            // this: a one-shot's loop address is its zero buffer, and it
            // counts the sound finished once the voice's address is there.
            self.cur = self.loop_addr;
            if self.looping {
                if self.format == FMT_ADPCM {
                    // As the AX microcode does: the loop's predictor, and
                    // its history unless the voice is a stream (whose
                    // history carries on).
                    self.pred_scale = self.loop_pred_scale;
                    if !self.stream {
                        self.yn1 = self.loop_yn1;
                        self.yn2 = self.loop_yn2;
                    }
                }
            } else {
                self.ended = true;
            }
        } else {
            self.cur = at.wrapping_add(1);
        }
        sample
    }
}

/// 4-point cubic (Catmull-Rom) between `h[1]` and `h[2]` at `t` in [0, 1).
fn cubic(h: [i16; 4], t: f32) -> i16 {
    let [a, b, c, d] = h.map(f32::from);
    let v = b + 0.5 * t * (c - a + t * (2.0 * a - 5.0 * b + 4.0 * c - d + t * (3.0 * (b - c) + d - a)));
    v.round().clamp(-32767.0, 32767.0) as i16
}

/// Per-sample volume ramp from `from` to `to` over a frame.
fn ramp(from: u16, to: u16) -> [i32; FRAME] {
    std::array::from_fn(|i| i32::from(from) + (i32::from(to) - i32::from(from)) * i as i32 / FRAME as i32)
}

/// Bus buffers: L, R, S of main, aux A, aux B, aux C.
type Buses = [[[i32; FRAME]; 3]; 4];

pub struct Ax {
    buses: Buses,
    /// The output command's last volume and the aux returns' (for ramps).
    pub last_main_volume: u16,
    pub last_aux_volume: [u16; 3],
    /// Mix the aux buses back after the CPU's effect callbacks. Off while
    /// `hle-os` answers those callbacks at once: their buffers then hold
    /// the dry signal, and mixing it back would double it 3 ms late.
    pub aux_return: bool,
    /// One-shot voices that played to their end.
    pub voices_ended: u64,
    pub frames: u64,
    /// The player's volumes for music, sound effects and menu sounds
    /// ([`Kind`]), Q15: 0x8000 leaves a voice as the game mixes it. While
    /// all three are 0x8000 no voice is looked at and the mix is the game's
    /// own, bit for bit.
    pub gains: [u16; 3],
}

impl Default for Ax {
    fn default() -> Self {
        Self { buses: [[[0; FRAME]; 3]; 4], last_main_volume: 0x8000, last_aux_volume: [0x8000; 3], aux_return: false, voices_ended: 0, frames: 0, gains: [0x8000; 3] }
    }
}

/// What a voice plays, for the volume sliders.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Music = 0,
    Effects = 1,
    Menu = 2,
}

/// The way from an AX voice to the sound it plays, through `nw4r::snd`
/// (USA rev 1 addresses, after WitherBlitz's NSMB-Wii-Recompilation's
/// `ax_voice_tags`): each level's callback is checked before its data is
/// trusted, and a voice whose chain doesn't hold is told apart by the
/// mixer's own rule (a stream is music, the rest effects).
struct VoiceTags {
    /// `__AXPB` (the DSP's parameter blocks), `__AXVPB` (the CPU's voices)
    /// and the voice count.
    pbs: u32,
    vpbs: u32,
    count: u32,
    /// The sound archive player's sound players and their count.
    players: u32,
    player_count: u32,
    /// The callbacks below, at the running version's addresses.
    callbacks: [u32; 6],
}

impl VoiceTags {
    const AX_PB: E1 = E1(0x8042_A608);
    const AX_VPB: E1 = E1(0x8042_A610);
    const AX_MAX_VOICES: E1 = E1(0x8042_A614);
    const PB_SIZE: u32 = 0x140;
    const VPB_SIZE: u32 = 0x168;
    /// `AxVoice::VoiceCallback` (VPB +0x10; +0x14 the AxVoice).
    const AX_VOICE_CALLBACK: E1 = E1(0x8026_26F0);
    /// `Voice::AxVoiceCallbackFunc` (AxVoice +0x38; +0x3C the Voice).
    const VOICE_AX_CALLBACK: E1 = E1(0x8028_1EF0);
    /// The Voice's player callback (+0x94; +0x98 its data).
    const STRM_VOICE_CALLBACK: E1 = E1(0x8027_EEF0);
    const CHANNEL_VOICE_CALLBACK: E1 = E1(0x8026_8910);
    /// The Channel's callback (+0xC0; +0xC4 its data).
    const WSD_CHANNEL_CALLBACK: E1 = E1(0x8028_5500);
    const SEQ_CHANNEL_CALLBACK: E1 = E1(0x8027_3B50);
    /// `SndAudioMgr::sInstance`; +0x5BC its SoundArchivePlayer, whose +0x3C
    /// is the sound-player count and +0x40 the array (0x64 bytes each).
    const SND_AUDIO_MGR: E1 = E1(0x8042_A488);
    /// Players 16-21 play music (`PLAYER_BGM_*`).
    const MUSIC_PLAYERS: std::ops::RangeInclusive<u32> = 16..=21;

    fn read(mem: &Mem) -> Option<Self> {
        let manager = r32(mem, Self::SND_AUDIO_MGR.va());
        let archive = if manager == 0 { 0 } else { r32(mem, manager + 0x5BC) };
        if archive == 0 {
            return None;
        }
        let callbacks = [Self::AX_VOICE_CALLBACK, Self::VOICE_AX_CALLBACK, Self::STRM_VOICE_CALLBACK, Self::CHANNEL_VOICE_CALLBACK, Self::WSD_CHANNEL_CALLBACK, Self::SEQ_CHANNEL_CALLBACK].map(E1::va);
        let tags = Self { pbs: r32(mem, Self::AX_PB.va()), vpbs: r32(mem, Self::AX_VPB.va()), count: r32(mem, Self::AX_MAX_VOICES.va()).min(MAX_VOICES as u32), players: r32(mem, archive + 0x40), player_count: r32(mem, archive + 0x3C), callbacks };
        (archive != 0 && tags.vpbs != 0 && tags.players != 0).then_some(tags)
    }

    /// What the voice whose parameter block is at `pb_addr` plays.
    fn kind(&self, mem: &Mem, pb_addr: u32, stream: bool) -> Kind {
        let own_rule = if stream { Kind::Music } else { Kind::Effects };
        let (pb, base) = (pb_addr & 0x1FFF_FFFF, self.pbs & 0x1FFF_FFFF);
        if pb < base || (pb - base) % Self::PB_SIZE != 0 || (pb - base) / Self::PB_SIZE >= self.count {
            return own_rule;
        }
        let index = (pb - base) / Self::PB_SIZE;
        let vpb = self.vpbs + index * Self::VPB_SIZE;
        if r32(mem, vpb + 0x18) != index {
            return own_rule;
        }
        match self.owner(mem, vpb) {
            Some((id, player)) if player == 0 || player == 11 || (player == 1 && matches!(id, 160..=163 | 1498..=1500 | 1615..=1617 | 1672..=1675 | 1678..=1680)) => Kind::Menu,
            Some((_, player)) if Self::MUSIC_PLAYERS.contains(&player) => Kind::Music,
            Some(_) => Kind::Effects,
            None => own_rule,
        }
    }

    /// The sound playing on the voice at `vpb`: its id and sound player.
    fn owner(&self, mem: &Mem, vpb: u32) -> Option<(u32, u32)> {
        let nonzero = |v: u32| (v != 0).then_some(v);
        let [ax_voice_callback, voice_ax_callback, strm, channel, wsd, seq] = self.callbacks;
        if r32(mem, vpb + 0x10) != ax_voice_callback {
            return None;
        }
        let ax_voice = nonzero(r32(mem, vpb + 0x14))?;
        if r32(mem, ax_voice + 0x38) != voice_ax_callback {
            return None;
        }
        let voice = nonzero(r32(mem, ax_voice + 0x3C))?;
        let (callback, data) = (r32(mem, voice + 0x94), nonzero(r32(mem, voice + 0x98))?);
        let player_object = match callback {
            // The data is one of the StrmPlayer's channels (+0xB58, 0x38
            // each, +4 the Voice): the one whose player then checks out.
            _ if callback == strm => {
                return (0..8u32).find_map(|channel| {
                    let player = data.checked_sub(0xB58 + channel * 0x38)?;
                    let channels = r32(mem, player + 0x80C);
                    (channel < channels && channels <= 8 && r32(mem, data + 4) == voice).then(|| self.sound_of(mem, player)).flatten()
                });
            }
            _ if callback == channel => match r32(mem, data + 0xC0) {
                c if c == wsd => r32(mem, data + 0xC4),
                c if c == seq => r32(mem, r32(mem, data + 0xC4) + 0xC4),
                _ => return None,
            },
            _ => return None,
        };
        self.sound_of(mem, player_object)
    }

    /// The sound whose StrmPlayer, WsdPlayer or SeqPlayer is at `object`
    /// (+0x110 of the sound): its id (+0x9C) and which of the archive's
    /// sound players (+0x10) it plays on.
    fn sound_of(&self, mem: &Mem, object: u32) -> Option<(u32, u32)> {
        let sound = object.checked_sub(0x110)?;
        let player = r32(mem, sound + 0x10).checked_sub(self.players)?;
        (player % 0x64 == 0 && player / 0x64 < self.player_count).then(|| (r32(mem, sound + 0x9C), player / 0x64))
    }
}

fn r16(mem: &Mem, a: u32) -> u16 {
    mem.ram(a, 2).map_or(0, |b| u16::from_be_bytes([b[0], b[1]]))
}

fn r32(mem: &Mem, a: u32) -> u32 {
    mem.ram(a, 4).map_or(0, |b| u32::from_be_bytes(b.try_into().unwrap()))
}

impl Ax {
    /// Runs the command list at `list`.
    pub fn frame(&mut self, list: u32, mem: &mut Mem) {
        self.frames += 1;
        let mut at = list;
        for _ in 0..64 {
            let cmd = r16(mem, at);
            let arg = |k: u32| r16(mem, at + 2 + 2 * k);
            let addr = |k: u32| r32(mem, at + 2 + 2 * k);
            let words = match cmd {
                0x00 => {
                    self.buses = [[[0; FRAME]; 3]; 4];
                    2
                }
                0x01 | 0x02 => {
                    let a = addr(0);
                    self.add_to_lr(mem, a, cmd == 0x02);
                    2
                }
                0x03 => {
                    let (a, b) = (addr(0), addr(2));
                    self.add_to_lr(mem, a, false);
                    self.add_to_lr(mem, b, true);
                    4
                }
                0x04 => {
                    let head = addr(0);
                    self.process(mem, head);
                    2
                }
                0x05..=0x07 => {
                    let (volume, write, read) = (arg(0), addr(1), addr(3));
                    self.mix_aux(mem, usize::from(cmd - 0x05), volume, write, read);
                    5
                }
                0x0A => 4,
                0x0B | 0x0C => {
                    let (volume, surround, lr) = (arg(0), addr(1), addr(3));
                    self.output(mem, volume, surround, lr);
                    5
                }
                0x0D => 8,
                _ => return, // 0x0E end, or a command this microcode never sends here
            };
            at += 2 + 2 * words;
        }
    }

    /// Adds (or subtracts) 96 big-endian `s32` at `addr` to main L and R.
    fn add_to_lr(&mut self, mem: &Mem, addr: u32, neg: bool) {
        let Some(b) = mem.ram(addr, FRAME * 4) else { return };
        for i in 0..FRAME {
            let v = i32::from_be_bytes(b[4 * i..4 * i + 4].try_into().unwrap());
            let v = if neg { -v } else { v };
            self.buses[0][0][i] += v;
            self.buses[0][1][i] += v;
        }
    }

    fn process(&mut self, mem: &mut Mem, head: u32) {
        let tags = if self.gains == [0x8000; 3] { None } else { VoiceTags::read(mem) };
        let mut addr = head;
        for _ in 0..MAX_VOICES {
            if addr == 0 {
                break;
            }
            let Some(mut p) = read_pb(mem, addr) else { break };
            if p.get(pb::RUNNING) == 1 {
                let gain = match &tags {
                    Some(t) => self.gains[t.kind(mem, addr, p.get(pb::IS_STREAM) != 0) as usize],
                    None if self.gains == [0x8000; 3] => 0x8000,
                    // The sound library isn't up yet: the mixer's own rule.
                    None => self.gains[if p.get(pb::IS_STREAM) != 0 { Kind::Music } else { Kind::Effects } as usize],
                };
                if self.voice(mem, &mut p, gain) {
                    self.voices_ended += 1;
                }
                write_pb(mem, addr, &p);
            }
            addr = p.get32(pb::NEXT);
        }
    }

    /// One frame of one running voice, at the player's `gain` (Q15) for
    /// what it plays; true if a one-shot voice ended.
    fn voice(&mut self, mem: &Mem, p: &mut Pb, gain: u16) -> bool {
        let mut acc = Accel::load(p);
        let ratio = p.get32(pb::RATIO);
        let src = p.get(pb::SRC_TYPE);
        let mut frac = u32::from(p.get(pb::FRAC));
        let mut hist: [i16; 4] = std::array::from_fn(|i| p.get_i(pb::LAST_SAMPLES + 2 * i as u32));
        let mut s = [0i32; FRAME];
        for out in s.iter_mut() {
            let t = frac as f32 / 65536.0;
            *out = i32::from(match src {
                0 => cubic(hist, t),
                1 => (f32::from(hist[1]) + (f32::from(hist[2]) - f32::from(hist[1])) * t) as i16,
                _ => hist[2],
            });
            frac += if src >= 2 { 0x1_0000 } else { ratio };
            while frac >= 0x1_0000 {
                hist.rotate_left(1);
                hist[3] = acc.read(mem);
                frac -= 0x1_0000;
            }
        }
        acc.store(p);
        p.set(pb::FRAC, frac as u16);
        for (i, h) in hist.iter().enumerate() {
            p.set(pb::LAST_SAMPLES + 2 * i as u32, *h as u16);
        }

        // Volume envelope.
        let (mut vol, delta) = (p.get(pb::VOL_ENV), p.get(pb::VOL_ENV + 2));
        for v in s.iter_mut() {
            *v = ((*v * i32::from(vol)) >> 15).clamp(-32767, 32767);
            vol = vol.wrapping_add(delta);
        }
        p.set(pb::VOL_ENV, vol);
        if gain != 0x8000 {
            for v in s.iter_mut() {
                *v = (*v * i32::from(gain)) >> 15;
            }
        }

        // One-pole low-pass: y = (a0 x + b0 y[-1]) >> 15.
        if p.get(pb::LPF) != 0 {
            let (mut y, a0, b0) = (i32::from(p.get_i(pb::LPF + 2)), i32::from(p.get(pb::LPF + 4)), i32::from(p.get(pb::LPF + 6)));
            for v in s.iter_mut() {
                y = ((a0 * *v + b0 * y) >> 15).clamp(-32767, 32767);
                *v = y;
            }
            p.set(pb::LPF + 2, y as u16);
        }
        // Biquad, Q14 coefficients with the feedback terms added (NSMBW's
        // high-pass: b = 0.84, -1.68, 0.84; a = 1.765, -0.799).
        if p.get(pb::BIQUAD) != 0 {
            let g = |k: u32| i32::from(p.get_i(pb::BIQUAD + 2 * k));
            let (mut x1, mut x2, mut y1, mut y2) = (g(1), g(2), g(3), g(4));
            let (b0, b1, b2, a1, a2) = (g(5), g(6), g(7), g(8), g(9));
            for v in s.iter_mut() {
                let y = ((b0 * *v + b1 * x1 + b2 * x2 + a1 * y1 + a2 * y2) >> 14).clamp(-32767, 32767);
                (x2, x1, y2, y1) = (x1, *v, y1, y);
                *v = y;
            }
            for (k, v) in [(1, x1), (2, x2), (3, y1), (4, y2)] {
                p.set(pb::BIQUAD + 2 * k, v as u16);
            }
        }

        // The buses, with per-sample volume ramps.
        let mc = p.get32(pb::MIXER_CONTROL);
        for (bus, &bit) in BUS_BITS.iter().enumerate() {
            for ch in 0..3 {
                let (enable, ramp_bit) = match ch {
                    0 => (bit, bit + 2),
                    1 => (bit + 1, bit + 2),
                    _ => (bit + 3, bit + 4),
                };
                if mc & (1 << enable) == 0 {
                    continue;
                }
                // Volume word index: L/R pairs per bus first, then S per bus.
                let vi = if ch < 2 { 4 * bus + 2 * ch } else { 16 + 2 * bus };
                let vaddr = pb::MIXER + 2 * vi as u32;
                let mut vol = p.get(vaddr);
                let delta = if mc & (1 << ramp_bit) != 0 { p.get(vaddr + 2) } else { 0 };
                let out = &mut self.buses[bus][ch];
                let mut last = 0;
                for (o, &x) in out.iter_mut().zip(&s) {
                    last = ((x * i32::from(vol)) >> 15).clamp(-32767, 32767);
                    *o += last;
                    vol = vol.wrapping_add(delta);
                }
                p.set(vaddr, vol);
                // De-pop: the last sample of each bus channel.
                let di = if ch < 2 { 4 * ch + bus } else { 8 + bus };
                p.set(pb::DPOP + 2 * di as u32, last as u16);
            }
        }

        if acc.ended {
            p.set(pb::RUNNING, 0);
            return true;
        }
        false
    }

    /// Aux bus `aux` (A, B, C) to the CPU at `write`; with
    /// [`Ax::aux_return`], what the effect callback left at `read` mixed into
    /// main at `volume`.
    #[allow(clippy::needless_range_loop)] // samples index the ramp and the guest buffer together
    fn mix_aux(&mut self, mem: &mut Mem, aux: usize, volume: u16, write: u32, read: u32) {
        let bus = aux + 1;
        if write != 0 {
            if let Some(b) = mem.ram_mut(write, 3 * FRAME * 4) {
                for ch in 0..3 {
                    for i in 0..FRAME {
                        let at = 4 * (ch * FRAME + i);
                        b[at..at + 4].copy_from_slice(&self.buses[bus][ch][i].to_be_bytes());
                    }
                }
            }
        }
        let r = ramp(self.last_aux_volume[aux], volume);
        self.last_aux_volume[aux] = volume;
        if !self.aux_return {
            return;
        }
        let Some(b) = mem.ram(read, 3 * FRAME * 4) else { return };
        for ch in 0..3 {
            for i in 0..FRAME {
                let at = 4 * (ch * FRAME + i);
                let v = i64::from(i32::from_be_bytes(b[at..at + 4].try_into().unwrap()));
                self.buses[0][ch][i] += ((v * i64::from(r[i])) >> 15) as i32;
            }
        }
    }

    /// Main S as `s32` to `surround`, and main L/R at `volume` as the 16-bit
    /// stereo frame (right, then left, big-endian) to `lr`.
    fn output(&mut self, mem: &mut Mem, volume: u16, surround: u32, lr: u32) {
        if let Some(b) = mem.ram_mut(surround, FRAME * 4) {
            for i in 0..FRAME {
                b[4 * i..4 * i + 4].copy_from_slice(&self.buses[0][2][i].to_be_bytes());
            }
        }
        let r = ramp(self.last_main_volume, volume);
        self.last_main_volume = volume;
        if let Some(b) = mem.ram_mut(lr, FRAME * 4) {
            for i in 0..FRAME {
                let ch = |c: usize| (((i64::from(self.buses[0][c][i]) * i64::from(r[i])) >> 15).clamp(-32767, 32767) as i16).to_be_bytes();
                b[4 * i..4 * i + 2].copy_from_slice(&ch(1));
                b[4 * i + 2..4 * i + 4].copy_from_slice(&ch(0));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w16(mem: &mut Mem, a: u32, v: u16) {
        mem.write_u16(a, v).unwrap();
    }
    fn w32(mem: &mut Mem, a: u32, v: u32) {
        mem.write_u32(a, v).unwrap();
    }

    /// A PCM16 voice at 1:1, no SRC, full volume to main L only: the output
    /// frame carries its samples on the left, silence on the right.
    #[test]
    fn pcm16_voice_reaches_the_left_channel() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        let (list, pbs, samples, out, surround) = (0x8000_1000u32, 0x8000_2000u32, 0x8000_4000u32, 0x8000_6000u32, 0x8000_7000u32);
        for i in 0..200u32 {
            w16(&mut mem, samples + 2 * i, (i as u16) * 100);
        }
        w16(&mut mem, pbs + pb::RUNNING, 1);
        w16(&mut mem, pbs + pb::SRC_TYPE, 2);
        w32(&mut mem, pbs + pb::MIXER_CONTROL, 1); // main L
        w16(&mut mem, pbs + pb::MIXER, 0x8000);
        w16(&mut mem, pbs + pb::VOL_ENV, 0x8000);
        w16(&mut mem, pbs + pb::FORMAT, FMT_PCM16);
        w32(&mut mem, pbs + pb::END_ADDR, (samples & 0x1FFF_FFFF) / 2 + 199);
        w32(&mut mem, pbs + pb::CUR_ADDR, (samples & 0x1FFF_FFFF) / 2);
        w32(&mut mem, pbs + pb::RATIO, 0x1_0000);
        for (i, w) in [0x0000, 0x8000, 0x3000, 0x0004, (pbs >> 16) as u16, pbs as u16, 0x000B, 0x8000, (surround >> 16) as u16, surround as u16, (out >> 16) as u16, out as u16, 0x000E].iter().enumerate() {
            w16(&mut mem, list + 2 * i as u32, *w);
        }
        let mut ax = Ax::default();
        ax.frame(list, &mut mem);
        // Without SRC, output i is input i - 2 (the history starts with
        // zeros); right channel first.
        let at = |i: u32| (r16(&mem, out + 4 * i) as i16, r16(&mem, out + 4 * i + 2) as i16);
        assert_eq!(at(10), (0, 800));
        assert_eq!(at(95), (0, 9300));
        assert_eq!(mem.read_u32(pbs + pb::CUR_ADDR).unwrap(), (samples & 0x1FFF_FFFF) / 2 + 96);
        // It runs out of samples on the third frame and stops, its address
        // back at its loop address (0 here), as NW4R expects.
        ax.frame(list, &mut mem);
        ax.frame(list, &mut mem);
        assert_eq!(mem.read_u16(pbs + pb::RUNNING).unwrap(), 0);
        assert_eq!(mem.read_u32(pbs + pb::CUR_ADDR).unwrap(), 0);
        assert_eq!(ax.voices_ended, 1);

        // The Sound Effects volume at half: with no sound library to ask,
        // a voice that isn't a stream is an effect, and plays at half.
        w16(&mut mem, pbs + pb::RUNNING, 1);
        w16(&mut mem, pbs + pb::VOL_ENV, 0x8000);
        w32(&mut mem, pbs + pb::CUR_ADDR, (samples & 0x1FFF_FFFF) / 2);
        for i in 0..4 {
            w16(&mut mem, pbs + pb::LAST_SAMPLES + 2 * i, 0);
        }
        let mut ax = Ax { gains: [0x8000, 0x4000, 0x8000], ..Ax::default() };
        ax.frame(list, &mut mem);
        let at = |i: u32| (r16(&mem, out + 4 * i) as i16, r16(&mem, out + 4 * i + 2) as i16);
        assert_eq!(at(10), (0, 400));
        assert_eq!(at(95), (0, 4650));
    }

    /// ADPCM: a frame whose nibbles are all +1 at scale 1 with no
    /// prediction decodes to a run of ones, and the header is skipped.
    #[test]
    fn adpcm_decodes_and_skips_headers() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        let data = 0x8000_4000u32;
        for f in 0..4 {
            w16(&mut mem, data + 8 * f, 0x0111); // header: predictor 0, scale 1; then nibbles 1, 1
            for k in 1..4 {
                w16(&mut mem, data + 8 * f + 2 * k, 0x1111);
            }
        }
        let mut p = Pb([0; pb::SIZE / 2]);
        p.set(pb::FORMAT, FMT_ADPCM);
        let nib = (data & 0x1FFF_FFFF) * 2;
        p.set32(pb::CUR_ADDR, nib + 2);
        p.set32(pb::END_ADDR, nib + 16 * 4 - 1);
        p.set(pb::PRED_SCALE, 0x01);
        let mut acc = Accel::load(&p);
        let got: Vec<i16> = (0..20).map(|_| acc.read(&mem)).collect();
        assert!(got.iter().all(|&v| v == 2), "{got:?}");
        // 20 nibbles: 14 in the first frame, 6 in the second (after its header).
        assert_eq!(acc.cur, nib + 16 + 2 + 6);
    }
}
