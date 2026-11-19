//! Guest memory for the Wii (Broadway) address space, big-endian.
//!
//! The game runs with the standard BAT setup, so effective addresses map as:
//!
//! | Effective | Physical | Backing |
//! |---|---|---|
//! | `0x80000000..0x81800000` (cached) / `0xC0000000..` (uncached) | `0x00000000..0x01800000` | MEM1, 24 MiB |
//! | `0x90000000..0x94000000` (cached) / `0xD0000000..` (uncached) | `0x10000000..0x14000000` | MEM2, 64 MiB |
//! | `0xCC000000..0xCE000000`, `0xC8000000..0xC9000000` | `0x0C000000..`, `0x08000000..` | hardware registers, EFB: [`Mmio`] |
//! | `0xE0000000..0xE0004000` | - | locked L1 cache used as scratch |
//!
//! Physical addresses are accepted too, because exception handlers run with
//! translation off. BATs are not modelled: the game never changes them.

mod flat;
pub mod snap;

pub const MEM1_SIZE: usize = 24 << 20;
pub const MEM2_SIZE: usize = 64 << 20;
pub const LOCKED_CACHE_SIZE: usize = 16 << 10;

/// An access to an address with no backing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fault {
    pub addr: u32,
    pub write: bool,
}

/// Hardware registers and the EFB window. `size` is 1, 2 or 4 bytes.
pub trait Mmio {
    fn read(&mut self, addr: u32, size: u32) -> u32;
    fn write(&mut self, addr: u32, size: u32, value: u32);
}

/// Reads return 0 and writes are dropped.
pub struct NullMmio;

impl Mmio for NullMmio {
    fn read(&mut self, _addr: u32, _size: u32) -> u32 {
        0
    }
    fn write(&mut self, _addr: u32, _size: u32, _value: u32) {}
}

/// Offsets of the banks inside [`Mem::ram`].
const MEM2_BASE: usize = MEM1_SIZE;
const LOCKED_BASE: usize = MEM1_SIZE + MEM2_SIZE;

/// Fast path: every 4 MiB page of MEM1 and MEM2, in all their mirrors, maps
/// straight to an offset in `ram`; anything else (MMIO, the locked cache,
/// unmapped) takes the slow path.
const PAGE_SHIFT: u32 = 22;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const NO_PAGE: u32 = u32::MAX;

#[derive(Clone, Copy)]
enum Loc {
    Ram(usize),
    Mmio,
}

fn is_mmio_phys(p: u32) -> bool {
    (0x0C00_0000..0x0E00_0000).contains(&p) || (0x0800_0000..0x0900_0000).contains(&p)
}

/// Where `len` bytes at `addr` live; `None` if unmapped or straddling a boundary.
fn locate(addr: u32, len: usize) -> Option<Loc> {
    let low = addr & 0x0FFF_FFFF;
    let (base, off, size) = match addr >> 28 {
        0x8 | 0xC | 0x0 if (low as usize) < MEM1_SIZE => (0, low as usize, MEM1_SIZE),
        0x9 | 0xD | 0x1 if (low as usize) < MEM2_SIZE => (MEM2_BASE, low as usize, MEM2_SIZE),
        0xC | 0x0 if is_mmio_phys(low) => return Some(Loc::Mmio),
        0xE if (low as usize) < LOCKED_CACHE_SIZE => (LOCKED_BASE, low as usize, LOCKED_CACHE_SIZE),
        _ => return None,
    };
    (off + len <= size).then_some(Loc::Ram(base + off))
}

fn page_table() -> Box<[u32; 1024]> {
    let mut t = Box::new([NO_PAGE; 1024]);
    for prefix in [0x0usize, 0x8, 0xC] {
        for p in 0..MEM1_SIZE / PAGE_SIZE {
            t[prefix << 6 | p] = (p * PAGE_SIZE) as u32;
        }
    }
    for prefix in [0x1usize, 0x9, 0xD] {
        for p in 0..MEM2_SIZE / PAGE_SIZE {
            t[prefix << 6 | p] = (MEM2_BASE + p * PAGE_SIZE) as u32;
        }
    }
    t
}

/// The RAM (MEM1, MEM2, locked cache), shared by [`Mem`] and any
/// [`RamView`]s so a reader on another thread keeps it alive. `ptr` is its
/// contiguous view; see [`flat`] for the guest-address views.
struct RamBuf {
    ptr: *mut u8,
    _backing: flat::Backing,
}

