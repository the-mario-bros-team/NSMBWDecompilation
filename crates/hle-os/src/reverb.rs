//! A native `nw4r::snd::FxReverbHi::UpdateBuffer` (0x8026C210), ported from
//! WitherBlitz's NSMB-Wii-Recompilation (`runtime/src/hle/audio/
//! axfx_reverb_native.cpp`). Interpreted, the reverb (the
//! `AXFXReverbHiExpCallback` network) cost about a quarter of guest time.
//!
//! The network per sample and channel: early reflections (three taps),
//! optional pre-delay, three combs, two shared all-passes, damping and a
//! per-channel all-pass, then the crosstalk mix. The effect's state stays in
//! guest memory in the game's own layout (`AXFX_REVERBHI_EXP` at
//! `this + 0x44`), so native and guest code never disagree about it. Host
//! `f32` maths rounds differently from the Gekko's in the last bits, so the
//! output is not bit-identical to the interpreted callback's; every machine
//! of an online session runs this same code, so they stay in step.

use game_version::E1;
use guest_mem::Mem;

pub const UPDATE_BUFFER: E1 = E1(0x8026_C210);

const SAMPLES: u32 = 96;
const STATE_BYTES: usize = 0x148;
/// The callback's float constants in `.sdata2` (read from the game).
const CONSTANTS: E1 = E1(0x8042_DF3C);

fn r32(mem: &Mem, a: u32) -> Option<u32> {
    mem.ram(a, 4).map(|b| u32::from_be_bytes(b.try_into().unwrap()))
}

/// A guest array of `count` 32-bit words: its address if it is plain RAM.
fn array(mem: &Mem, a: u32, count: u32) -> Option<u32> {
    (a != 0 && count != 0 && mem.ram(a, 4 * count as usize).is_some()).then_some(a)
}

/// Everything one block reads and writes.
struct Buffers {
    state: u32,
    input: [u32; 3],
    bus_in: [Option<u32>; 3],
    bus_out: [Option<u32>; 3],
    early_len: u32,
    pre_len: u32,
    comb_len: [u32; 3],
    ap_len: [u32; 2],
    last_len: [u32; 3],
    early: [u32; 3],
    pre: [Option<u32>; 3],
    comb: [[u32; 3]; 3],
    ap: [[u32; 2]; 3],
    last: [u32; 3],
}

/// `None` when anything is not plain RAM or an index is out of range (the
/// game's own code runs then).
fn gather(mem: &Mem, r: u32, input: [u32; 3]) -> Option<Buffers> {
    mem.ram(r, STATE_BYTES)?;
    let s = |o: u32| r32(mem, r + o);
    let mut b = Buffers {
        state: r,
        input: [0; 3],
        bus_in: [None; 3],
        bus_out: [None; 3],
        early_len: s(0x18)?,
        pre_len: s(0x3C)?,
        comb_len: [s(0x74)?, s(0x78)?, s(0x7C)?],
        ap_len: [s(0xB8)?, s(0xBC)?],
        last_len: [s(0xE0)?, s(0xE4)?, s(0xE8)?],
        early: [0; 3],
        pre: [None; 3],
        comb: [[0; 3]; 3],
        ap: [[0; 2]; 3],
        last: [0; 3],
    };
    let (bus_in, bus_out) = (s(0x138)?, s(0x13C)?);
    for ch in 0..3u32 {
        let c = ch as usize;
        b.input[c] = array(mem, input[c], SAMPLES)?;
        if bus_in != 0 {
            b.bus_in[c] = Some(array(mem, r32(mem, bus_in + 4 * ch)?, SAMPLES)?);
        }
        if bus_out != 0 {
            b.bus_out[c] = Some(array(mem, r32(mem, bus_out + 4 * ch)?, SAMPLES)?);
        }
        b.early[c] = array(mem, s(4 * ch)?, b.early_len)?;
        if b.pre_len != 0 {
            b.pre[c] = Some(array(mem, s(0x2C + 4 * ch)?, b.pre_len)?);
        }
        for k in 0..3u32 {
            b.comb[c][k as usize] = array(mem, s(0x44 + 12 * ch + 4 * k)?, b.comb_len[k as usize])?;
        }
        for k in 0..2u32 {
            b.ap[c][k as usize] = array(mem, s(0x98 + 8 * ch + 4 * k)?, b.ap_len[k as usize])?;
        }
        b.last[c] = array(mem, s(0xC8 + 4 * ch)?, b.last_len[c])?;
        if s(0xD4 + 4 * ch)? >= b.last_len[c] {
            return None;
        }
    }
    for k in 0..3u32 {
        if s(0x0C + 4 * k)? >= b.early_len || s(0x68 + 4 * k)? >= b.comb_len[k as usize] {
            return None;
        }
    }
    if s(0xB0)? >= b.ap_len[0] || s(0xB4)? >= b.ap_len[1] || (b.pre_len != 0 && s(0x38)? >= b.pre_len) {
        return None;
    }
    Some(b)
}

