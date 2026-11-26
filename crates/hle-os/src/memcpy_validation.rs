//! Bounded comparison of the Rust byte-copy primitive with the live guest call.
use super::{replace_memcpy, CURRENT_THREAD};
use guest_cpu::Cpu;
use guest_mem::{Mem, NullMmio, LOCKED_CACHE_SIZE, MEM1_SIZE, MEM2_SIZE};

const SAMPLE_LIMIT: usize = 8;
const BANKS: [(u32, usize); 3] = [
    (0x8000_0000, MEM1_SIZE),
    (0x9000_0000, MEM2_SIZE),
    (0xe000_0000, LOCKED_CACHE_SIZE),
];

struct Pending {
    expected: Cpu,
    start_cycles: u64,
    stack: u32,
    thread: Option<u32>,
    destination: u32,
    len: usize,
}

pub(super) struct Comparison {
    registers_equal: bool,
    reference_cycles: u64,
    native_cycles: u64,
    timebase_equal: bool,
    destination_equal: bool,
    ram_equal: bool,
}

impl Comparison {
    pub fn report(&self) {
        eprintln!(
            "memcpy validation return: registers_equal={} reference_cycles={} native_cycles={} timebase_equal={} destination_equal={} ram_equal={}",
            self.registers_equal, self.reference_cycles, self.native_cycles,
            self.timebase_equal, self.destination_equal, self.ram_equal,
        );
    }
}

pub(super) struct Validator {
    shadow: Mem,
    pending: Option<Pending>,
    remaining: usize,
}

fn current_thread(mem: &Mem) -> Option<u32> {
    Some(u32::from_be_bytes(
        mem.ram(CURRENT_THREAD, 4)?.try_into().ok()?,
    ))
}

impl Validator {
    pub fn new() -> Self {
        Self {
            shadow: Mem::new(Box::new(NullMmio)),
            pending: None,
            remaining: SAMPLE_LIMIT,
        }
    }

    pub fn begin(&mut self, cpu: &Cpu, mem: &Mem) {
        if self.remaining == 0 || self.pending.is_some() {
            return;
        }
        self.remaining -= 1;
        for (addr, len) in BANKS {
            self.shadow
                .ram_mut(addr, len)
                .unwrap()
                .copy_from_slice(mem.ram(addr, len).unwrap());
        }
        let mut expected = cpu.clone();
        if !replace_memcpy(&mut expected, &mut self.shadow) {
            eprintln!("memcpy validation skipped: native hook declined the call");
            return;
        }
        self.pending = Some(Pending {
            expected,
            start_cycles: cpu.cycles,
            stack: cpu.gpr[1],
            thread: current_thread(mem),
            destination: cpu.gpr[3],
            len: cpu.gpr[5] as usize,
        });
    }

    pub fn finish(&mut self, cpu: &Cpu, mem: &Mem) -> Option<Comparison> {
        let pending = self.pending.as_ref()?;
        if cpu.pc != pending.expected.pc
            || cpu.gpr[1] != pending.stack
            || current_thread(mem) != pending.thread
        {
            return None;
        }
        let pending = self.pending.take().unwrap();
        let expected = &pending.expected;
        Some(Comparison {
            registers_equal: cpu.gpr == expected.gpr
                && cpu.cr == expected.cr
                && cpu.xer == expected.xer
                && cpu.lr == expected.lr
                && cpu.ctr == expected.ctr
                && cpu.msr == expected.msr
                && cpu.fpscr == expected.fpscr
                && cpu.sr == expected.sr
                && cpu.spr == expected.spr
                && cpu
                    .fpr
                    .iter()
                    .zip(&expected.fpr)
                    .all(|(a, b)| a.map(f64::to_bits) == b.map(f64::to_bits)),
            reference_cycles: cpu.cycles - pending.start_cycles,
            native_cycles: expected.cycles - pending.start_cycles,
            timebase_equal: cpu.timebase() == expected.timebase(),
            destination_equal: mem.ram(pending.destination, pending.len)
                == self.shadow.ram(pending.destination, pending.len),
            ram_equal: BANKS
                .iter()
                .all(|&(addr, len)| mem.ram(addr, len) == self.shadow.ram(addr, len)),
        })
    }

    pub fn cancel_at_slice_boundary(&mut self) {
        if self.pending.take().is_some() {
            eprintln!("memcpy validation inconclusive: reference call crossed a hardware slice");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_candidate_registers_cycles_and_final_ram() {
        let mut mem = Mem::new(Box::new(NullMmio));
        let mut cpu = Cpu::new();
        cpu.pc = 0x8000_1000;
        cpu.lr = 0x8000_2000;
        cpu.gpr[1] = 0x8000_3000;
        cpu.gpr[3] = 0x8000_4000;
        cpu.gpr[4] = 0x8000_5000;
        cpu.gpr[5] = 4;
        mem.write_u32(cpu.gpr[4], 0x1234_5678).unwrap();

        let mut verifier = Validator::new();
        verifier.begin(&cpu, &mem);
        assert!(verifier.finish(&cpu, &mem).is_none());
        assert!(replace_memcpy(&mut cpu, &mut mem));
        let identical = verifier.finish(&cpu, &mem).unwrap();
        assert!(identical.registers_equal);
        assert_eq!(identical.reference_cycles, identical.native_cycles);
        assert!(identical.destination_equal && identical.ram_equal);

        cpu.pc = 0x8000_1000;
        verifier.begin(&cpu, &mem);
        assert!(replace_memcpy(&mut cpu, &mut mem));
        cpu.gpr[4] += 4;
        cpu.cycles += 5;
        mem.write_u8(0x8000_6000, 1).unwrap();
        let different = verifier.finish(&cpu, &mem).unwrap();
        assert!(!different.registers_equal);
        assert_eq!(
            (different.reference_cycles, different.native_cycles),
            (5, 0)
        );
        assert!(different.destination_equal);
        assert!(!different.ram_equal);
    }
}