// SAFETY: plain memory; see `RamView` for how it is shared between threads.
unsafe impl Send for RamBuf {}
unsafe impl Sync for RamBuf {}

/// Reading guest RAM by effective address.
pub trait Ram {
    /// RAM bytes at `addr`, or `None` for MMIO or unmapped addresses.
    fn ram(&self, addr: u32, len: usize) -> Option<&[u8]>;
}

/// A read-only view of guest RAM for another thread (the GX thread).
///
/// The guest CPU keeps writing while a view reads, as the real GPU reads RAM
/// while the CPU runs: the emulator only lets the reader touch memory the
/// game has handed to the GPU (vertex arrays, textures), and synchronises at
/// the GPU's finish and token events, as the hardware does. Reads of bytes
/// being written at the same moment may see either value, but never memory
/// outside the allocation, which the view keeps alive.
#[derive(Clone)]
pub struct RamView {
    buf: std::sync::Arc<RamBuf>,
}

impl Ram for RamView {
    fn ram(&self, addr: u32, len: usize) -> Option<&[u8]> {
        match locate(addr, len)? {
            // SAFETY: `locate` keeps `off + len` inside the allocation, which
            // `buf` keeps alive; see the type's notes on concurrent writes.
            Loc::Ram(off) => Some(unsafe { std::slice::from_raw_parts(self.buf.ptr.add(off), len) }),
            Loc::Mmio => None,
        }
    }
}

impl Ram for Mem {
    fn ram(&self, addr: u32, len: usize) -> Option<&[u8]> {
        Mem::ram(self, addr, len)
    }
}

pub struct Mem {
    /// MEM1, then MEM2, then the locked cache (`buf`'s bytes).
    ram: &'static mut [u8],
    buf: std::sync::Arc<RamBuf>,
    pages: Box<[u32; 1024]>,
    pub mmio: Box<dyn Mmio>,
    /// Per 4 KiB of `ram`: whether decoded code lives there, and a
    /// generation bumped by every write to such a page (stores and
    /// [`Mem::ram_mut`]), so code caches can tell when to decode again.
    code: Vec<bool>,
    code_gen: Vec<u32>,
    /// Bumped with any code page's generation: unchanged means no code
    /// anywhere was written (recompiled code checks only this on a call).
    code_epoch: u32,
    /// RAM at every guest address it appears at (`flat` module), or null.
    flat: *mut u8,
}

/// Granularity of code-write tracking.
pub const CODE_PAGE_SHIFT: u32 = 12;

impl Mem {
    pub fn new(mmio: Box<dyn Mmio>) -> Self {
        let size = MEM1_SIZE + MEM2_SIZE + LOCKED_CACHE_SIZE;
        let code_pages = size >> CODE_PAGE_SHIFT;
        let backing = flat::Backing::new();
        let (ptr, flat) = (backing.ptr, backing.flat);
        debug_assert!(size <= flat::SECTION_SIZE);
        let buf = std::sync::Arc::new(RamBuf { ptr, _backing: backing });
        // SAFETY: `buf` owns the memory and lives as long as `self`; the
        // slice never leaves `Mem` with the `'static` lifetime.
        let ram = unsafe { std::slice::from_raw_parts_mut(ptr, size) };
        Self { ram, buf, pages: page_table(), mmio, code: vec![false; code_pages], code_gen: vec![0; code_pages], code_epoch: 0, flat }
    }

    /// Whether guest addresses map straight to host memory ([`Mem::raw_read_u32`] and co.).
    pub fn is_flat(&self) -> bool {
        !self.flat.is_null()
    }

    /// `N` bytes at `addr`, which the caller knows is RAM (recompiled code:
    /// stack and small-data accesses), with no lookup. Without the flat
    /// mapping, the checked path.
    #[inline(always)]
    fn raw_read<const N: usize>(&mut self, addr: u32) -> Result<[u8; N], Fault> {
        if self.flat.is_null() {
            return self.read_n(addr);
        }
        // SAFETY: the reservation spans every 32-bit address, so this stays
        // inside it (bar the last N-1 bytes of the address space, which no
        // stack or small-data access reaches); RAM is mapped there, and
        // anything else faults (access violation) instead of reading other
        // memory.
        Ok(unsafe { self.flat.add(addr as usize).cast::<[u8; N]>().read_unaligned() })
    }

