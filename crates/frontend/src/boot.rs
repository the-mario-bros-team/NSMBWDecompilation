//! `--boot`: boots `main.dol` through `hle_os::System` (interpreter, hardware
//! registers, IOS HLE) and reports how far it gets: OS messages, IOS requests,
//! REL links, video retraces, GX statistics, and where it stopped. Nothing
//! is drawn here; without a mode flag the frontend runs the game in a window.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use guest_mem::Mem;
use hle_os::System;

/// `main` (`start_main_thread` in the reconstruction, `src/engine/startup.rs`),
/// called by `__start` at `0x800041B4` after `OSInit`.
const MAIN: u32 = 0x800C_9F90;

/// Text-output entry points whose first argument is a C string: `os_report`
/// (`OSReport(fmt, ...)`) and `0x801AB1C0` (prints `"%s\n"` with r3).
const REPORTS: [u32; 2] = [0x801B_B9E0, 0x801A_B1C0];

fn c_string(mem: &mut Mem, mut addr: u32) -> String {
    let mut s = Vec::new();
    while let Ok(b) = mem.read_u8(addr) {
        if b == 0 || s.len() >= 200 {
            break;
        }
        s.push(b);
        addr = addr.wrapping_add(1);
    }
    String::from_utf8_lossy(&s).into_owned()
}

/// Semantic names from the ledger (`fn_<addr>` placeholders are skipped).
fn load_names(work_root: &Path) -> HashMap<u32, String> {
    let mut names = HashMap::new();
    if let Ok(text) = fs::read_to_string(work_root.join("ledger/functions.csv")) {
        for line in text.lines().skip(1) {
            let mut cols = line.splitn(4, ',');
            let (_, Some(addr), Some(name)) = (cols.next(), cols.next(), cols.next()) else { continue };
            if let Ok(a) = u32::from_str_radix(addr, 16) {
                if !name.starts_with("fn_") {
                    names.insert(a, name.to_string());
                }
            }
        }
    }
    names
}

fn label(names: &HashMap<u32, String>, addr: u32) -> String {
    match names.get(&addr) {
        Some(n) => format!("{addr:#010x} {n}"),
        None => format!("{addr:#010x}"),
    }
}

/// Call stack from a saved stack pointer: each frame holds the back chain at
/// +0 and the caller's saved LR at +4 of the frame above.
fn backtrace(mem: &mut Mem, mut sp: u32, names: &HashMap<u32, String>) -> String {
    let mut out = Vec::new();
    for _ in 0..12 {
        let Ok(next) = mem.read_u32(sp) else { break };
        if next == 0 || next == 0xFFFF_FFFF {
            break;
        }
        let Ok(lr) = mem.read_u32(next + 4) else { break };
        out.push(label(names, lr));
        sp = next;
    }
    out.join(" <- ")
}

/// The OS's active-thread list (head at 0x800000DC, linked at +0x2FC):
/// state, priority, saved PC and LR, and the call stack.
fn dump_threads(sys: &mut System, names: &HashMap<u32, String>) {
    let mem = &mut sys.mem;
    let current = mem.read_u32(0x8000_00E4).unwrap_or(0);
    let mut t = mem.read_u32(0x8000_00DC).unwrap_or(0);
    println!("threads (current {current:#010x}):");
    for _ in 0..32 {
        if t == 0 {
            break;
        }
        let state = mem.read_u16(t + 0x2C8).unwrap_or(0);
        let prio = mem.read_u32(t + 0x2D0).unwrap_or(0);
        let queue = mem.read_u32(t + 0x2DC).unwrap_or(0);
        let (pc, lr, sp) = if t == current {
            (sys.cpu.pc, sys.cpu.lr, sys.cpu.gpr[1])
        } else {
            (mem.read_u32(t + 0x198).unwrap_or(0), mem.read_u32(t + 0x84).unwrap_or(0), mem.read_u32(t + 4).unwrap_or(0))
        };
        let st = match state {
            1 => "ready",
            2 => "running",
            4 => "waiting",
            8 => "moribund",
            _ => "?",
        };
        println!("  {t:#010x} {st:<8} prio {prio:>2} queue {queue:#010x} pc {} lr {}", label(names, pc), label(names, lr));
        println!("      stack: {}", backtrace(mem, sp, names));
        t = mem.read_u32(t + 0x2FC).unwrap_or(0);
    }
}

