//! GX on its own thread, as Dolphin's dual-core mode runs it, but with the
//! game-visible timing unchanged.
//!
//! The game thread runs a light [`Scanner`] over the FIFO stream. It finds
//! each command's length, raises the PE events (finish, tokens) at the same
//! point as inline decoding, copies display lists into the stream, and
//! resolves indexed XF loads. The [`Gpu`] on the worker thread does the rest
//! (vertex loading, transform, lighting, textures, EFB copies) from that
//! stream.
//!
//! The game thread waits for the worker at every PE event and when it takes
//! a field's output: at those points the worker has read everything the game
//! sent before, as the real GPU has once it signals finish. Vertex arrays and
//! textures are still read from guest RAM by the worker in between (a
//! [`RamView`]), as the real GPU reads them.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use guest_mem::{Mem, RamView};

use crate::vertex::{Cp, Layout};
use crate::{be32, Event, Gpu, Output};

/// Commands are sent to the worker in batches of about this many bytes.
const BATCH: usize = 16 << 10;

/// Finds command boundaries and PE events in the FIFO stream and rewrites
/// the commands that read RAM by address (display lists, indexed XF loads)
/// into plain commands, so the worker sees the bytes as they were when the
/// game sent them.
#[derive(Default)]
pub struct Scanner {
    cp: Cp,
    /// Draw sizes per VAT, valid until the next CP load.
    sizes: [Option<usize>; 8],
    /// BP 0x47 and 0x48 (tokens) and the BP write mask, for token values.
    tokens: [u32; 2],
    mask: u32,
    /// An incomplete command from the last call.
    pending: Vec<u8>,
}

impl Scanner {
    fn new() -> Self {
        Self { mask: 0xFF_FFFF, ..Default::default() }
    }

    /// Scans `bytes` after any incomplete command from before; complete
    /// commands go to `out` (rewritten where needed) and PE events to
    /// `events`. Returns true if it met an invalid opcode (the worker should
    /// then drop the rest of its batch, as inline decoding drops its buffer).
    ///
    /// This runs for every 32-byte burst, so it allocates nothing: with
    /// nothing left over (the usual case) it scans `bytes` where they are,
    /// and otherwise appends them to the leftover and scans that buffer.
    fn feed(&mut self, bytes: &[u8], mem: &Mem, out: &mut Vec<u8>, events: &mut Vec<Event>) -> bool {
        let mut buf = std::mem::take(&mut self.pending);
        let bad = if buf.is_empty() {
            let (at, bad) = self.scan(bytes, mem, out, events, false);
            if !bad {
                buf.extend_from_slice(&bytes[at.min(bytes.len())..]);
            }
            bad
        } else {
            buf.extend_from_slice(bytes);
            let (at, bad) = self.scan(&buf, mem, out, events, false);
            buf.drain(..at.min(buf.len()));
            bad
        };
        if bad {
            buf.clear();
        }
        self.pending = buf;
        bad
    }

    fn draw_size(&mut self, vat: usize) -> usize {
        *self.sizes[vat].get_or_insert_with(|| Layout::new(&self.cp, vat).size)
    }

