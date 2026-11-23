//! IOS high-level emulation: the requests the game sends through the IPC
//! registers (docs/PORT_PLAN.md, section 2.4).
//!
//! A request is 0x40 bytes in guest memory at a physical address:
//! `+0 command, +4 result, +8 fd, +0xC.. arguments`. Replying writes the
//! result, moves the command to `+8` and sets `+0` to 8 (reply), as IOS does.

pub mod bt;
pub mod bt_passthrough;
mod di;
pub mod nand;
pub mod wiimote;

use std::path::{Path, PathBuf};

use guest_mem::Mem;

pub use di::Disc;

const CMD_OPEN: u32 = 1;
const CMD_CLOSE: u32 = 2;
const CMD_READ: u32 = 3;
const CMD_WRITE: u32 = 4;
const CMD_SEEK: u32 = 5;
const CMD_IOCTL: u32 = 6;
const CMD_IOCTLV: u32 = 7;
const CMD_REPLY: u32 = 8;

const IPC_EINVAL: i32 = -4;
const IPC_ENOENT: i32 = -6;
const FS_EINVAL: i32 = -101;
const FS_EEXIST: i32 = -105;
const FS_ENOENT: i32 = -106;
const ES_EINVAL: i32 = -1017;
/// `/dev/di` reports success as 1.
const DI_OK: i32 = 1;
const DI_ERROR: i32 = 2;

struct OpenFile {
    host: PathBuf,
    data: Vec<u8>,
    pos: usize,
    dirty: bool,
}

enum Handle {
    Fs,
    Es,
    StmImmediate,
    StmEventHook,
    Di,
    /// Bluetooth (`/dev/usb/oh1/57e/305`) with the emulated Wii Remotes.
    Bluetooth,
    File(OpenFile),
}

/// Guest buffer `(physical address, length)`.
#[derive(Clone, Copy)]
struct Buf(u32, u32);

pub struct Ios {
    nand: PathBuf,
    pub disc: Disc,
    tmd: Vec<u8>,
    title_id: u64,
    fds: Vec<Option<Handle>>,
    /// Print every request and its result.
    pub trace: bool,
    /// Requests held without a reply (device, request address).
    pub held: Vec<(String, u32)>,
    pub bt: bt::Bt,
    /// Bluetooth passthrough's folder: its pairings, apart from the
    /// emulated remotes' (`start_passthrough`).
    passthrough: Option<PathBuf>,
    /// Extra detail for the trace line of the request being handled.
    note: String,
}

const SYSCONF: &str = "/shared2/sys/SYSCONF";
const LINK_KEYS: &str = "link_keys.txt";
const DINF: &str = "BT.DINF";

fn rd32(mem: &mut Mem, a: u32) -> u32 {
    mem.read_u32(a).unwrap_or(0)
}

fn wr32(mem: &mut Mem, a: u32, v: u32) {
    let _ = mem.write_u32(a, v);
}

fn cstr(mem: &mut Mem, mut a: u32, max: usize) -> String {
    let mut s = Vec::new();
    while s.len() < max {
        match mem.read_u8(a) {
            Ok(0) | Err(_) => break,
            Ok(b) => s.push(b),
        }
        a = a.wrapping_add(1);
    }
    String::from_utf8_lossy(&s).into_owned()
}

fn bytes(mem: &Mem, b: Buf) -> Vec<u8> {
    mem.ram(b.0, b.1 as usize).map(<[u8]>::to_vec).unwrap_or_default()
}

/// Copies `data` into the buffer, truncated to its length; returns bytes copied.
fn put(mem: &mut Mem, b: Buf, data: &[u8]) -> usize {
    let n = data.len().min(b.1 as usize);
    if let Some(dst) = mem.ram_mut(b.0, n) {
        dst.copy_from_slice(&data[..n]);
    }
    n
}

fn zero(mem: &mut Mem, b: Buf) {
    if let Some(dst) = mem.ram_mut(b.0, b.1 as usize) {
        dst.fill(0);
    }
}