pub fn run(game: &Path, max_steps: u64) -> bool {
    match boot(game, max_steps) {
        Ok(reached) => reached,
        Err(e) => {
            eprintln!("boot: {e}");
            false
        }
    }
}

fn boot(game: &Path, max_steps: u64) -> Result<bool, String> {
    let names = load_names(Path::new("."));
    let mut sys = System::boot(game, "saves/nand".into())?;
    {
        let mut hw = sys.hw.borrow_mut();
        hw.ios.trace = true;
        hw.ios.bt.trace = true;
        // Wii Remote 1 is switched on, as in a live run.
        hw.ios.bt.set_input(0, hle_ios::wiimote::Input { connected: true, ..Default::default() });
    }
    println!("boot: entry {:#010x}, IOS{}, running up to {max_steps} instructions", sys.cpu.pc, sys.ios_version);

    let mut recent_pcs = [0u32; 32];
    let mut recent_calls: Vec<(u32, u32)> = Vec::new();
    let mut called: HashMap<u32, u64> = HashMap::new();
    let mut main_at = None;
    let started = Instant::now();
    let mut stop = None;
    let mut linked_seen = 0;
    let mut next_retrace_report = 60;
    let (mut last_retrace, mut new_textures, mut texture_bytes, mut max_cmds, mut max_vertices) = (0, 0, 0, 0, 0);
    while sys.cpu.cycles < max_steps {
        let base = sys.steps;
        let mut i = 0u64;
        let r = sys.run_slice(hle_os::SLICE.min(max_steps - sys.cpu.cycles), |cpu, mem, pc| {
            recent_pcs[((base + i) % 32) as usize] = pc;
            i += 1;
            if cpu.pc == MAIN && main_at.is_none() {
                main_at = Some(base + i);
                println!("  -- reached main ({MAIN:#010x}) after {} instructions", base + i);
            }
            if cpu.lr == pc.wrapping_add(4) && cpu.pc != pc.wrapping_add(4) && mem.read_u32(pc).is_ok_and(|w| w >> 26 == 18 || w >> 26 == 19 || w >> 26 == 16) {
                *called.entry(cpu.pc).or_insert(0) += 1;
                if REPORTS.contains(&cpu.pc) {
                    let text = c_string(mem, cpu.gpr[3]);
                    println!("  [report from {pc:#010x}] {}", text.trim_end());
                }
                recent_calls.push((pc, cpu.pc));
                if recent_calls.len() > 64 {
                    recent_calls.drain(..32);
                }
            }
            true
        });
        if let Err(s) = r {
            stop = Some(s);
            break;
        }
        for m in &sys.linked[linked_seen..] {
            let secs: Vec<String> = m.sections.iter().map(|(a, s, x)| format!("{a:#010x}+{s:#x}{}", if *x { " x" } else { "" })).collect();
            println!("  -- OSLink: module {} at {:#010x} {} [{}]", m.id, m.header, if m.ok { "ok" } else { "FAILED" }, secs.join(", "));
        }
        linked_seen = sys.linked.len();
        let retraces = sys.hw.borrow().retraces;
        if retraces != last_retrace {
            // One video field: drain what the GPU decoded, as the renderer would.
            last_retrace = retraces;
            let out = sys.hw.borrow_mut().gpu.take_output();
            new_textures += out.new_textures.len();
            texture_bytes += out.new_textures.iter().map(|t| t.levels.iter().map(Vec::len).sum::<usize>()).sum::<usize>();
            max_cmds = max_cmds.max(out.cmds.len());
            max_vertices = max_vertices.max(out.vertices.len());
        }
        if retraces >= next_retrace_report {
            let secs = started.elapsed().as_secs_f64();
            println!("  -- {retraces} VI retraces after {} instructions ({:.1} retraces/s host time)", sys.steps, retraces as f64 / secs);
            next_retrace_report += 60;
        }
    }
    let secs = started.elapsed().as_secs_f64();
    println!(
        "ran {} instructions in {secs:.2}s ({:.1} M/s); guest time {:.2}s, {:.0}% of it idle-skipped",
        sys.steps,
        sys.steps as f64 / secs / 1e6,
        sys.cpu.cycles as f64 / 729e6,
        sys.idle_cycles as f64 * 100.0 / sys.cpu.cycles.max(1) as f64
    );
    match stop {
        Some(s) => println!("STOPPED: {s:?}"),
        None => println!("STOPPED: instruction limit, pc {}", label(&names, sys.cpu.pc)),
    }

    let hw = sys.hw.borrow();
    let g = &hw.gpu.inline().expect("--boot decodes GX inline").stats;
    println!(
        "interrupts: {} external, {} decrementer; VI retraces {}; PE draw-done {}; AI DMA blocks {}",
        sys.interrupts(), sys.decrementers(), hw.retraces, hw.pe_finishes, hw.dsp.ai_blocks
    );
    println!(
        "GX: {} draws, {} vertices, {} display lists, {} XFB copies, {} texture copies, {} line and {} point draws; bad opcodes {} (last {:#04x})",
        g.draws, g.vertices, g.display_lists, g.xfb_copies, g.tex_copies, g.lines, g.points, g.bad_opcodes, g.last_bad
    );
    println!("GX per field: up to {max_cmds} commands and {max_vertices} vertices; {new_textures} textures decoded ({} KiB RGBA)", texture_bytes / 1024);
    println!("DSP command lists: {}", hw.dsp.command_lists);
    println!("REL modules linked: {}", sys.linked.len());
    for (what, req) in &hw.ios.held {
        println!("IOS request held without reply: {what} (request {req:#010x})");
    }
    println!("last mails CPU->DSP: {:x?}", hw.dsp.from_cpu.iter().rev().take(8).collect::<Vec<_>>());
    println!("last calls (call site -> target):");
    for (site, t) in recent_calls.iter().rev().take(16).rev() {
        println!("  {site:#010x} -> {}", label(&names, *t));
    }
    println!("last instructions:");
    drop(hw);
    for i in 0..16u64 {
        let at = recent_pcs[((sys.steps + 32 - 16 + i) % 32) as usize];
        let word = sys.mem.read_u32(at).unwrap_or(0);
        println!("  {at:#010x}: {word:08x}");
    }
    if stop.is_none() {
        // Where is it spinning? Sample the next 200k instructions.
        let mut hot: HashMap<u32, u32> = HashMap::new();
        for _ in 0..400 {
            if sys.run_slice(500, |_, _, pc| {
                *hot.entry(pc).or_default() += 1;
                true
            })
            .is_err()
            {
                break;
            }
        }
        let mut hot: Vec<_> = hot.into_iter().collect();
        hot.sort_by(|a, b| b.1.cmp(&a.1));
        println!("hottest PCs over the next 200k instructions:");
        for (pc, n) in hot.iter().take(10) {
            println!("  {}  {n}", label(&names, *pc));
        }
    }
    let mut top: Vec<_> = called.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{} distinct call targets; most called:", top.len());
    for (t, n) in top.iter().take(12) {
        println!("  {:>10}  {}", n, label(&names, *t));
    }
    dump_threads(&mut sys, &names);
    let hw = sys.hw.borrow();
    println!("unmodelled hardware registers touched ({}):", hw.unknown.len());
    for ((addr, size, write), u) in &hw.unknown {
        let dir = if *write { "W" } else { "R" };
        println!("  {dir}{size} {addr:#010x}  first at cycle {:>10}  x{:<8} last value {:#010x}", u.first, u.count, u.last);
    }
    Ok(main_at.is_some())
}