    /// Mirrors `Gpu::run`: returns the bytes consumed and whether an invalid
    /// opcode ended the buffer.
    fn scan(&mut self, buf: &[u8], mem: &Mem, out: &mut Vec<u8>, events: &mut Vec<Event>, in_list: bool) -> (usize, bool) {
        let mut at = 0;
        while at < buf.len() {
            let rest = &buf[at..];
            let op = rest[0];
            let need = match op {
                0x00 | 0x44 | 0x48 => 1,
                0x08 => 6,
                0x10 => {
                    if rest.len() < 5 {
                        break;
                    }
                    5 + 4 * (((be32(&rest[1..]) >> 16) & 0xF) as usize + 1)
                }
                0x20 | 0x28 | 0x30 | 0x38 => 5,
                0x40 => 9,
                0x61 => 5,
                0x80..=0xBF => {
                    if rest.len() < 3 {
                        break;
                    }
                    let n = u16::from_be_bytes([rest[1], rest[2]]) as usize;
                    3 + n * self.draw_size((op & 7) as usize)
                }
                _ => {
                    // Inline decoding drops the rest of the buffer (of the
                    // list, inside one). At top level the worker gets the
                    // byte, counts it, and drops the rest of its batch.
                    if !in_list {
                        out.push(op);
                    }
                    return (buf.len(), true);
                }
            };
            if rest.len() < need {
                break;
            }
            let cmd = &rest[..need];
            match op {
                0x08 => {
                    self.cp.load(cmd[1], be32(&cmd[2..]));
                    self.sizes = [None; 8];
                    out.extend_from_slice(cmd);
                }
                0x20 | 0x28 | 0x30 | 0x38 => {
                    // Indexed XF load: the words as they are now, as plain
                    // XF writes (one per word, skipping words outside RAM as
                    // the inline decoder does).
                    let v = be32(&cmd[1..]);
                    let array = 12 + ((op >> 3) & 3) as usize;
                    let (index, size, addr) = (v >> 16, ((v >> 12) & 0xF) + 1, v & 0xFFF);
                    let src = self.cp.array_base[array].wrapping_add(index * self.cp.array_stride[array]);
                    match mem.ram(src, 4 * size as usize) {
                        Some(words) => {
                            out.push(0x10);
                            out.extend_from_slice(&(((size - 1) << 16) | addr).to_be_bytes());
                            out.extend_from_slice(words);
                        }
                        None => {
                            for i in 0..size {
                                if let Some(w) = mem.ram(src + 4 * i, 4) {
                                    out.push(0x10);
                                    out.extend_from_slice(&(addr + i).to_be_bytes());
                                    out.extend_from_slice(w);
                                }
                            }
                        }
                    }
                }
                0x40 if !in_list => {
                    // A display list: its complete commands inline (a list
                    // cannot call another, so any call in it is dropped).
                    let (addr, size) = (be32(&cmd[1..]), be32(&cmd[5..]) as usize);
                    // Read in place: nothing writes RAM while the game
                    // thread scans.
                    if let Some(list) = mem.ram(addr, size) {
                        self.scan(list, mem, out, events, true);
                    }
                }
                0x40 => {}
                0x61 => {
                    let v = be32(&cmd[1..]);
                    let reg = (v >> 24) as usize;
                    let val = v & 0xFF_FFFF;
                    if reg == 0xFE {
                        self.mask = val;
                    } else {
                        let mask = std::mem::replace(&mut self.mask, 0xFF_FFFF);
                        match reg {
                            0x45 => events.push(Event::Finish),
                            0x47 | 0x48 => {
                                let t = &mut self.tokens[reg - 0x47];
                                *t = (*t & !mask) | (val & mask);
                                events.push(if reg == 0x47 { Event::TokenInt(*t as u16) } else { Event::Token(*t as u16) });
                            }
                            _ => {}
                        }
                    }
                    out.extend_from_slice(cmd);
                }
                _ => out.extend_from_slice(cmd),
            }
            at += need;
        }
        (at, false)
    }
}

enum Msg {
    /// Complete commands (an invalid opcode can only be the last byte).
    Data(Vec<u8>),
    Poke { x: u32, y: u32, value: u32, z: bool },
    Writeback(u32, Vec<u8>),
    Recycle(Box<Output>),
    Records(bool),
    Patches(bool),
    /// Reply once everything before has been decoded.
    Sync,
    /// Reply with the field's output.
    TakeOutput,
    Drop(bool),
    Wide(f32),
    World(Option<crate::WorldCamera>),
    Views([Option<crate::xf::View>; 4]),
    /// Reply with the registers saved ([`Gpu::snap_regs`]).
    SaveRegs,
    /// Load registers saved by `SaveRegs`.
    LoadRegs(Vec<u8>),
    /// Use this texture pack ([`Gpu::set_texture_pack`]).
    TexturePack(Option<Box<crate::replace::Pack>>),
}

enum Reply {
    Synced,
    Output(Box<Output>),
    Regs(Vec<u8>),
}

/// The worker thread and the game thread's side of it.
pub struct GxThread {
    scanner: Scanner,
    batch: Vec<u8>,
    to: Option<Sender<Msg>>,
    from: Receiver<Reply>,
    handle: Option<JoinHandle<()>>,
    /// Time the game thread spent waiting for the worker.
    pub waited: std::time::Duration,
}