impl Ios {
    /// `game` is the extracted disc, `nand` the host folder for NAND files.
    pub fn new(game: &Path, nand: PathBuf, title_id: u64) -> Result<Self, String> {
        nand::ensure_system_files(&nand).map_err(|e| format!("{}: {e}", nand.display()))?;
        // The title's data directory exists once a title has run, as the
        // system menu creates it.
        let data = nand.join(format!("title/{:08x}/{:08x}/data", title_id >> 32, title_id as u32));
        std::fs::create_dir_all(&data).map_err(|e| format!("{}: {e}", data.display()))?;
        let remotes: Vec<[u8; 6]> = (0..bt::REMOTES).map(bt::remote_address).collect();
        nand::register_wiimotes(&nand, &remotes).map_err(|e| format!("{}: {e}", nand.display()))?;
        let tmd = std::fs::read(game.join("tmd.bin")).map_err(|e| format!("tmd.bin: {e}"))?;
        Ok(Self { nand, disc: Disc::open(game)?, tmd, title_id, fds: Vec::new(), trace: false, held: Vec::new(), bt: bt::Bt::default(), passthrough: None, note: String::new() })
    }

    /// Bluetooth passthrough from now on, before the game starts: `adapter`
    /// in place of the emulated remotes (`bt_passthrough`). Its pairings
    /// live in `folder`: the link keys (`link_keys.txt`), and the game's
    /// list of paired remotes (`BT.DINF`), put in `SYSCONF` in place of the
    /// emulated remotes' and copied back whenever the game writes it.
    pub fn start_passthrough(&mut self, adapter: Box<dyn bt_passthrough::Adapter>, wii_module: bool, folder: PathBuf) -> Result<(), String> {
        std::fs::create_dir_all(&folder).map_err(|e| format!("{}: {e}", folder.display()))?;
        let keys = std::fs::read_to_string(folder.join(LINK_KEYS)).map(|t| bt_passthrough::parse_link_keys(&t)).unwrap_or_default();
        let dinf = std::fs::read(folder.join(DINF)).ok().filter(|d| d.len() == 0x461).unwrap_or_else(|| nand::bt_dinf(&[]));
        let path = nand::host_path(&self.nand, SYSCONF).ok_or("SYSCONF")?;
        let mut conf = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if !nand::set_sysconf_big(&mut conf, "BT.DINF", &dinf) {
            return Err("SYSCONF has no BT.DINF".into());
        }
        std::fs::write(&path, conf).map_err(|e| format!("{}: {e}", path.display()))?;
        self.bt.passthrough = Some(bt_passthrough::Passthrough::new(adapter, wii_module, keys));
        self.passthrough = Some(folder);
        Ok(())
    }

    /// Passthrough's pairings to its folder (when the game wrote them).
    fn save_link_keys(&mut self) {
        let (Some(p), Some(folder)) = (&mut self.bt.passthrough, &self.passthrough) else { return };
        if p.take_keys_changed() {
            let _ = std::fs::write(folder.join(LINK_KEYS), bt_passthrough::format_link_keys(&p.link_keys));
        }
    }

    /// The console's sync button (passthrough only): pressed, or held 10 s.
    pub fn sync_button(&mut self, held: bool) {
        if let Some(p) = &mut self.bt.passthrough {
            p.sync_button(held);
        }
    }

    /// The game is done: passthrough's adapter is reset and let go, and
    /// its counts returned (with the game's commands by opcode).
    pub fn stop_passthrough(&mut self) -> Option<(bt_passthrough::Stats, std::collections::BTreeMap<u16, u64>)> {
        self.save_link_keys();
        let mut p = self.bt.passthrough.take()?;
        p.shutdown();
        Some((p.stats, std::mem::take(&mut p.command_counts)))
    }

