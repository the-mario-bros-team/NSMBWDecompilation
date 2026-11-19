//! `ppc-recomp`: writes Rust for the guest functions a run executed
//! (docs/PORT_PLAN.md, P6). The output goes to `crates/game-recomp/gen/`,
//! which `game-recomp` compiles into the port.
//!
//! ```text
//! ppc-recomp --ram <image> --map <MAP.txt> --profile <file>... [--out <dir>]
//! ppc-recomp --empty [--out <dir>]
//! ```
//!
//! `--max <n>` keeps only the first n functions (for quick test builds).
//!
//! ```text
//! ```
//!
//! - `--ram`: MEM1 then MEM2, from the port's `--dump-ram` (taken once the
//!   game's REL modules are linked, so their code is in place).
//! - `--map`: one `<address> <name>` per line, every function start (the
//!   recomp project's `projects/nsmbw/MAP.txt` lists this build's). A
//!   function runs to the next start.
//! - `--profile`: block starts from the port's `--code-profile`. Functions
//!   with one of them are generated, and every profiled address in a
//!   function becomes a place it can be entered (switch-table targets, the
//!   points where threads resume).
//!
//! Each function becomes `fn f_<addr>(c, m, rt)`: a `loop { match pc }` over
//! its basic blocks. The functions are split by address into [`SHARDS`]
//! files, each its own crate (`crates/game-recomp/shards/sNN`) so cargo
//! compiles them in parallel; a call within a shard is a direct Rust call,
//! one into another shard goes through the function table by index. Instructions call the interpreter's handlers with
//! constant words (`guest_cpu::fast`, `guest_cpu::interp`); branches,
//! calls and returns are native. See `guest_cpu::recomp` for the contract.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::PathBuf;

/// Shard crates (fixed: `crates/game-recomp/shards/s00` .. `s15`).
const SHARDS: usize = 16;

const MEM1: u32 = 0x8000_0000;
const MEM1_SIZE: usize = 0x0180_0000;
const MEM2: u32 = 0x9000_0000;
const MEM2_SIZE: usize = 0x0400_0000;

struct Ram(Vec<u8>);

impl Ram {
    fn word(&self, a: u32) -> Option<u32> {
        let off = if (MEM1..MEM1 + MEM1_SIZE as u32).contains(&a) {
            (a - MEM1) as usize
        } else if (MEM2..MEM2 + MEM2_SIZE as u32).contains(&a) {
            MEM1_SIZE + (a - MEM2) as usize
        } else {
            return None;
        };
        let b = self.0.get(off..off + 4)?;
        Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
}

/// What an instruction does to control flow.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    /// `b`/`bl`/`ba`: target, link.
    B(u32, bool),
    /// `bc`: BO, BI, target, link.
    Bc(u32, u32, u32, bool),
    /// `bclr`: BO, BI, link.
    Bclr(u32, u32, bool),
    /// `bcctr`: BO, BI, link.
    Bcctr(u32, u32, bool),
    /// Left to the interpreter: `sc`, `rfi`, `mtmsr` (they change the MSR or
    /// take an exception).
    Island,
    Plain,
}

fn kind(pc: u32, w: u32) -> Kind {
    let (bo, bi) = ((w >> 21) & 31, (w >> 16) & 31);
    let link = w & 1 != 0;
    match w >> 26 {
        18 => {
            let li = ((w & 0x03FF_FFFC) ^ 0x0200_0000).wrapping_sub(0x0200_0000);
            Kind::B(if w & 2 != 0 { li } else { pc.wrapping_add(li) }, link)
        }
        16 => {
            let bd = (w & 0xFFFC) as i16 as i32 as u32;
            Kind::Bc(bo, bi, if w & 2 != 0 { bd } else { pc.wrapping_add(bd) }, link)
        }
        17 => Kind::Island,
        19 => match (w >> 1) & 0x3FF {
            16 => Kind::Bclr(bo, bi, link),
            528 => Kind::Bcctr(bo, bi, link),
            50 => Kind::Island,
            _ => Kind::Plain,
        },
        31 if (w >> 1) & 0x3FF == 146 => Kind::Island,
        _ => Kind::Plain,
    }
}