fn worker(mut gpu: Gpu, ram: RamView, msgs: Receiver<Msg>, replies: Sender<Reply>) {
    let mut events = Vec::new();
    // Time spent decoding this field (`Output::busy`).
    let mut busy = std::time::Duration::ZERO;
    for msg in msgs {
        match msg {
            Msg::Data(bytes) => {
                let t = std::time::Instant::now();
                gpu.feed(&bytes, &ram, &mut events);
                busy += t.elapsed();
                events.clear();
            }
            Msg::Poke { x, y, value, z } => gpu.poke(x, y, value, z),
            Msg::Writeback(addr, bytes) => gpu.note_writeback(addr, &bytes),
            Msg::Recycle(out) => gpu.recycle(*out),
            Msg::Records(on) => gpu.records_next = on,
            Msg::Patches(on) => gpu.patches = on,
            Msg::Drop(on) => gpu.set_drop(on),
            Msg::Wide(a) => gpu.set_wide(a),
            Msg::World(c) => gpu.set_world(c),
            Msg::Views(v) => gpu.set_views(v),
            Msg::Sync => {
                // Let the transform thread get on with what is decoded.
                gpu.flush_back();
                if replies.send(Reply::Synced).is_err() {
                    return;
                }
            }
            Msg::TakeOutput => {
                let mut out = gpu.take_output();
                out.busy[0] = std::mem::take(&mut busy);
                if replies.send(Reply::Output(Box::new(out))).is_err() {
                    return;
                }
            }
            Msg::SaveRegs => {
                let mut s = guest_mem::snap::Snap::saver();
                gpu.snap_regs(&mut s);
                if replies.send(Reply::Regs(s.into_bytes())).is_err() {
                    return;
                }
            }
            Msg::LoadRegs(bytes) => {
                // Checked by the game thread before it was sent.
                gpu.snap_regs(&mut guest_mem::snap::Snap::loader(bytes));
            }
            Msg::TexturePack(pack) => gpu.set_texture_pack(pack.map(|p| *p)),
        }
    }
}

impl GxThread {
    /// Moves `gpu` to a new thread reading RAM through `ram`, with its
    /// transform half on a third. Both threads run `on_start` first (the
    /// frontend raises their priority with it).
    pub fn spawn(mut gpu: Gpu, ram: RamView, on_start: fn()) -> Self {
        gpu.start_transform_thread(on_start);
        let (to, msgs) = channel();
        let (replies, from) = channel();
        let handle = std::thread::Builder::new()
            .name("gx".into())
            .spawn(move || {
                on_start();
                worker(gpu, ram, msgs, replies)
            })
            .expect("GX thread");
        Self { scanner: Scanner::new(), batch: Vec::with_capacity(BATCH * 2), to: Some(to), from, handle: Some(handle), waited: Default::default() }
    }

    fn send(&self, msg: Msg) {
        self.to.as_ref().expect("GX thread").send(msg).expect("the GX thread stopped");
    }

    fn flush(&mut self) {
        if !self.batch.is_empty() {
            let batch = std::mem::replace(&mut self.batch, Vec::with_capacity(BATCH * 2));
            self.send(Msg::Data(batch));
        }
    }

    fn reply(&mut self) -> Reply {
        let t = std::time::Instant::now();
        let r = self.from.recv().expect("the GX thread stopped");
        self.waited += t.elapsed();
        r
    }

    /// The worker has decoded everything sent so far.
    fn sync(&mut self) {
        self.flush();
        self.send(Msg::Sync);
        match self.reply() {
            Reply::Synced => {}
            _ => unreachable!("unrequested reply"),
        }
    }

    fn feed(&mut self, bytes: &[u8], mem: &Mem, events: &mut Vec<Event>) {
        let before = events.len();
        let bad = self.scanner.feed(bytes, mem, &mut self.batch, events);
        if bad || self.batch.len() >= BATCH {
            self.flush();
        }
        if events.len() > before {
            // The game will act on these (a finish or a token): the GPU has
            // read everything before them.
            self.sync();
        }
    }

    fn request_output(&mut self) {
        self.flush();
        self.send(Msg::TakeOutput);
    }

    fn finish_output(&mut self) -> Output {
        match self.reply() {
            Reply::Output(out) => *out,
            _ => unreachable!("unrequested reply"),
        }
    }
}

impl guest_mem::snap::Snapshot for Scanner {
    fn snap(&mut self, s: &mut guest_mem::snap::Snap) {
        guest_mem::snap!(s; self.cp, self.tokens, self.mask, self.pending);
        if s.loading() {
            self.sizes = [None; 8];
        }
    }
}