/// fctiwz: toward zero, saturating, NaN to `i32::MIN`.
fn to_int(v: f32) -> i32 {
    if v.is_nan() {
        i32::MIN
    } else {
        v as i32 // Rust's float-to-int cast saturates
    }
}

#[allow(clippy::needless_range_loop)] // channels and taps index several parallel arrays
fn run(mem: &mut Mem, b: &Buffers, k: [f32; 4]) {
    let [_zero, one, cross_k, mix_k] = k;
    let rd = |mem: &Mem, a: u32| r32(mem, a).unwrap_or(0);
    let rf = |mem: &Mem, a: u32| f32::from_bits(rd(mem, a));
    let wr = |mem: &mut Mem, a: u32, v: u32| {
        let _ = mem.write_u32(a, v);
    };
    let wf = |mem: &mut Mem, a: u32, v: f32| {
        let _ = mem.write_u32(a, v.to_bits());
    };
    let r = b.state;
    let mut early_idx = [rd(mem, r + 0x0C), rd(mem, r + 0x10), rd(mem, r + 0x14)];
    let mut pre_idx = rd(mem, r + 0x38);
    let mut comb_idx = [rd(mem, r + 0x68), rd(mem, r + 0x6C), rd(mem, r + 0x70)];
    let mut ap_idx = [rd(mem, r + 0xB0), rd(mem, r + 0xB4)];
    let mut last_idx: [u32; 3] = std::array::from_fn(|c| rd(mem, r + 0xD4 + 4 * c as u32));
    let mut lowpass: [f32; 3] = std::array::from_fn(|c| rf(mem, r + 0xFC + 4 * c as u32));
    let tap = [rf(mem, r + 0x20), rf(mem, r + 0x24), rf(mem, r + 0x28)];
    let comb_coef = [rf(mem, r + 0x8C), rf(mem, r + 0x90), rf(mem, r + 0x94)];
    let allpass = rf(mem, r + 0xF8);
    let damping = rf(mem, r + 0x108);
    let undamped = one - damping;
    let cross = cross_k * rf(mem, r + 0x134);
    let mix = mix_k * rf(mem, r + 0x12C);
    let (gain, bus_gain) = (rf(mem, r + 0x140), rf(mem, r + 0x144));

    for i in 0..SAMPLES {
        let mut out = [0f32; 3];
        for ch in 0..3 {
            let mut sample = rd(mem, b.input[ch] + 4 * i);
            if let Some(bi) = b.bus_in[ch] {
                sample = sample.wrapping_add(rd(mem, bi + 4 * i));
            }
            let x = sample as i32 as f32;
            let e = b.early[ch];
            let (e0, e1, e2) = (rf(mem, e + 4 * early_idx[0]), rf(mem, e + 4 * early_idx[1]), rf(mem, e + 4 * early_idx[2]));
            wf(mem, e + 4 * early_idx[2], x);
            let reflections = tap[2] * e2 + (tap[0] * e0 + tap[1] * e1);
            let mut delayed = x;
            if let Some(p) = b.pre[ch] {
                delayed = rf(mem, p + 4 * pre_idx);
                wf(mem, p + 4 * pre_idx, x);
            }
            let mut acc = 0f32;
            for k in 0..3 {
                let slot = b.comb[ch][k] + 4 * comb_idx[k];
                let v = rf(mem, slot);
                acc += v;
                wf(mem, slot, delayed + v * comb_coef[k]);
            }
            let mut a = acc;
            for k in 0..2 {
                let slot = b.ap[ch][k] + 4 * ap_idx[k];
                let v = rf(mem, slot);
                let t = a + v * allpass;
                wf(mem, slot, t);
                a = v - t * allpass;
            }
            lowpass[ch] = undamped * a + damping * lowpass[ch];
            let slot = b.last[ch] + 4 * last_idx[ch];
            let v = rf(mem, slot);
            let t = lowpass[ch] + v * allpass;
            wf(mem, slot, t);
            last_idx[ch] += 1;
            if last_idx[ch] >= b.last_len[ch] {
                last_idx[ch] = 0;
            }
            out[ch] = (v - t * allpass) * cross + reflections;
        }
        let mixed = [out[0] + (out[1] + out[2]) * mix, out[1] + (out[0] + out[2]) * mix, out[2] + (out[0] + out[1]) * mix];
        for ch in 0..3 {
            wr(mem, b.input[ch] + 4 * i, to_int(mixed[ch] * gain) as u32);
            if let Some(bo) = b.bus_out[ch] {
                wr(mem, bo + 4 * i, to_int(mixed[ch] * bus_gain) as u32);
            }
        }
        for k in 0..3 {
            early_idx[k] += 1;
            if early_idx[k] >= b.early_len {
                early_idx[k] = 0;
            }
            comb_idx[k] += 1;
            if comb_idx[k] >= b.comb_len[k] {
                comb_idx[k] = 0;
            }
        }
        if b.pre_len != 0 {
            pre_idx += 1;
            if pre_idx >= b.pre_len {
                pre_idx = 0;
            }
        }
        for k in 0..2 {
            ap_idx[k] += 1;
            if ap_idx[k] >= b.ap_len[k] {
                ap_idx[k] = 0;
            }
        }
    }
    for k in 0..3u32 {
        wr(mem, r + 0x0C + 4 * k, early_idx[k as usize]);
        wr(mem, r + 0x68 + 4 * k, comb_idx[k as usize]);
    }
    wr(mem, r + 0x38, pre_idx);
    wr(mem, r + 0xB0, ap_idx[0]);
    wr(mem, r + 0xB4, ap_idx[1]);
    for ch in 0..3u32 {
        wr(mem, r + 0xD4 + 4 * ch, last_idx[ch as usize]);
        wf(mem, r + 0xFC + 4 * ch, lowpass[ch as usize]);
    }
}