/// BO with "ignore CTR" and "ignore the condition" both set.
fn always(bo: u32) -> bool {
    bo & 0x14 == 0x14
}

/// `mftb`, and `mfspr`/`mtspr` of DEC or the timebase: they count from
/// `cpu.cycles`, so it must be exact at them.
fn reads_cycles(w: u32) -> bool {
    let spr = ((w >> 16) & 0x1F) | (((w >> 11) & 0x1F) << 5);
    w >> 26 == 31 && ((w >> 1) & 0x3FF == 371 || matches!((w >> 1) & 0x3FF, 339 | 467) && matches!(spr, 22 | 268 | 269 | 284 | 285))
}

struct Func {
    start: u32,
    end: u32,
    name: String,
    /// Block start -> instruction addresses (a straight run; the last may be
    /// a branch).
    blocks: BTreeMap<u32, u32>,
    /// One past the last instruction any block covers.
    covered_end: u32,
}

struct Gen<'a> {
    ram: &'a Ram,
    special: BTreeSet<u32>,
    /// Entry address -> owning function's index.
    entry_owner: HashMap<u32, usize>,
    /// Per function index: start and shard.
    start_of: Vec<u32>,
    shard_of: Vec<usize>,
    /// While writing a function: each block's (group, index in group), and
    /// the block being written.
    pos: std::cell::RefCell<HashMap<u32, (usize, usize)>>,
    here: std::cell::Cell<(usize, usize)>,
}

/// Blocks per group of nested labeled blocks (bounds the nesting depth).
const GROUP: usize = 128;

/// How code in one shard reaches a function.
enum Callee {
    Direct(String),
    Index(usize),
}

/// Finds a function's blocks: from its start, following fall-through and
/// branches that stay inside it; then again with the profiled addresses
/// that this did not reach (targets of computed jumps). Profiled addresses
/// inside a block are where a slice happened to end; the interpreter runs
/// from there to the next block.
fn discover(ram: &Ram, special: &BTreeSet<u32>, start: u32, end: u32, seeds: &[u32]) -> Func {
    let first = descend(ram, special, start, end, &[]);
    let inside_block = |a: u32| first.blocks.range(..=a).next_back().is_some_and(|(_, &e)| a < e);
    let extra: Vec<u32> = seeds.iter().copied().filter(|&a| !inside_block(a)).collect();
    if extra.is_empty() {
        first
    } else {
        descend(ram, special, start, end, &extra)
    }
}