impl guest_mem::snap::Snapshot for GpuPort {
    /// The game thread's scanner state, then the decoder's registers as a
    /// nested block (inline, the scanner part is taken from the decoder),
    /// so states move between inline and threaded decoding.
    fn snap(&mut self, s: &mut guest_mem::snap::Snap) {
        s.tag(b"GX  ");
        match self {
            Self::Inline(gpu) => {
                let mut sc = Scanner { cp: gpu.cp.clone(), tokens: [gpu.bp[0x47], gpu.bp[0x48]], mask: gpu.bp_mask, pending: gpu.pending.clone(), ..Scanner::new() };
                sc.snap(s);
                let mut regs = Vec::new();
                if !s.loading() {
                    let mut inner = guest_mem::snap::Snap::saver();
                    gpu.snap_regs(&mut inner);
                    regs = inner.into_bytes();
                }
                regs.snap(s);
                if s.loading() {
                    let mut inner = guest_mem::snap::Snap::loader(regs);
                    gpu.snap_regs(&mut inner);
                    if inner.finish().is_err() {
                        s.fail();
                    }
                }
            }
            Self::Thread(t) => {
                t.scanner.snap(s);
                let mut regs = Vec::new();
                if !s.loading() {
                    t.flush();
                    t.send(Msg::SaveRegs);
                    regs = match t.reply() {
                        Reply::Regs(r) => r,
                        _ => unreachable!("unrequested reply"),
                    };
                }
                regs.snap(s);
                if s.loading() {
                    let mut check = Gpu::default();
                    let mut inner = guest_mem::snap::Snap::loader(regs.clone());
                    check.snap_regs(&mut inner);
                    if inner.finish().is_err() {
                        s.fail();
                        return;
                    }
                    t.flush();
                    t.send(Msg::LoadRegs(regs));
                }
            }
        }
    }
}