/// `FxReverbHi::UpdateBuffer(this, channels, buffers, size, format,
/// sampleRate, outputMode)` with `this` in r3, `buffers` in r5 and
/// `outputMode` in r8. True when it was answered here (return to `lr`);
/// false to run the game's own code.
pub fn update_buffer(mem: &mut Mem, this: u32, buffers: u32, output_mode: u32) -> bool {
    let (Some(active), Some(in0), Some(in1), Some(in2)) = (r32(mem, this + 0xC), r32(mem, buffers), r32(mem, buffers + 4), r32(mem, buffers + 8)) else {
        return false;
    };
    let Some(k) = (0..4).map(|i| r32(mem, CONSTANTS.va() + 4 * i).map(f32::from_bits)).collect::<Option<Vec<_>>>() else {
        return false;
    };
    // Off, or Dolby Pro Logic II output (its own effect): nothing to do.
    if active >> 24 == 0 || output_mode == 2 {
        return true;
    }
    let Some(flags) = r32(mem, this + 0x44 + 0x10C) else { return false };
    if flags != 0 {
        // The effect was just switched: skip a block, as the game does.
        let _ = mem.write_u32(this + 0x44 + 0x10C, flags & !2);
        return true;
    }
    let Some(b) = gather(mem, this + 0x44, [in0, in1, in2]) else { return false };
    run(mem, &b, [k[0], k[1], k[2], k[3]]);
    true
}