fn descend(ram: &Ram, special: &BTreeSet<u32>, start: u32, end: u32, seeds: &[u32]) -> Func {
    let inside = |a: u32| (start..end).contains(&a);
    // Leaders first (every place a block must start), then the blocks.
    let mut leaders: BTreeSet<u32> = BTreeSet::new();
    let mut work: Vec<u32> = std::iter::once(start).chain(seeds.iter().copied()).filter(|&a| inside(a)).collect();
    let mut visited: BTreeSet<u32> = BTreeSet::new();
    while let Some(l) = work.pop() {
        let Some(w) = ram.word(l) else { continue };
        if kind(l, w) == Kind::Island {
            // Not an entry; the interpreter runs it, then whatever follows.
            if inside(l + 4) && leaders.insert(l + 4) {
                work.push(l + 4);
            }
            continue;
        }
        leaders.insert(l);
        if !visited.insert(l) {
            continue;
        }
        let mut a = l;
        while let Some(w) = ram.word(a) {
            match kind(a, w) {
                Kind::Island => {
                    work.push(a);
                    break;
                }
                Kind::Plain => {}
                k => {
                    let next = a + 4;
                    let target = match k {
                        Kind::B(t, link) => {
                            if link && inside(next) {
                                work.push(next);
                            }
                            Some(t)
                        }
                        Kind::Bc(bo, _, t, _) => {
                            if !always(bo) && inside(next) {
                                work.push(next);
                            }
                            Some(t)
                        }
                        Kind::Bclr(bo, _, link) | Kind::Bcctr(bo, _, link) => {
                            if (link || !always(bo)) && inside(next) {
                                work.push(next);
                            }
                            None
                        }
                        _ => unreachable!(),
                    };
                    if let Some(t) = target.filter(|&t| inside(t)) {
                        work.push(t);
                    }
                    break;
                }
            }
            a += 4;
            if !inside(a) {
                break;
            }
            if special.contains(&a) {
                work.push(a);
                break;
            }
        }
    }
    // Cut runs at leaders: block = leader up to (not including) the next
    // leader, or through its branch.
    let mut blocks = BTreeMap::new();
    let mut covered_end = start;
    let all: Vec<u32> = leaders.iter().copied().collect();
    for (i, &l) in all.iter().enumerate() {
        let next_leader = all.get(i + 1).copied().unwrap_or(end);
        let mut a = l;
        while let Some(w) = ram.word(a) {
            let k = kind(a, w);
            if k == Kind::Island {
                break;
            }
            a += 4;
            if k != Kind::Plain || a >= next_leader || !inside(a) || special.contains(&a) {
                break;
            }
        }
        if a > l {
            blocks.insert(l, a);
            covered_end = covered_end.max(a);
        }
    }
    Func { start, end, name: String::new(), blocks, covered_end }
}

