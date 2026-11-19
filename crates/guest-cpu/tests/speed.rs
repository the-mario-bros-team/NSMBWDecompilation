//! Interpreter throughput on small loops (`cargo test --release -p guest-cpu
//! --test speed -- --ignored --nocapture`). Not run by default.

use std::time::Instant;

use guest_cpu::blocks::Blocks;
use guest_cpu::Cpu;
use guest_mem::{Mem, NullMmio};

const BASE: u32 = 0x8000_1000;

fn run(name: &str, words: &[u32], n: u64) {
    let mut mem = Mem::new(Box::new(NullMmio));
    for (i, w) in words.iter().enumerate() {
        mem.write_u32(BASE + 4 * i as u32, *w).unwrap();
    }
    let mut cpu = Cpu::new();
    cpu.pc = BASE;
    cpu.gpr[4] = 0x8010_0000;
    cpu.ctr = u32::MAX;
    let mut blocks = Blocks::new(&[]);
    let t = Instant::now();
    let mut done = 0u64;
    while done < n {
        done += u64::from(cpu.run_block(&mut mem, &mut blocks).unwrap().0);
    }
    let s = t.elapsed().as_secs_f64();
    let t2 = Instant::now();
    cpu.pc = BASE;
    for _ in 0..n {
        cpu.step(&mut mem).unwrap();
    }
    let s2 = t2.elapsed().as_secs_f64();
    println!("{name:<28} blocks {:>6.1} M/s   step {:>6.1} M/s", done as f64 / s / 1e6, n as f64 / s2 / 1e6);
}

#[test]
#[ignore]
fn speed() {
    // addi r3,r3,1 ; addi r5,r5,2 ; xor r6,r3,r5 ; bdnz -12
    run("alu loop", &[0x3863_0001, 0x38A5_0002, 0x7C66_2A78, 0x4200_FFF4], 200_000_000);
    // lwz r3,0(r4) ; stw r3,4(r4) ; addi r3,r3,1 ; bdnz -12
    run("load/store loop", &[0x8064_0000, 0x9064_0004, 0x3863_0001, 0x4200_FFF4], 200_000_000);
    // lfs f1,0(r4) ; fmadds f2,f1,f1,f2 ; stfs f2,4(r4) ; bdnz -12
    run("fp loop", &[0xC024_0000, 0xEC41_107A, 0xD044_0004, 0x4200_FFF4], 100_000_000);
}