impl Drop for GxThread {
    fn drop(&mut self) {
        drop(self.to.take());
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

/// GX decoding inline on the game thread, or on its own thread.
pub enum GpuPort {
    Inline(Box<Gpu>),
    Thread(Box<GxThread>),
}

impl Default for GpuPort {
    fn default() -> Self {
        Self::Inline(Box::default())
    }
}

impl GpuPort {
    /// Time the game thread has waited for the GX thread so far.
    pub fn waited(&self) -> std::time::Duration {
        match self {
            GpuPort::Thread(t) => t.waited,
            GpuPort::Inline(_) => Default::default(),
        }
    }

    /// Moves decoding to its own thread (no-op if it already is); its
    /// threads run `on_start` first.
    pub fn start_thread(&mut self, ram: RamView, on_start: fn()) {
        if let Self::Inline(gpu) = std::mem::take(self) {
            *self = Self::Thread(Box::new(GxThread::spawn(*gpu, ram, on_start)));
        }
    }

    /// Bytes the GP reads from its FIFO; PE events go to `events`.
    pub fn feed(&mut self, bytes: &[u8], mem: &Mem, events: &mut Vec<Event>) {
        match self {
            Self::Inline(gpu) => gpu.feed(bytes, mem, events),
            Self::Thread(t) => t.feed(bytes, mem, events),
        }
    }

    pub fn poke(&mut self, x: u32, y: u32, value: u32, z: bool) {
        match self {
            Self::Inline(gpu) => gpu.poke(x, y, value, z),
            Self::Thread(t) => {
                t.flush();
                t.send(Msg::Poke { x, y, value, z });
            }
        }
    }

    /// The field's output (see [`Gpu::take_output`]).
    pub fn take_output(&mut self) -> Output {
        self.request_output();
        self.finish_output()
    }

    /// Asks for the field's output without waiting for it: the GX threads
    /// finish it while the caller does something else (a paced game thread
    /// sleeps until the field's slot ends). [`GpuPort::finish_output`] takes
    /// it; nothing else may reach the GPU in between.
    pub fn request_output(&mut self) {
        if let Self::Thread(t) = self {
            t.request_output();
        }
    }

    /// The output asked for with [`GpuPort::request_output`].
    pub fn finish_output(&mut self) -> Output {
        match self {
            Self::Inline(gpu) => gpu.take_output(),
            Self::Thread(t) => t.finish_output(),
        }
    }

    pub fn recycle(&mut self, out: Output) {
        match self {
            Self::Inline(gpu) => gpu.recycle(out),
            Self::Thread(t) => t.send(Msg::Recycle(Box::new(out))),
        }
    }

    /// Whether fields from the next one carry interpolation records.
    /// The game camera for the draws sent from now on (see
    /// [`Gpu::set_world`]).
    pub fn set_world(&mut self, camera: Option<crate::WorldCamera>) {
        match self {
            Self::Inline(gpu) => gpu.set_world(camera),
            Self::Thread(t) => {
                t.flush();
                t.send(Msg::World(camera));
            }
        }
    }

    /// Each player's window (see [`Gpu::set_views`]).
    pub fn set_views(&mut self, views: [Option<crate::xf::View>; 4]) {
        match self {
            Self::Inline(gpu) => gpu.set_views(views),
            Self::Thread(t) => {
                t.flush();
                t.send(Msg::Views(views));
            }
        }
    }

    /// Whether EFB copies into part of a texture patch it ([`crate::Cmd::Patch`]).
    pub fn set_patches(&mut self, on: bool) {
        match self {
            Self::Inline(gpu) => gpu.patches = on,
            Self::Thread(t) => t.send(Msg::Patches(on)),
        }
    }

    /// The draws sent from now on are dropped (`on`) or drawn, in order
    /// with the commands.
    pub fn set_drop(&mut self, on: bool) {
        match self {
            Self::Inline(gpu) => gpu.set_drop(on),
            Self::Thread(t) => {
                t.flush();
                t.send(Msg::Drop(on));
            }
        }
    }

    /// The draws sent from now on widen 16:9 projections to `aspect` (0: none).
    pub fn set_wide(&mut self, aspect: f32) {
        match self {
            Self::Inline(gpu) => gpu.set_wide(aspect),
            Self::Thread(t) => {
                t.flush();
                t.send(Msg::Wide(aspect));
            }
        }
    }

    pub fn set_records(&mut self, on: bool) {
        match self {
            Self::Inline(gpu) => gpu.records_next = on,
            Self::Thread(t) => t.send(Msg::Records(on)),
        }
    }

    pub fn note_writeback(&mut self, addr: u32, ram: &[u8]) {
        match self {
            Self::Inline(gpu) => gpu.note_writeback(addr, ram),
            Self::Thread(t) => {
                t.flush();
                t.send(Msg::Writeback(addr, ram.to_vec()));
            }
        }
    }

    /// Draws textures from `pack` from the next draw on (`None`: the
    /// game's own); see [`crate::replace`].
    pub fn set_texture_pack(&mut self, pack: Option<crate::replace::Pack>) {
        match self {
            Self::Inline(gpu) => gpu.set_texture_pack(pack),
            Self::Thread(t) => {
                t.flush();
                t.send(Msg::TexturePack(pack.map(Box::new)));
            }
        }
    }

    /// The decoder, when it runs inline.
    pub fn inline(&self) -> Option<&Gpu> {
        match self {
            Self::Inline(gpu) => Some(gpu),
            Self::Thread(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bp(v: u32) -> [u8; 5] {
        let b = v.to_be_bytes();
        [0x61, b[0], b[1], b[2], b[3]]
    }

    /// The same stream inline and threaded gives the same output and events,
    /// with a display list and an indexed XF load in it.
    #[test]
    fn threaded_matches_inline() {
        let mut mem = Mem::new(Box::new(guest_mem::NullMmio));
        // A display list at 0x80001000: a token, then a BP write.
        let mut list = Vec::new();
        list.extend(bp(0x48_00_12_34));
        list.extend(bp(0x41_00_00_18));
        list.resize(32, 0);
        mem.ram_mut(0x8000_1000, 32).unwrap().copy_from_slice(&list);
        // XF array 12 at 0x80002000: one matrix row.
        for (i, w) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            mem.write_u32(0x8000_2000 + 4 * i as u32, w.to_bits()).unwrap();
        }
        let mut s = Vec::new();
        s.extend([0x08, 0xAC, 0x00, 0x00, 0x20, 0x00]); // CP array base 12 = 0x2000
        s.extend([0x08, 0xBC, 0x00, 0x00, 0x00, 0x10]); // stride 16
        s.extend([0x20, 0x00, 0x00, 0x30, 0x00]); // index 0, 4 words to XF 0
        s.extend([0x40, 0x80, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x20]);
        s.extend(bp(0xFE_00_00_FF)); // mask, then a masked token
        s.extend(bp(0x47_00_00_56));
        s.extend(bp(0x45_00_00_02)); // finish

        let mut inline = GpuPort::default();
        let mut threaded = GpuPort::default();
        threaded.start_thread(mem.view(), || {});
        let (mut e1, mut e2) = (Vec::new(), Vec::new());
        // Fed in uneven pieces, as bursts arrive.
        for piece in s.chunks(7) {
            inline.feed(piece, &mem, &mut e1);
            threaded.feed(piece, &mem, &mut e2);
        }
        assert_eq!(e1, e2);
        assert_eq!(e2, vec![Event::Token(0x1234), Event::TokenInt(0x0056), Event::Finish]);
        let (o1, o2) = (inline.take_output(), threaded.take_output());
        assert_eq!(format!("{:?}", o1.cmds), format!("{:?}", o2.cmds));
    }
}