    /// Runs the request at physical address `req`. Returns true if it was
    /// answered (the reply is already written to guest memory), false if the
    /// device holds it.
    pub fn execute(&mut self, mem: &mut Mem, req: u32) -> bool {
        let cmd = rd32(mem, req);
        let fd = rd32(mem, req + 8) as i32;
        let arg: [u32; 5] = std::array::from_fn(|i| rd32(mem, req + 0xC + 4 * i as u32));
        self.note.clear();
        let (what, result) = self.dispatch(mem, req, cmd, fd, arg);
        if self.trace {
            match result {
                Some(r) => println!("  [ios] {what} -> {r}{}", self.note),
                None => println!("  [ios] {what} -> held{}", self.note),
            }
        }
        match result {
            Some(r) => {
                write_reply(mem, req, r);
                true
            }
            None => {
                self.held.push((what, req));
                false
            }
        }
    }

    /// Advances the Wii Remotes to guest cycle `now` and answers held
    /// requests that can complete (Bluetooth events and ACL data); returns
    /// their addresses for the IPC reply queue.
    pub fn poll(&mut self, mem: &mut Mem, now: u64) -> Vec<u32> {
        self.bt.update(now);
        self.save_link_keys();
        let mut done = Vec::new();
        while self.bt.has_event() && !self.bt.intr_waiting.is_empty() {
            let (req, buf, len) = self.bt.intr_waiting.pop_front().unwrap();
            let ev = self.bt.next_event().unwrap();
            let n = put(mem, Buf(buf, len), &ev);
            write_reply(mem, req, n as i32);
            done.push(req);
        }
        while self.bt.has_acl() && !self.bt.bulk_waiting.is_empty() {
            let (req, buf, len) = self.bt.bulk_waiting.pop_front().unwrap();
            let p = self.bt.next_acl().unwrap();
            let n = put(mem, Buf(buf, len), &p);
            write_reply(mem, req, n as i32);
            done.push(req);
        }
        if !done.is_empty() {
            self.held.retain(|h| !done.contains(&h.1));
        }
        done
    }

    fn dispatch(&mut self, mem: &mut Mem, req: u32, cmd: u32, fd: i32, arg: [u32; 5]) -> (String, Option<i32>) {
        if cmd == CMD_OPEN {
            let path = cstr(mem, arg[0], 64);
            let r = self.open(&path, arg[1]);
            return (format!("open {path:?} mode {}", arg[1]), Some(r));
        }
        let Some(Some(h)) = self.fds.get_mut(fd as usize) else {
            return (format!("command {cmd} on bad fd {fd}"), Some(IPC_EINVAL));
        };
        let dev = match h {
            Handle::Fs => "fs",
            Handle::Es => "es",
            Handle::StmImmediate => "stm/immediate",
            Handle::StmEventHook => "stm/eventhook",
            Handle::Di => "di",
            Handle::Bluetooth => "bt",
            Handle::File(_) => "file",
        };
        let what = format!("{dev}#{fd} cmd {cmd} args {:08x} {:08x} {:08x} {:08x} {:08x}", arg[0], arg[1], arg[2], arg[3], arg[4]);
        let r = match cmd {
            CMD_CLOSE => {
                if let Some(Handle::File(f)) = self.fds[fd as usize].take() {
                    if f.dirty {
                        let _ = std::fs::write(&f.host, &f.data);
                        // Passthrough's paired remotes, as the game left them.
                        if let Some(folder) = self.passthrough.as_ref().filter(|_| nand::host_path(&self.nand, SYSCONF).as_ref() == Some(&f.host)) {
                            if let Some(dinf) = nand::sysconf_big(&f.data, "BT.DINF") {
                                let _ = std::fs::write(folder.join(DINF), dinf);
                            }
                        }
                    }
                }
                Some(0)
            }
            CMD_READ | CMD_WRITE | CMD_SEEK => match h {
                Handle::File(f) => Some(file_io(mem, f, cmd, arg)),
                _ => Some(IPC_EINVAL),
            },
            CMD_IOCTL => {
                let (inb, out) = (Buf(arg[1], arg[2]), Buf(arg[3], arg[4]));
                let i = arg[0];
                match h {
                    Handle::Fs => Some(self.fs_ioctl(mem, i, inb, out)),
                    Handle::File(f) if i == 11 => {
                        // GetFileStats: size, position.
                        wr32(mem, out.0, f.data.len() as u32);
                        wr32(mem, out.0 + 4, f.pos as u32);
                        Some(0)
                    }
                    Handle::StmImmediate => Some(0),
                    Handle::StmEventHook => None,
                    Handle::Di => Some(self.di_ioctl(mem, i, inb, out)),
                    Handle::Bluetooth => None,
                    _ => Some(IPC_EINVAL),
                }
            }
            CMD_IOCTLV => {
                let (nin, nio) = (arg[1] as usize, arg[2] as usize);
                let vecs: Vec<Buf> = (0..nin + nio).map(|k| Buf(rd32(mem, arg[3] + 8 * k as u32), rd32(mem, arg[3] + 8 * k as u32 + 4))).collect();
                let (ins, ios) = vecs.split_at(nin);
                match h {
                    Handle::Fs => Some(self.fs_ioctlv(mem, arg[0], ins, ios)),
                    Handle::Es => Some(self.es_ioctlv(mem, arg[0], ins, ios)),
                    Handle::Di => Some(self.di_ioctlv(mem, arg[0], ios)),
                    Handle::Bluetooth => self.bt_ioctlv(mem, arg[0], ins, ios, req),
                    _ => Some(IPC_EINVAL),
                }
            }
            _ => Some(IPC_EINVAL),
        };
        (what, r)
    }