impl Gen<'_> {
    /// Code that continues at `to`: a jump inside the function, or leaving
    /// it. A jump forward within the current group of blocks is a `break`
    /// out of the labeled blocks they are nested in (a plain jump); any
    /// other goes back through the `match` on `pc` at the loop's top.
    fn goto(&self, f: &Func, to: u32, out: &mut String) {
        if f.blocks.contains_key(&to) && !self.special.contains(&to) {
            let (seg, i) = self.pos.borrow()[&to];
            let (here_seg, here) = self.here.get();
            if seg == here_seg && i > here {
                let _ = write!(out, "break 'b{i}; ");
            } else {
                let _ = write!(out, "pc = {to:#010x}; continue 'top; ");
            }
        } else {
            let _ = write!(out, "c.pc = {to:#010x}; return Ok(()); ");
        }
    }

    /// The function with an entry at `to`, if any (and `to` is no hook),
    /// as code in shard `from` reaches it.
    fn callee(&self, to: u32, from: usize) -> Option<Callee> {
        if self.special.contains(&to) {
            return None;
        }
        let &i = self.entry_owner.get(&to)?;
        Some(if self.shard_of[i] == from { Callee::Direct(format!("f_{:08x}", self.start_of[i])) } else { Callee::Index(i) })
    }

    fn function(&self, f: &Func, idx: usize, out: &mut String) {
        let _ = writeln!(out, "// {} ({:#010x}..{:#010x})", f.name, f.start, f.end);
        let _ = writeln!(out, "pub fn f_{:08x}(c: &mut Cpu, m: &mut Mem, rt: &mut Rt) -> R {{", f.start);
        let _ = writeln!(out, "    if !rt.enter(m, {idx}) {{ return Ok(()); }}");
        let _ = writeln!(out, "    let mut pc = c.pc;");
        let _ = writeln!(out, "    'top: loop {{");
        let specials: Vec<u32> = f.blocks.keys().copied().filter(|a| self.special.contains(a)).collect();
        // Groups of consecutive blocks; in each, block i sits after the end
        // of labeled block 'bi and inside 'b(i+1).., so `break 'bj` from any
        // earlier block lands on block j. A `match` on `pc` enters the group.
        let all: Vec<(u32, u32)> = f.blocks.iter().map(|(&l, &e)| (l, e)).collect();
        let groups: Vec<&[(u32, u32)]> = all.chunks(GROUP).collect();
        *self.pos.borrow_mut() = groups.iter().enumerate().flat_map(|(g, bs)| bs.iter().enumerate().map(move |(i, b)| (b.0, (g, i)))).collect();
        for (g, bs) in groups.iter().enumerate() {
            let _ = writeln!(out, "    if pc >= {:#010x} && pc < {:#010x} {{", bs[0].0, bs[bs.len() - 1].1);
            for i in (1..bs.len()).rev() {
                let _ = write!(out, "'b{i}: {{ ");
            }
            let arms: Vec<String> = bs.iter().enumerate().map(|(i, b)| if i == 0 { format!("{:#010x} => {{}}", b.0) } else { format!("{:#010x} => break 'b{i}", b.0) }).collect();
            let _ = writeln!(out, "\n    match pc {{ {}, _ => {{ c.pc = pc; return Ok(()); }} }}", arms.join(", "));
            for (i, &(l, e)) in bs.iter().enumerate() {
                if i > 0 {
                    let _ = writeln!(out, "    }} // 'b{i}");
                }
                self.here.set((g, i));
                self.block(f, idx, l, e, &specials, out);
            }
            let _ = writeln!(out, "    }}");
        }
        let _ = writeln!(out, "    c.pc = pc; return Ok(());");
        let _ = writeln!(out, "    }}");
        let _ = writeln!(out, "}}");
    }

    fn block(&self, f: &Func, idx: usize, l: u32, e: u32, specials: &[u32], out: &mut String) {
        {
            let words: Vec<(u32, u32)> = (l..e).step_by(4).map(|a| (a, self.ram.word(a).unwrap())).collect();
            let n = words.len() as u64;
            let mut s = String::new();
            let _ = write!(s, "        // {l:#010x}\n        ");
            if words.iter().any(|&(_, w)| guest_cpu::is_fp(w)) {
                let _ = write!(s, "if c.msr & MSR_FP == 0 {{ c.pc = {l:#010x}; return Ok(()); }} ");
            }
            let _ = writeln!(s, "c.cycles += {n};");
            for (i, &(a, w)) in words.iter().enumerate() {
                let k = kind(a, w);
                if k != Kind::Plain {
                    self.branch(f, idx, a, k, &mut s, specials);
                    break;
                }
                let later = n - i as u64 - 1;
                let adjust = reads_cycles(w) && later > 0;
                s.push_str("            ");
                if adjust {
                    let _ = write!(s, "c.cycles -= {later}; ");
                }
                match guest_cpu::fast::handler_name(w) {
                    // Stack and small-data accesses go straight to RAM.
                    Some(h) if (h.starts_with('l') || h.starts_with('s')) && guest_cpu::fast::raw_ok(w) => {
                        let _ = write!(s, "at(fast::{h}_via::<fast::Raw>(c, m, {w:#010x}), {a:#010x})?;");
                    }
                    Some(h) if h.starts_with('l') || h.starts_with('s') => {
                        let _ = write!(s, "at(fast::{h}(c, m, {w:#010x}), {a:#010x})?;");
                    }
                    Some(h) => {
                        let _ = write!(s, "let _ = fast::{h}(c, m, {w:#010x});");
                    }
                    None => {
                        let call = match w >> 26 {
                            4 => format!("interp::paired(c, m, {w:#010x})"),
                            19 => format!("interp::op19(c, {w:#010x})"),
                            31 => format!("interp::op31(c, m, {w:#010x})"),
                            59 => format!("interp::fp_single(c, {w:#010x})"),
                            63 => format!("interp::fp_double(c, {w:#010x})"),
                            _ => format!("interp::op_main(c, m, {w:#010x})"),
                        };
                        let _ = write!(s, "at({call}, {a:#010x})?;");
                    }
                }
                if adjust {
                    let _ = write!(s, " c.cycles += {later};");
                }
                s.push('\n');
                if i + 1 == words.len() {
                    // Fell off the block: the next block, or leave.
                    s.push_str("            ");
                    self.goto(f, e, &mut s);
                    s.push('\n');
                }
            }
            out.push_str(&s);
        }
    }

    /// A branch instruction. Every form first sets `lr` if it links, then
    /// checks the slice budget with `pc` at its destination (`slice`; the
    /// interpreter ends slices after branches too), then moves on.
    fn branch(&self, f: &Func, idx: usize, a: u32, k: Kind, s: &mut String, specials: &[u32]) {
        let next = a + 4;
        let shard = self.shard_of[idx];
        let check = |s: &mut String, to: &str| {
            let _ = write!(s, "if !slice(c, m, rt, {to}) {{ return Ok(()); }} ");
        };
        s.push_str("            ");
        match k {
            Kind::B(t, true) => {
                let _ = write!(s, "c.lr = {next:#010x}; ");
                check(s, &format!("{t:#010x}"));
                match self.callee(t, shard) {
                    Some(callee) => {
                        let _ = match callee {
                            Callee::Direct(name) => write!(s, "if !call(c, m, rt, {name}, {t:#010x}, {next:#010x})? {{ return Ok(()); }} "),
                            Callee::Index(i) => write!(s, "if !call_index(c, m, rt, {i}, {t:#010x}, {next:#010x})? {{ return Ok(()); }} "),
                        };
                        self.goto(f, next, s);
                    }
                    None => {
                        let _ = write!(s, "return Ok(());");
                    }
                }
            }
            Kind::B(t, false) => {
                check(s, &format!("{t:#010x}"));
                match self.callee(t, shard).filter(|_| !f.blocks.contains_key(&t)) {
                    // Into another function: run it, then return as it does.
                    Some(Callee::Direct(name)) => {
                        let _ = write!(s, "return tail(c, m, rt, {name}, {t:#010x});");
                    }
                    Some(Callee::Index(i)) => {
                        let _ = write!(s, "return tail_index(c, m, rt, {i}, {t:#010x});");
                    }
                    None => self.goto(f, t, s),
                }
            }
            Kind::Bc(bo, bi, t, link) => {
                if always(bo) && !link {
                    check(s, &format!("{t:#010x}"));
                    self.goto(f, t, s);
                } else {
                    let _ = write!(s, "let taken = fast::branch_taken(c, {bo}, {bi}, true); ");
                    if link {
                        let _ = write!(s, "c.lr = {next:#010x}; ");
                    }
                    check(s, &format!("if taken {{ {t:#010x} }} else {{ {next:#010x} }}"));
                    let _ = write!(s, "if taken {{ ");
                    if link {
                        let _ = write!(s, "return Ok(()); ");
                    } else {
                        self.goto(f, t, s);
                    }
                    let _ = write!(s, "}} ");
                    self.goto(f, next, s);
                }
            }
            Kind::Bclr(bo, bi, link) => {
                let _ = write!(s, "let t = c.lr & !3; ");
                if always(bo) && !link {
                    check(s, "t");
                    let _ = write!(s, "return Ok(());");
                } else {
                    let _ = write!(s, "let taken = fast::branch_taken(c, {bo}, {bi}, true); ");
                    if link {
                        let _ = write!(s, "c.lr = {next:#010x}; ");
                    }
                    check(s, &format!("if taken {{ t }} else {{ {next:#010x} }}"));
                    let _ = write!(s, "if taken {{ ");
                    if link {
                        let _ = write!(s, "if !call_indirect(c, m, rt, t, {next:#010x})? {{ return Ok(()); }} ");
                        self.goto(f, next, s);
                    } else {
                        let _ = write!(s, "return Ok(()); ");
                    }
                    let _ = write!(s, "}} ");
                    self.goto(f, next, s);
                }
            }
            Kind::Bcctr(bo, bi, link) => {
                let _ = write!(s, "let t = c.ctr & !3; ");
                if always(bo) {
                    let _ = write!(s, "let taken = true; ");
                } else {
                    let _ = write!(s, "let taken = fast::branch_taken(c, {bo}, {bi}, false); ");
                }
                if link {
                    let _ = write!(s, "c.lr = {next:#010x}; ");
                }
                check(s, &format!("if taken {{ t }} else {{ {next:#010x} }}"));
                let _ = write!(s, "if taken {{ ");
                if link {
                    let _ = write!(s, "if !call_indirect(c, m, rt, t, {next:#010x})? {{ return Ok(()); }} ");
                    self.goto(f, next, s);
                } else {
                    // A computed jump (switch tables): one of ours, or leave.
                    if !specials.is_empty() {
                        let list: Vec<String> = specials.iter().map(|a| format!("{a:#010x}")).collect();
                        let _ = write!(s, "if matches!(t, {}) {{ return Ok(()); }} ", list.join(" | "));
                    }
                    let _ = write!(s, "pc = t; continue 'top; ");
                }
                let _ = write!(s, "}} ");
                self.goto(f, next, s);
            }
            Kind::Island | Kind::Plain => unreachable!(),
        }
        s.push('\n');
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ppc-recomp: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let (mut ram, mut map, mut profiles, mut out, mut empty, mut max) = (None, None, Vec::new(), PathBuf::from("crates/game-recomp/gen"), false, usize::MAX);
    while let Some(a) = args.next() {
        let mut val = |n: &str| args.next().ok_or(format!("{n} needs a value"));
        match a.as_str() {
            "--ram" => ram = Some(PathBuf::from(val("--ram")?)),
            "--map" => map = Some(PathBuf::from(val("--map")?)),
            "--profile" => profiles.push(PathBuf::from(val("--profile")?)),
            "--out" => out = PathBuf::from(val("--out")?),
            "--empty" => empty = true,
            "--max" => max = val("--max")?.parse().map_err(|_| "--max: not a number")?,
            _ => return Err(format!("unknown argument {a}")),
        }
    }
    std::fs::create_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
    let special: BTreeSet<u32> = hle_os::SPECIAL.iter().copied().collect();
    if empty {
        return write_all(&out, &special, &[], &[], &vec![String::new(); SHARDS]);
    }
    let ram = Ram(std::fs::read(ram.ok_or("--ram is required")?).map_err(|e| format!("RAM image: {e}"))?);
    let map_path = map.ok_or("--map is required")?;
    let mut syms: Vec<(u32, String)> = std::fs::read_to_string(&map_path)
        .map_err(|e| format!("{}: {e}", map_path.display()))?
        .lines()
        .filter_map(|l| {
            let mut it = l.split_whitespace();
            Some((u32::from_str_radix(it.next()?, 16).ok()?, it.next().unwrap_or("").to_string()))
        })
        .collect();
    syms.sort();
    syms.dedup_by_key(|s| s.0);
    let mut seeds: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    let mut unmapped = 0;
    for p in &profiles {
        let text = std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
        for a in text.lines().filter_map(|l| u32::from_str_radix(l.trim(), 16).ok()) {
            match syms.partition_point(|s| s.0 <= a).checked_sub(1) {
                Some(i) if a < MEM1 + MEM1_SIZE as u32 && a >= MEM1 => seeds.entry(i).or_default().push(a),
                _ => unmapped += 1,
            }
        }
    }
    let mut funcs = Vec::new();
    for (&i, s) in seeds.iter().take(max) {
        let (start, name) = (syms[i].0, &syms[i].1);
        let end = syms.get(i + 1).map_or(start + 4, |n| n.0);
        let mut f = discover(&ram, &special, start, end, s);
        if f.blocks.is_empty() {
            continue;
        }
        f.name = name.clone();
        funcs.push(f);
    }
    let instrs: u64 = funcs.iter().map(|f| f.blocks.iter().map(|(a, e)| u64::from((e - a) / 4)).sum::<u64>()).sum();
    println!("{} functions, {} instructions, {} blocks; {unmapped} profiled addresses outside the map", funcs.len(), instrs, funcs.iter().map(|f| f.blocks.len()).sum::<usize>());
    // Contiguous address ranges of about equal size per shard.
    let per = instrs.div_ceil(SHARDS as u64).max(1);
    let mut shard_of = Vec::with_capacity(funcs.len());
    let mut acc = 0u64;
    for f in &funcs {
        shard_of.push(((acc / per) as usize).min(SHARDS - 1));
        acc += f.blocks.iter().map(|(a, e)| u64::from((e - a) / 4)).sum::<u64>();
    }
    let entry_owner = funcs.iter().enumerate().flat_map(|(i, f)| f.blocks.keys().map(move |&b| (b, i))).collect();
    let g = Gen { ram: &ram, special: special.clone(), entry_owner, start_of: funcs.iter().map(|f| f.start).collect(), shard_of: shard_of.clone(), pos: Default::default(), here: Default::default() };
    let mut bodies = vec![String::new(); SHARDS];
    for (i, f) in funcs.iter().enumerate() {
        let body = &mut bodies[shard_of[i]];
        let ws: Vec<String> = (f.start..f.covered_end).step_by(4).map(|a| format!("{:#x}", ram.word(a).unwrap())).collect();
        let es: Vec<String> = f.blocks.keys().map(|x| format!("{x:#x}")).collect();
        let _ = writeln!(body, "pub static W_{:08x}: [u32; {}] = [{}];", f.start, ws.len(), ws.join(","));
        let _ = writeln!(body, "pub static E_{:08x}: [u32; {}] = [{}];", f.start, es.len(), es.join(","));
        g.function(f, i, body);
    }
    write_all(&out, &special, &funcs, &shard_of, &bodies)?;
    Ok(())
}

