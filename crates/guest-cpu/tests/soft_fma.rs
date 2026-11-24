//! On a processor without the FMA instruction (the x86-64-v2 compat build for
//! processors without AVX2), `guest_cpu::fp::fma` uses `fp::fma_soft`, which
//! must give the instruction's results bit for bit, or the compat build's
//! results would differ from the normal build's. It is software whatever the
//! processor; this compares it with the instruction on a processor that has
//! one.
//!
//! cargo test --release -p guest-cpu --test soft_fma

#![cfg(target_arch = "x86_64")]

#[target_feature(enable = "fma")]
unsafe fn hard(a: f64, b: f64, c: f64) -> f64 {
    a.mul_add(b, c)
}

fn soft(a: f64, b: f64, c: f64) -> f64 {
    use std::hint::black_box;
    guest_cpu::fp::fma_soft(black_box(a), black_box(b), black_box(c))
}

fn check(a: f64, b: f64, c: f64) {
    // SAFETY: only called once the processor is known to have FMA.
    let (s, h) = (soft(a, b, c), unsafe { hard(a, b, c) });
    // Of several NaNs, which comes out depends on the instruction's form
    // (see fma_soft): only that it is one of them.
    if [a, b, c].iter().filter(|x| x.is_nan()).count() > 1 {
        assert!([a, b, c].iter().any(|x| x.to_bits() | 1 << 51 == s.to_bits()), "{a:e} * {b:e} + {c:e}: {s:e}");
        return;
    }
    // Bit for bit, NaNs' payloads too.
    assert_eq!(s.to_bits(), h.to_bits(), "{a:e} * {b:e} + {c:e}: {s:e} (software) != {h:e} (instruction)");
}

#[test]
fn software_fma_matches_the_instruction() {
    if !std::arch::is_x86_feature_detected!("fma") {
        eprintln!("no FMA instruction to compare with");
        return;
    }
    let edges = [
        0.0, -0.0, 1.0, -1.0, f64::MIN_POSITIVE, -f64::MIN_POSITIVE, 5e-324, -5e-324, f64::MAX, f64::MIN, f64::INFINITY, f64::NEG_INFINITY, f64::NAN, -f64::NAN,
        f64::from_bits(0x7FF0_0000_0000_0001), f64::from_bits(0xFFF4_0000_0000_1234), 1.0 + f64::EPSILON, 1.0 - f64::EPSILON / 2.0, 3.0, 1.0 / 3.0, 0.1, 1e300, 1e-300,
    ];
    for &a in &edges {
        for &b in &edges {
            for &c in &edges {
                check(a, b, c);
            }
        }
    }
    // Random bit patterns, values near 1, and addends that nearly cancel the
    // product (where a second rounding would show).
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let near_one = |r: u64| f64::from_bits(0x3FF0_0000_0000_0000 | r >> 12);
    for i in 0..3_000_000u32 {
        let (a, b) = match i % 3 {
            0 => (f64::from_bits(next()), f64::from_bits(next())),
            _ => (near_one(next()), near_one(next())),
        };
        let c = match i % 3 {
            0 => f64::from_bits(next()),
            1 => near_one(next()),
            _ => -(a * b) * (1.0 + (next() >> 40) as f64 * f64::EPSILON),
        };
        check(a, b, c);
    }
}