    fn alloc_fd(&mut self, h: Handle) -> i32 {
        match self.fds.iter().position(Option::is_none) {
            Some(i) => {
                self.fds[i] = Some(h);
                i as i32
            }
            None => {
                self.fds.push(Some(h));
                self.fds.len() as i32 - 1
            }
        }
    }

    fn open(&mut self, path: &str, mode: u32) -> i32 {
        let h = match path {
            "/dev/fs" => Handle::Fs,
            "/dev/es" => Handle::Es,
            "/dev/stm/immediate" => Handle::StmImmediate,
            "/dev/stm/eventhook" => Handle::StmEventHook,
            "/dev/di" => Handle::Di,
            "/dev/usb/oh1/57e/305" => Handle::Bluetooth,
            p if p.starts_with("/dev/") => return IPC_ENOENT,
            p => {
                let Some(host) = nand::host_path(&self.nand, p) else { return FS_EINVAL };
                let data = match std::fs::read(&host) {
                    Ok(d) => d,
                    Err(_) => return FS_ENOENT,
                };
                let _ = mode;
                Handle::File(OpenFile { host, data, pos: 0, dirty: false })
            }
        };
        self.alloc_fd(h)
    }

    fn nand_path(&mut self, mem: &mut Mem, at: u32) -> Option<PathBuf> {
        let path = cstr(mem, at, 64);
        self.note.push_str(&format!(" ({path})"));
        nand::host_path(&self.nand, &path)
    }