/// `mod.rs` (the function table and the hook list) and the shard files.
fn write_all(out: &std::path::Path, special: &BTreeSet<u32>, funcs: &[Func], shard_of: &[usize], bodies: &[String]) -> Result<(), String> {
    let write = |name: &str, text: &str| {
        let path = out.join(name);
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
    };
    let head = "// Generated by ppc-recomp; do not edit.\n#![allow(unused, unreachable_code, nonstandard_style, clippy::all)]\n\n";
    for (i, b) in bodies.iter().enumerate() {
        let uses = "use guest_cpu::recomp::{at, call, call_index, call_indirect, slice, tail, tail_index, Rt, R};\nuse guest_cpu::{fast, interp, Cpu, MSR_FP};\nuse guest_mem::Mem;\n\n";
        write(&format!("s{i:02}.rs"), &format!("{head}{uses}{b}"))?;
    }
    let mut m = format!("{head}use guest_cpu::recomp::FnInfo;\n\n");
    let list: Vec<String> = special.iter().map(|a| format!("{a:#010x}")).collect();
    let _ = writeln!(m, "/// The hook addresses this code was generated for.\npub const SPECIAL: &[u32] = &[{}];\n", list.join(", "));
    let _ = writeln!(m, "pub static FUNCTIONS: [FnInfo; {}] = [", funcs.len());
    for (f, &sh) in funcs.iter().zip(shard_of) {
        let _ = writeln!(m, "    FnInfo {{ start: {0:#010x}, words: &recomp_s{1:02}::W_{0:08x}, entries: &recomp_s{1:02}::E_{0:08x}, f: recomp_s{1:02}::f_{0:08x} }},", f.start, sh);
    }
    m.push_str("];\n");
    write("mod.rs", &m)
}