    /// Like [`Mem::raw_read`]. It does not count writes to code pages:
    /// stacks and small data hold no code.
    #[inline(always)]
    fn raw_write<const N: usize>(&mut self, addr: u32, bytes: [u8; N]) -> Result<(), Fault> {
        if self.flat.is_null() {
            return self.write_n(addr, bytes);
        }
        // SAFETY: as in `raw_read`.
        unsafe { self.flat.add(addr as usize).cast::<[u8; N]>().write_unaligned(bytes) };
        Ok(())
    }

    #[inline(always)]
    pub fn raw_read_u8(&mut self, addr: u32) -> Result<u8, Fault> {
        Ok(self.raw_read::<1>(addr)?[0])
    }
    #[inline(always)]
    pub fn raw_read_u16(&mut self, addr: u32) -> Result<u16, Fault> {
        Ok(u16::from_be_bytes(self.raw_read(addr)?))
    }
    #[inline(always)]
    pub fn raw_read_u32(&mut self, addr: u32) -> Result<u32, Fault> {
        Ok(u32::from_be_bytes(self.raw_read(addr)?))
    }
    #[inline(always)]
    pub fn raw_read_u64(&mut self, addr: u32) -> Result<u64, Fault> {
        Ok(u64::from_be_bytes(self.raw_read(addr)?))
    }
    #[inline(always)]
    pub fn raw_write_u8(&mut self, addr: u32, v: u8) -> Result<(), Fault> {
        self.raw_write(addr, [v])
    }
    #[inline(always)]
    pub fn raw_write_u16(&mut self, addr: u32, v: u16) -> Result<(), Fault> {
        self.raw_write(addr, v.to_be_bytes())
    }
    #[inline(always)]
    pub fn raw_write_u32(&mut self, addr: u32, v: u32) -> Result<(), Fault> {
        self.raw_write(addr, v.to_be_bytes())
    }
    #[inline(always)]
    pub fn raw_write_u64(&mut self, addr: u32, v: u64) -> Result<(), Fault> {
        self.raw_write(addr, v.to_be_bytes())
    }

    /// A read-only view of RAM for another thread.
    pub fn view(&self) -> RamView {
        RamView { buf: self.buf.clone() }
    }

    /// A write touched `ram[off..off + len]`: bump any code page in it.
    #[inline(always)]
    fn wrote(&mut self, off: usize, len: usize) {
        let (a, b) = (off >> CODE_PAGE_SHIFT, (off + len.max(1) - 1) >> CODE_PAGE_SHIFT);
        // SAFETY: `off + len` lies within `ram`, and `code` has an entry per
        // 4 KiB of it.
        if unsafe { *self.code.get_unchecked(a) | *self.code.get_unchecked(b) } {
            self.wrote_code(a, b);
        }
    }

    #[cold]
    fn wrote_code(&mut self, a: usize, b: usize) {
        for p in a..=b {
            if self.code[p] {
                self.code_gen[p] = self.code_gen[p].wrapping_add(1);
                self.code_epoch = self.code_epoch.wrapping_add(1);
            }
        }
    }

    /// Marks the `len` bytes at `addr` as code and returns their first and
    /// last code page (indices for [`Mem::code_gen`]), or `None` outside RAM.
    pub fn mark_code(&mut self, addr: u32, len: usize) -> Option<(u32, u32)> {
        let Some(Loc::Ram(off)) = locate(addr, len) else { return None };
        let (a, b) = (off >> CODE_PAGE_SHIFT, (off + len.max(1) - 1) >> CODE_PAGE_SHIFT);
        self.code[a..=b].fill(true);
        Some((a as u32, b as u32))
    }

    /// A 64-bit hash of all of RAM (MEM1, MEM2, the locked cache), for
    /// checking that two machines in lockstep still agree.
    pub fn hash(&self) -> u64 {
        let mut h: u64 = 0xCBF2_9CE4_8422_2325;
        for c in self.ram.chunks_exact(8) {
            h = (h ^ u64::from_le_bytes(c.try_into().unwrap())).wrapping_mul(0x9E37_79B9_7F4A_7C15).rotate_left(23);
        }
        h
    }

    /// The write generation of code page `page`.
    #[inline(always)]
    pub fn code_gen(&self, page: u32) -> u32 {
        self.code_gen[page as usize]
    }

    /// Changes whenever any code page's generation does.
    #[inline(always)]
    pub fn code_epoch(&self) -> u32 {
        self.code_epoch
    }