    fn fs_ioctl(&mut self, mem: &mut Mem, i: u32, inb: Buf, out: Buf) -> i32 {
        match i {
            // GetStats: cluster size, free/used clusters, inodes...
            2 => {
                zero(mem, out);
                wr32(mem, out.0, 0x4000);
                wr32(mem, out.0 + 4, 0x5DEC);
                0
            }
            // CreateDirectory / CreateFile: attribute block, path at +6.
            3 | 9 => {
                let Some(p) = self.nand_path(mem, inb.0 + 6) else { return FS_EINVAL };
                if p.exists() {
                    return FS_EEXIST;
                }
                let ok = if i == 3 {
                    std::fs::create_dir_all(&p).is_ok()
                } else {
                    p.parent().is_some_and(|d| d.is_dir()) && std::fs::write(&p, []).is_ok()
                };
                if ok {
                    0
                } else {
                    FS_ENOENT
                }
            }
            // SetAttr: accepted, attributes are not stored.
            5 => 0,
            // GetAttr: in = path; out = owner, group, path, modes, attribute.
            6 => {
                let path = cstr(mem, inb.0, 64);
                self.note = format!(" ({path})");
                let Some(p) = nand::host_path(&self.nand, &path) else { return FS_EINVAL };
                if !p.exists() {
                    return FS_ENOENT;
                }
                zero(mem, out);
                wr32(mem, out.0, 0x1000);
                put(mem, Buf(out.0 + 6, 64), path.as_bytes());
                put(mem, Buf(out.0 + 0x46, 3), &[3, 3, 3]);
                0
            }
            // Delete
            7 => {
                let Some(p) = self.nand_path(mem, inb.0) else { return FS_EINVAL };
                let r = if p.is_dir() { std::fs::remove_dir_all(&p) } else { std::fs::remove_file(&p) };
                if r.is_ok() {
                    0
                } else {
                    FS_ENOENT
                }
            }
            // Rename: two 64-byte paths.
            8 => {
                let (Some(a), Some(b)) = (self.nand_path(mem, inb.0), self.nand_path(mem, inb.0 + 64)) else { return FS_EINVAL };
                if std::fs::rename(&a, &b).is_ok() {
                    0
                } else {
                    FS_ENOENT
                }
            }
            _ => FS_EINVAL,
        }
    }

