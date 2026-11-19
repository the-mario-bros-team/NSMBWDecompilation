//! Broadway floating-point details that differ from plain IEEE arithmetic.

const SIGN: u64 = 0x8000_0000_0000_0000;
const FRAC: u64 = 0x000F_FFFF_FFFF_FFFF;

/// `lfs` / `psq_l` (float): single bits to double bits. Bit-exact, including
/// NaN payloads and denormals, because games copy raw words through FPRs.
#[inline(always)]
pub fn single_to_double(value: u32) -> u64 {
    let x = value as u64;
    let exp = (x >> 23) & 0xFF;
    if exp > 0 && exp < 255 {
        let y = (exp >> 7) ^ 1;
        let z = (y << 61) | (y << 60) | (y << 59);
        ((x & 0xC000_0000) << 32) | z | ((x & 0x3FFF_FFFF) << 29)
    } else if exp == 0 && x & 0x007F_FFFF != 0 {
        denormal_to_double(x)
    } else {
        let y = exp >> 7;
        let z = (y << 61) | (y << 60) | (y << 59);
        ((x & 0xC000_0000) << 32) | z | ((x & 0x3FFF_FFFF) << 29)
    }
}

/// [`single_to_double`] of a denormal single.
#[cold]
#[inline(never)]
fn denormal_to_double(x: u64) -> u64 {
    let mut frac = x & 0x007F_FFFF;
    let mut e: u64 = 1023 - 126;
    loop {
        frac <<= 1;
        e -= 1;
        if frac & 0x0080_0000 != 0 {
            break;
        }
    }
    ((x & 0x8000_0000) << 32) | (e << 52) | ((frac & 0x007F_FFFF) << 29)
}

/// `stfs` / `psq_st` (float): double bits to single bits by truncation, the
/// way the hardware does it (no rounding).
#[inline(always)]
pub fn double_to_single(x: u64) -> u32 {
    let exp = ((x >> 52) & 0x7FF) as u32;
    if exp > 896 || (x & !SIGN) == 0 {
        (((x >> 32) & 0xC000_0000) | ((x >> 29) & 0x3FFF_FFFF)) as u32
    } else if exp >= 874 {
        let mut t = (0x8000_0000 | ((x & FRAC) >> 21)) as u32;
        t >>= 905 - exp;
        t | ((x >> 32) & 0x8000_0000) as u32
    } else {
        (((x >> 32) & 0xC000_0000) | ((x >> 29) & 0x3FFF_FFFF)) as u32
    }
}

/// Rounds a double result to single precision (results of `fadds`, `ps_add`, ...).
#[inline]
pub fn round_single(v: f64) -> f64 {
    v as f32 as f64
}

/// The multiplier input (frC) of single-precision multiplies keeps only 25
/// significant bits on Broadway (round half up on bit 27).
#[inline]
pub fn force25(v: f64) -> f64 {
    if !v.is_finite() {
        return v;
    }
    let b = v.to_bits();
    f64::from_bits((b & 0xFFFF_FFFF_F800_0000).wrapping_add(b & 0x0800_0000))
}

/// `fctiw` / `fctiwz`: convert to a 32-bit integer with saturation; the FPR
/// receives `0xFFF8_0000_xxxx_xxxx`. `mode` is FPSCR[RN].
#[inline]
pub fn to_int_bits(v: f64, mode: u32) -> u64 {
    let value: u32 = if v.is_nan() {
        0x8000_0000
    } else {
        let r = match mode & 3 {
            0 => v.round_ties_even(),
            1 => v.trunc(),
            2 => v.ceil(),
            _ => v.floor(),
        };
        if r >= 2_147_483_648.0 {
            0x7FFF_FFFF
        } else if r < -2_147_483_648.0 {
            0x8000_0000
        } else {
            r as i32 as u32
        }
    };
    let mut bits = 0xFFF8_0000_0000_0000 | value as u64;
    if value == 0 && v.is_sign_negative() {
        bits |= 0x1_0000_0000;
    }
    bits
}

/// Condition-register nibble for a floating compare: LT 8, GT 4, EQ 2, unordered 1.
#[inline]
pub fn compare(a: f64, b: f64) -> u32 {
    if a.is_nan() || b.is_nan() {
        1
    } else if a < b {
        8
    } else if a > b {
        4
    } else {
        2
    }
}

/// Hardware reciprocal estimate (`fres`, `ps_res`).
// Approximation: exact reciprocal rounded to single. Broadway uses a 12-bit
// table estimate; replace with the exact table before relying on bit-identical
// physics (docs/PORT_PLAN.md, section 2.2).
pub fn reciprocal_estimate(v: f64) -> f64 {
    round_single(1.0 / v)
}

/// Hardware reciprocal square-root estimate (`frsqrte`, `ps_rsqrte`).
// Approximation: see `reciprocal_estimate`.
pub fn rsqrt_estimate(v: f64) -> f64 {
    1.0 / v.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_round_trip_is_bit_exact() {
        for bits in [0u32, 0x8000_0000, 0x3F80_0000, 0x7F80_0000, 0x7FC0_1234, 0x7F80_0001, 0x0000_0001, 0x8040_0000, 0x0123_4567] {
            assert_eq!(double_to_single(single_to_double(bits)), bits, "{bits:#010x}");
        }
        assert_eq!(f64::from_bits(single_to_double(0x3F80_0000)), 1.0);
        assert_eq!(f64::from_bits(single_to_double(0x0000_0001)), f32::from_bits(1) as f64);
    }

    #[test]
    fn store_single_truncates() {
        // 1 + 2^-30 is not representable; stfs truncates rather than rounds.
        let v = 1.0f64 + (2f64).powi(-24) * 1.5;
        assert_eq!(double_to_single(v.to_bits()), 0x3F80_0000);
    }

    #[test]
    fn convert_to_int() {
        assert_eq!(to_int_bits(2.5, 0) as u32, 2);
        assert_eq!(to_int_bits(3.5, 0) as u32, 4);
        assert_eq!(to_int_bits(-2.7, 1) as u32, (-2i32) as u32);
        assert_eq!(to_int_bits(3e10, 1) as u32, 0x7FFF_FFFF);
        assert_eq!(to_int_bits(-3e10, 1) as u32, 0x8000_0000);
        assert_eq!(to_int_bits(f64::NAN, 1) as u32, 0x8000_0000);
        assert_eq!(to_int_bits(5.0, 1) >> 32, 0xFFF8_0000);
    }

    #[test]
    fn force25_rounds_low_bits() {
        let v = f64::from_bits(0x3FF0_0000_0800_0000);
        assert_eq!(force25(v).to_bits(), 0x3FF0_0000_1000_0000);
        let w = f64::from_bits(0x3FF0_0000_07FF_FFFF);
        assert_eq!(force25(w).to_bits(), 0x3FF0_0000_0000_0000);
    }
}

/// Fused multiply-add `a * b + c` with a single rounding, as Broadway does.
/// Uses the host FMA instruction when there is one: without it,
/// `f64::mul_add` is a software routine many times slower.
#[inline(always)]
pub fn fma(a: f64, b: f64, c: f64) -> f64 {
    #[cfg(target_arch = "x86_64")]
    if std::arch::is_x86_feature_detected!("fma") {
        // SAFETY: the host supports FMA (checked above).
        return unsafe { fma_x86(a, b, c) };
    }
    a.mul_add(b, c)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "fma")]
unsafe fn fma_x86(a: f64, b: f64, c: f64) -> f64 {
    a.mul_add(b, c)
}