    /// Offset in `ram` of `n` bytes at `addr` if they sit inside one RAM page.
    #[inline(always)]
    fn fast(&self, addr: u32, n: usize) -> Option<usize> {
        let base = self.pages[(addr >> PAGE_SHIFT) as usize];
        let in_page = (addr as usize) & (PAGE_SIZE - 1);
        (base != NO_PAGE && in_page + n <= PAGE_SIZE).then(|| base as usize + in_page)
    }

    /// RAM bytes at `addr`, or `None` for MMIO or unmapped addresses.
    pub fn ram(&self, addr: u32, len: usize) -> Option<&[u8]> {
        match locate(addr, len)? {
            Loc::Ram(off) => Some(&self.ram[off..off + len]),
            Loc::Mmio => None,
        }
    }

    pub fn ram_mut(&mut self, addr: u32, len: usize) -> Option<&mut [u8]> {
        match locate(addr, len)? {
            Loc::Ram(off) => {
                // Bulk writes (DMA, IOS replies) count as writes to any code there.
                let (a, b) = (off >> CODE_PAGE_SHIFT, (off + len.max(1) - 1) >> CODE_PAGE_SHIFT);
                self.wrote_code(a, b.min(self.code.len() - 1));
                Some(&mut self.ram[off..off + len])
            }
            Loc::Mmio => None,
        }
    }

    #[inline(always)]
    fn read_n<const N: usize>(&mut self, addr: u32) -> Result<[u8; N], Fault> {
        match self.fast(addr, N) {
            // SAFETY: `fast` returns offsets of N bytes inside one RAM page.
            Some(o) => Ok(unsafe { self.ram.as_ptr().add(o).cast::<[u8; N]>().read_unaligned() }),
            None => self.read_slow(addr),
        }
    }

    #[cold]
    #[inline(never)]
    fn read_slow<const N: usize>(&mut self, addr: u32) -> Result<[u8; N], Fault> {
        match locate(addr, N) {
            Some(Loc::Ram(off)) => Ok(self.ram[off..off + N].try_into().unwrap()),
            Some(Loc::Mmio) if N <= 4 => {
                let v = self.mmio.read(addr, N as u32);
                let bytes = v.to_be_bytes();
                Ok(bytes[4 - N..].try_into().unwrap())
            }
            _ => Err(Fault { addr, write: false }),
        }
    }

    #[inline(always)]
    fn write_n<const N: usize>(&mut self, addr: u32, bytes: [u8; N]) -> Result<(), Fault> {
        match self.fast(addr, N) {
            Some(o) => {
                // SAFETY: `fast` returns offsets of N bytes inside one RAM page.
                unsafe { self.ram.as_mut_ptr().add(o).cast::<[u8; N]>().write_unaligned(bytes) };
                self.wrote(o, N);
                Ok(())
            }
            None => self.write_slow(addr, bytes),
        }
    }

    #[cold]
    #[inline(never)]
    fn write_slow<const N: usize>(&mut self, addr: u32, bytes: [u8; N]) -> Result<(), Fault> {
        match locate(addr, N) {
            Some(Loc::Ram(off)) => {
                self.ram[off..off + N].copy_from_slice(&bytes);
                self.wrote(off, N);
                Ok(())
            }
            Some(Loc::Mmio) if N <= 4 => {
                let mut v = [0u8; 4];
                v[4 - N..].copy_from_slice(&bytes);
                self.mmio.write(addr, N as u32, u32::from_be_bytes(v));
                Ok(())
            }
            _ => Err(Fault { addr, write: true }),
        }
    }