    fn fs_ioctlv(&mut self, mem: &mut Mem, i: u32, ins: &[Buf], ios: &[Buf]) -> i32 {
        let Some(&path_buf) = ins.first() else { return FS_EINVAL };
        let Some(p) = self.nand_path(mem, path_buf.0) else { return FS_EINVAL };
        match i {
            // ReadDirectory: with only a count vector, return the count;
            // otherwise packed NUL-terminated names, then the count.
            4 => {
                // A file is not a directory: "invalid", which the SDK takes
                // as "exists" (games probe their save files this way).
                if p.is_file() {
                    return FS_EINVAL;
                }
                let Ok(rd) = std::fs::read_dir(&p) else { return FS_ENOENT };
                let mut names: Vec<String> = rd.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).collect();
                names.sort();
                if ios.len() == 1 {
                    wr32(mem, ios[0].0, names.len() as u32);
                } else if ins.len() >= 2 && ios.len() >= 2 {
                    let max = rd32(mem, ins[1].0) as usize;
                    names.truncate(max);
                    let mut packed = Vec::new();
                    for n in &names {
                        packed.extend_from_slice(n.as_bytes());
                        packed.push(0);
                    }
                    zero(mem, ios[0]);
                    put(mem, ios[0], &packed);
                    wr32(mem, ios[1].0, names.len() as u32);
                }
                0
            }
            // GetUsage: clusters, inodes.
            12 => {
                if !p.exists() {
                    return FS_ENOENT;
                }
                let (mut size, mut inodes) = (0u64, 0u32);
                let mut stack = vec![p];
                while let Some(d) = stack.pop() {
                    inodes += 1;
                    if let Ok(rd) = std::fs::read_dir(&d) {
                        for e in rd.flatten() {
                            match e.metadata() {
                                Ok(m) if m.is_dir() => stack.push(e.path()),
                                Ok(m) => {
                                    inodes += 1;
                                    size += m.len().div_ceil(0x4000);
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
                if ios.len() >= 2 {
                    wr32(mem, ios[0].0, size as u32);
                    wr32(mem, ios[1].0, inodes);
                }
                0
            }
            _ => FS_EINVAL,
        }
    }

    fn es_ioctlv(&mut self, mem: &mut Mem, i: u32, ins: &[Buf], ios: &[Buf]) -> i32 {
        match (i, ios.first()) {
            // GetDeviceID
            (0x07, Some(&o)) => {
                wr32(mem, o.0, 0x0403_AC68);
                0
            }
            // GetNumTicketViews
            (0x12, Some(&o)) => {
                wr32(mem, o.0, 1);
                0
            }
            // GetConsumption: no limits, zero entries.
            (0x16, _) => {
                if let Some(&c) = ios.get(1) {
                    wr32(mem, c.0, 0);
                }
                0
            }
            // DIGetTicketView: the running title's ticket view.
            (0x1B, Some(&o)) => {
                zero(mem, o);
                let _ = mem.write_u64(o.0 + 0x10, self.title_id);
                0
            }
            // GetTitleDir(title id) -> "/title/xxxxxxxx/xxxxxxxx/data"
            (0x1D, Some(&o)) => {
                let tid = ins.first().and_then(|b| mem.read_u64(b.0).ok()).unwrap_or(self.title_id);
                let s = format!("/title/{:08x}/{:08x}/data\0", tid >> 32, tid as u32);
                zero(mem, o);
                put(mem, o, s.as_bytes());
                0
            }
            // GetTitleID
            (0x20, Some(&o)) => {
                let _ = mem.write_u64(o.0, self.title_id);
                0
            }
            _ => ES_EINVAL,
        }
    }

    fn di_ioctl(&mut self, mem: &mut Mem, i: u32, inb: Buf, out: Buf) -> i32 {
        match i {
            // Inquiry: drive revision and date.
            0x12 => {
                zero(mem, out);
                wr32(mem, out.0, 0x0002);
                wr32(mem, out.0 + 4, 0x2006_0526);
                DI_OK
            }
            // ReadDiscID: the first 0x20 bytes of the header, also mirrored at 0.
            0x70 => {
                let h = self.disc.header[..0x20].to_vec();
                put(mem, out, &h);
                DI_OK
            }
            // Read / UnencryptedRead: in = [cmd, length, offset >> 2].
            0x71 | 0x8D => {
                let len = rd32(mem, inb.0 + 4) as usize;
                let off = u64::from(rd32(mem, inb.0 + 8)) << 2;
                let mut buf = vec![0u8; len.min(out.1 as usize)];
                if let Some(f) = self.disc.file_at(off) {
                    self.note = format!(" ({}+{:#x})", f.file_name().map_or(f.to_string_lossy(), |n| n.to_string_lossy()), off);
                }
                if !self.disc.read(off, &mut buf) {
                    return DI_ERROR;
                }
                put(mem, out, &buf);
                DI_OK
            }
            // GetCoverRegister: cover closed. GetCoverStatus: disc present.
            0x7A | 0xE0 => {
                wr32(mem, out.0, 0);
                DI_OK
            }
            0x88 => {
                wr32(mem, out.0, 2);
                DI_OK
            }
            // ClearCoverInterrupt, Reset, ClosePartition, StopMotor, AudioBufferConfig, ...
            _ => DI_OK,
        }
    }

    /// USB messages: 0 control (HCI command), 1 bulk (ACL), 2 interrupt (HCI event).
    fn bt_ioctlv(&mut self, mem: &mut Mem, i: u32, ins: &[Buf], ios: &[Buf], req: u32) -> Option<i32> {
        let &data = ios.first()?;
        match i {
            0 => {
                let packet = bytes(mem, data);
                self.bt.command(&packet);
                Some(data.1 as i32)
            }
            1 | 2 => {
                let endpoint = ins.first().and_then(|b| mem.read_u8(b.0).ok()).unwrap_or(0);
                if endpoint & 0x80 == 0 {
                    // ACL data out.
                    if i == 1 {
                        let packet = bytes(mem, data);
                        self.bt.acl(&packet);
                    }
                    return Some(data.1 as i32);
                }
                let w = (req, data.0, data.1);
                if i == 2 {
                    self.bt.intr_waiting.push_back(w);
                } else {
                    self.bt.bulk_waiting.push_back(w);
                }
                None
            }
            _ => Some(IPC_EINVAL),
        }
    }

    fn di_ioctlv(&mut self, mem: &mut Mem, i: u32, ios: &[Buf]) -> i32 {
        match i {
            // OpenPartition: io[0] receives the TMD, io[1] the ES result.
            0x8B => {
                if let Some(&t) = ios.first() {
                    let tmd = self.tmd.clone();
                    put(mem, t, &tmd);
                }
                if let Some(&e) = ios.get(1) {
                    wr32(mem, e.0, 0);
                }
                DI_OK
            }
            _ => DI_OK,
        }
    }
}

fn write_reply(mem: &mut Mem, req: u32, result: i32) {
    let cmd = rd32(mem, req);
    wr32(mem, req + 4, result as u32);
    wr32(mem, req + 8, cmd);
    wr32(mem, req, CMD_REPLY);
}

fn file_io(mem: &mut Mem, f: &mut OpenFile, cmd: u32, arg: [u32; 5]) -> i32 {
    match cmd {
        CMD_READ => {
            let n = (arg[1] as usize).min(f.data.len().saturating_sub(f.pos));
            let chunk = f.data[f.pos..f.pos + n].to_vec();
            put(mem, Buf(arg[0], n as u32), &chunk);
            f.pos += n;
            n as i32
        }
        CMD_WRITE => {
            let data = bytes(mem, Buf(arg[0], arg[1]));
            let end = f.pos + data.len();
            if f.data.len() < end {
                f.data.resize(end, 0);
            }
            f.data[f.pos..end].copy_from_slice(&data);
            f.pos = end;
            f.dirty = true;
            data.len() as i32
        }
        _ => {
            let base = match arg[1] {
                0 => 0,
                1 => f.pos as i64,
                2 => f.data.len() as i64,
                _ => return FS_EINVAL,
            };
            let to = base + i64::from(arg[0] as i32);
            if to < 0 || to as usize > f.data.len() {
                return FS_EINVAL;
            }
            f.pos = to as usize;
            to as i32
        }
    }
}

use guest_mem::snap::{Snap, Snapshot};

/// Open descriptors and the Bluetooth stack. An open file is saved with its
/// path relative to the NAND root, so it reopens under the loading
/// machine's own root. The disc, the TMD and the NAND root itself are the
/// loading machine's.
impl Snapshot for Ios {
    fn snap(&mut self, s: &mut Snap) {
        s.tag(b"IOS ");
        let n = s.len(self.fds.len(), 1 << 16);
        if s.loading() {
            self.fds.clear();
            self.fds.resize_with(n, || None);
        }
        for fd in &mut self.fds {
            let mut tag: u8 = match fd {
                None => 0,
                Some(Handle::Fs) => 1,
                Some(Handle::Es) => 2,
                Some(Handle::StmImmediate) => 3,
                Some(Handle::StmEventHook) => 4,
                Some(Handle::Di) => 5,
                Some(Handle::Bluetooth) => 6,
                Some(Handle::File(_)) => 7,
            };
            tag.snap(s);
            if s.loading() {
                *fd = match tag {
                    1 => Some(Handle::Fs),
                    2 => Some(Handle::Es),
                    3 => Some(Handle::StmImmediate),
                    4 => Some(Handle::StmEventHook),
                    5 => Some(Handle::Di),
                    6 => Some(Handle::Bluetooth),
                    7 => Some(Handle::File(OpenFile { host: PathBuf::new(), data: Vec::new(), pos: 0, dirty: false })),
                    _ => None,
                };
            }
            if let Some(Handle::File(f)) = fd {
                let mut rel = f.host.strip_prefix(&self.nand).map(Path::to_path_buf).unwrap_or_default();
                guest_mem::snap!(s; rel, f.data, f.pos, f.dirty);
                if s.loading() {
                    f.host = self.nand.join(rel);
                }
            }
        }
        guest_mem::snap!(s; self.held, self.bt);
    }
}
