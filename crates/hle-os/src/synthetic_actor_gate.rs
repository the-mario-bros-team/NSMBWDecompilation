// The dispatcher uses a Rust-only result type in its native calling convention.
#![allow(improper_ctypes_definitions)]

use super::*;
use guest_cpu::recomp::{self, FnInfo, Rt};
use std::time::{SystemTime, UNIX_EPOCH};

const ACTOR: u32 = 0x817f_0000;
const PARK: u32 = ACTOR + 0x100;
const STATE: u32 = 0x93f0_4000;
const HEADER: u32 = 0x93f0_0000;
const TABLE: u32 = 0x93f0_0100;
const ENTRY: RelAddress = RelAddress {
    module: 1,
    section: 1,
    offset: 0,
};
const WORDS: [u32; 4] = [
    0x8083_0000, // lwz r4,0(r3)
    0x3884_0003, // addi r4,r4,3
    0x9083_0000, // stw r4,0(r3)
    0x4e80_0020, // blr
];
static NATIVE: [FnInfo; 1] = [FnInfo {
    start: ACTOR,
    words: &WORDS,
    entries: &[ACTOR],
    f: tick,
}];

guest_cpu::guest_fn! {
    pub fn tick(cpu: &mut Cpu, mem: &mut Mem, rt: &mut Rt) -> recomp::R {
        if !rt.enter(mem, 0) {
            return Ok(());
        }
        let state = cpu.gpr[3];
        cpu.gpr[4] = mem.read_u32(state).unwrap().wrapping_add(3);
        mem.write_u32(state, cpu.gpr[4]).unwrap();
        cpu.cycles += WORDS.len() as u64;
        recomp::slice(cpu, mem, rt, cpu.lr);
        Ok(())
    }
}

struct DiscFixture(PathBuf);

impl DiscFixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("hle-native-actor-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        let fixture = Self(path);
        let sys = fixture.0.join("sys");
        fs::create_dir(&sys).unwrap();
        let mut boot = [0u8; 0x440];
        boot[..6].copy_from_slice(b"TEST00");
        boot[0x420..0x424].copy_from_slice(&0x1000u32.to_be_bytes());
        boot[0x424..0x428].copy_from_slice(&0x1100u32.to_be_bytes());
        fs::write(sys.join("boot.bin"), boot).unwrap();
        fs::write(sys.join("bi2.bin"), []).unwrap();
        fs::write(sys.join("apploader.img"), []).unwrap();
        let mut dol = [0u8; 0x100];
        dol[0xe0..0xe4].copy_from_slice(&PARK.to_be_bytes());
        fs::write(sys.join("main.dol"), dol).unwrap();
        let mut fst = [0u8; 12];
        fst[0] = 1; // root directory, with no children
        fst[8..].copy_from_slice(&1u32.to_be_bytes());
        fs::write(sys.join("fst.bin"), fst).unwrap();
        fs::write(fixture.0.join("tmd.bin"), [0u8; 0x18c]).unwrap();
        fixture
    }

    fn boot(&self, name: &str) -> System {
        let mut system = System::boot(&self.0, self.0.join(name)).unwrap();
        system.memcpy_validator = None;
        system.native_memcpy = None;

        for (index, word) in WORDS.into_iter().enumerate() {
            system
                .mem
                .write_u32(ACTOR + 4 * index as u32, word)
                .unwrap();
        }
        for (address, word) in [
            (PARK, 0x4800_0000), // branch to self until the field ends
            (STATE, 100),
            (MODULE_LIST_HEAD, HEADER),
            (HEADER, ENTRY.module),
            (HEADER + 12, 2),
            (HEADER + 16, TABLE),
            (TABLE + 8, ACTOR | 1),
            (TABLE + 12, WORDS.len() as u32 * 4),
        ] {
            system.mem.write_u32(address, word).unwrap();
        }
        system
    }
}

impl Drop for DiscFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn step_actor(system: &mut System) -> (Vec<u8>, u64) {
    system.cpu.pc = system.resolve_rel_address(ENTRY, 4, true).unwrap();
    system.cpu.gpr[3] = STATE;
    system.cpu.lr = PARK;
    system.run_field(system.cpu.cycles + SLICE).unwrap();
    assert_eq!(system.cpu.pc, PARK);
    let mut snap = Snap::saver();
    system.cpu.snap(&mut snap);
    (snap.into_bytes(), system.mem.hash())
}

#[test]
#[ignore = "full-RAM hashes are slow in debug; run this gate with --profile quick -- --ignored"]
fn synthetic_native_actor_matches_98_checkpoints() {
    let fixture = DiscFixture::new();
    let mut interpreted = fixture.boot("interpreter");
    let mut native = fixture.boot("native");
    let rt = Rt::new(&NATIVE, &mut native.mem, &[], &[]).unwrap();
    native.set_recomp(rt);

    for checkpoint in 1..=98 {
        let (reference_cpu, reference_hash) = step_actor(&mut interpreted);
        let (native_cpu, native_hash) = step_actor(&mut native);
        assert_eq!(reference_cpu, native_cpu, "CPU at checkpoint {checkpoint}");
        assert_eq!(
            reference_hash, native_hash,
            "RAM at checkpoint {checkpoint}"
        );
        assert_eq!(
            interpreted.steps, native.steps,
            "steps at checkpoint {checkpoint}"
        );
        assert_eq!(native.mem.read_u32(STATE), Ok(100 + 3 * checkpoint));
        assert_eq!(interpreted.recomp_steps, 0);
        assert_eq!(
            native.recomp_steps,
            u64::from(checkpoint) * WORDS.len() as u64
        );
        assert_eq!(native.recomp_entries, u64::from(checkpoint));
    }
}