    #[inline(always)]
    pub fn read_u8(&mut self, addr: u32) -> Result<u8, Fault> {
        Ok(self.read_n::<1>(addr)?[0])
    }
    #[inline(always)]
    pub fn read_u16(&mut self, addr: u32) -> Result<u16, Fault> {
        Ok(u16::from_be_bytes(self.read_n(addr)?))
    }
    #[inline(always)]
    pub fn read_u32(&mut self, addr: u32) -> Result<u32, Fault> {
        Ok(u32::from_be_bytes(self.read_n(addr)?))
    }
    #[inline(always)]
    pub fn read_u64(&mut self, addr: u32) -> Result<u64, Fault> {
        Ok(u64::from_be_bytes(self.read_n(addr)?))
    }
    #[inline(always)]
    pub fn write_u8(&mut self, addr: u32, v: u8) -> Result<(), Fault> {
        self.write_n(addr, [v])
    }
    #[inline(always)]
    pub fn write_u16(&mut self, addr: u32, v: u16) -> Result<(), Fault> {
        self.write_n(addr, v.to_be_bytes())
    }
    #[inline(always)]
    pub fn write_u32(&mut self, addr: u32, v: u32) -> Result<(), Fault> {
        self.write_n(addr, v.to_be_bytes())
    }
    #[inline(always)]
    pub fn write_u64(&mut self, addr: u32, v: u64) -> Result<(), Fault> {
        self.write_n(addr, v.to_be_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Raw accesses (the flat views) and checked ones (the contiguous view)
    /// see the same bytes at every mirror.
    #[test]
    fn flat_views_alias_ram() {
        let mut m = Mem::new(Box::new(NullMmio));
        #[cfg(windows)]
        assert!(m.is_flat(), "the flat mapping should work on this system");
        m.write_u32(0x8000_1230, 0x1122_3344).unwrap();
        for a in [0x0000_1230, 0x8000_1230, 0xC000_1230] {
            assert_eq!(m.raw_read_u32(a), Ok(0x1122_3344), "{a:#x}");
        }
        m.raw_write_u64(0xD100_0008, 0x0102_0304_0506_0708).unwrap();
        assert_eq!(m.read_u64(0x9100_0008), Ok(0x0102_0304_0506_0708));
        assert_eq!(m.ram(0x1100_0008, 2), Some(&[1u8, 2][..]));
        m.raw_write_u16(0xE000_0010, 0xBEEF).unwrap();
        assert_eq!(m.read_u16(0xE000_0010), Ok(0xBEEF));
        // Two machines do not share memory.
        let mut other = Mem::new(Box::new(NullMmio));
        assert_eq!(other.raw_read_u32(0x8000_1230), Ok(0));
    }

    #[test]
    fn mirrors_share_backing() {
        let mut m = Mem::new(Box::new(NullMmio));
        m.write_u32(0x8000_1000, 0x1122_3344).unwrap();
        assert_eq!(m.read_u32(0xC000_1000), Ok(0x1122_3344));
        assert_eq!(m.read_u32(0x0000_1000), Ok(0x1122_3344));
        assert_eq!(m.read_u16(0x8000_1002), Ok(0x3344));
        m.write_u64(0x9000_0008, 0x0102_0304_0506_0708).unwrap();
        assert_eq!(m.read_u8(0xD000_000F), Ok(8));
        assert_eq!(m.read_u32(0x1000_0008), Ok(0x0102_0304));
    }

    #[test]
    fn bounds_and_unmapped_fault() {
        let mut m = Mem::new(Box::new(NullMmio));
        assert!(m.read_u32(0x817F_FFFC).is_ok());
        assert_eq!(m.read_u32(0x817F_FFFE), Err(Fault { addr: 0x817F_FFFE, write: false }));
        assert!(m.read_u32(0x8180_0000).is_err());
        assert!(m.write_u8(0xA000_0000, 1).is_err());
        assert!(m.read_u64(0xCC00_0000).is_err(), "64-bit MMIO is not a thing");
    }

    #[test]
    fn mmio_routes_by_size() {
        struct Echo(Vec<(u32, u32, u32)>);
        impl Mmio for Echo {
            fn read(&mut self, addr: u32, size: u32) -> u32 {
                addr.wrapping_add(size)
            }
            fn write(&mut self, addr: u32, size: u32, value: u32) {
                self.0.push((addr, size, value));
            }
        }
        let mut m = Mem::new(Box::new(Echo(Vec::new())));
        assert_eq!(m.read_u16(0xCC00_2000), Ok(0x2002));
        assert_eq!(m.read_u32(0x0D00_0000), Ok(0x0D00_0004));
        m.write_u16(0xCD00_0004, 0xBEEF).unwrap();
        assert!(m.ram(0xCC00_0000, 4).is_none());
    }
}

/// RAM's bytes. Loading counts as a write to every code page, so decoded
/// blocks are decoded again.
impl snap::Snapshot for Mem {
    fn snap(&mut self, s: &mut snap::Snap) {
        s.tag(b"RAM ");
        s.bytes(self.ram);
        if s.loading() {
            for g in &mut self.code_gen {
                *g = g.wrapping_add(1);
            }
            self.code_epoch = self.code_epoch.wrapping_add(1);
        }
    }
}
