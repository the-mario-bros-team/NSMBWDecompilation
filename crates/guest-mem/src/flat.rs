//! RAM as a pagefile-backed section, mapped twice over:
//! - once contiguously (MEM1, MEM2, the locked cache: [`Mem`](crate::Mem)'s
//!   `ram`, DMA, save states, the GX thread's [`RamView`](crate::RamView));
//! - at every guest address it appears at (MEM1 at `0x00`, `0x80` and
//!   `0xC0000000`; MEM2 at `0x10`, `0x90` and `0xD0000000`; the locked cache
//!   at `0xE0000000`), inside a 4 GiB reservation of host address space.
//!
//! With the second, a guest address is a host pointer, `base + address`,
//! with no lookup. Recompiled code uses it for accesses it knows are RAM
//! (stack and small-data relative); an address with nothing mapped faults
//! cleanly (an access violation) instead of reaching other host memory,
//! because the reservation covers every 32-bit address.
//!
//! If the system cannot do this (it needs Windows 10 1803 or newer), RAM is
//! a plain allocation and every access takes the checked path.

use crate::{LOCKED_CACHE_SIZE, MEM1_SIZE, MEM2_SIZE};

/// Placeholders split at the allocation granularity, so the locked cache's
/// view is padded to it.
const GRANULE: usize = 64 << 10;
/// Section layout: MEM1, MEM2, then the locked cache padded to [`GRANULE`].
const LOCKED_OFFSET: usize = MEM1_SIZE + MEM2_SIZE;
pub(crate) const SECTION_SIZE: usize = LOCKED_OFFSET + GRANULE;
const _: () = assert!(LOCKED_CACHE_SIZE <= GRANULE && MEM1_SIZE % GRANULE == 0 && MEM2_SIZE % GRANULE == 0);

/// (guest address, section offset, size) of every flat view.
const VIEWS: [(u32, usize, usize); 7] = [
    (0x0000_0000, 0, MEM1_SIZE),
    (0x8000_0000, 0, MEM1_SIZE),
    (0xC000_0000, 0, MEM1_SIZE),
    (0x1000_0000, MEM1_SIZE, MEM2_SIZE),
    (0x9000_0000, MEM1_SIZE, MEM2_SIZE),
    (0xD000_0000, MEM1_SIZE, MEM2_SIZE),
    (0xE000_0000, LOCKED_OFFSET, GRANULE),
];

pub(crate) struct Backing {
    /// The contiguous view (`SECTION_SIZE` bytes).
    pub ptr: *mut u8,
    /// The 4 GiB reservation with the guest views, or null.
    pub flat: *mut u8,
    #[cfg(windows)]
    os: Option<win::Os>,
}

// SAFETY: plain memory mappings; `RamBuf` documents how they are shared.
unsafe impl Send for Backing {}
unsafe impl Sync for Backing {}

impl Backing {
    pub fn new() -> Self {
        #[cfg(windows)]
        if let Some((ptr, flat, os)) = win::map() {
            return Self { ptr, flat, os: Some(os) };
        }
        let ptr = Box::into_raw(vec![0u8; SECTION_SIZE].into_boxed_slice()).cast::<u8>();
        Self {
            ptr,
            flat: std::ptr::null_mut(),
            #[cfg(windows)]
            os: None,
        }
    }
}

impl Drop for Backing {
    fn drop(&mut self) {
        #[cfg(windows)]
        if let Some(os) = self.os.take() {
            os.unmap(self.ptr, self.flat);
            return;
        }
        // SAFETY: `ptr` came from `Box::into_raw` of `SECTION_SIZE` bytes.
        unsafe { drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(self.ptr, SECTION_SIZE))) };
    }
}

#[cfg(windows)]
mod win {
    use super::{SECTION_SIZE, VIEWS};
    use std::ffi::c_void;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::System::Memory::{
        CreateFileMappingW, MapViewOfFile, MapViewOfFile3, UnmapViewOfFile, VirtualAlloc2, VirtualFree, FILE_MAP_ALL_ACCESS, MEMORY_MAPPED_VIEW_ADDRESS, MEM_RELEASE, MEM_REPLACE_PLACEHOLDER, MEM_RESERVE, MEM_RESERVE_PLACEHOLDER, PAGE_NOACCESS, PAGE_READWRITE,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    const SPAN: usize = 1 << 32;
    /// `VirtualFree`'s `MEM_PRESERVE_PLACEHOLDER` (split, keep reserved).
    const PRESERVE_PLACEHOLDER: u32 = 2;

    pub struct Os {
        section: HANDLE,
        /// Address ranges in the reservation that are still placeholders.
        holes: Vec<(usize, usize)>,
    }

    /// The section, its contiguous view, and the reservation with every
    /// guest view, or `None` if any step fails (everything made is undone).
    pub fn map() -> Option<(*mut u8, *mut u8, Os)> {
        // SAFETY: plain Win32 calls; every handle and view made is released
        // on failure here or by `Os::unmap`.
        unsafe {
            let section = CreateFileMappingW(INVALID_HANDLE_VALUE, std::ptr::null(), PAGE_READWRITE, 0, SECTION_SIZE as u32, std::ptr::null());
            if section.is_null() {
                return None;
            }
            let view = MapViewOfFile(section, FILE_MAP_ALL_ACCESS, 0, 0, SECTION_SIZE).Value.cast::<u8>();
            if view.is_null() {
                CloseHandle(section);
                return None;
            }
            let process = GetCurrentProcess();
            let base = VirtualAlloc2(process, std::ptr::null(), SPAN, MEM_RESERVE | MEM_RESERVE_PLACEHOLDER, PAGE_NOACCESS, std::ptr::null_mut(), 0).cast::<u8>();
            if base.is_null() {
                UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS { Value: view.cast() });
                CloseHandle(section);
                return None;
            }
            let mut os = Os { section, holes: vec![(0, SPAN)] };
            let mut views = VIEWS;
            views.sort_by_key(|v| v.0);
            for &(addr, offset, size) in &views {
                let (addr, end) = (addr as usize, addr as usize + size);
                // Split the placeholder holding [addr, end) off its hole.
                let i = os.holes.iter().position(|&(a, e)| a <= addr && end <= e)?;
                let (a, e) = os.holes.remove(i);
                let split = |from: usize, len: usize| VirtualFree(base.add(from).cast(), len, MEM_RELEASE | PRESERVE_PLACEHOLDER) != 0;
                if (a < addr && !split(a, addr - a)) || (end < e && !split(end, e - end)) {
                    os.holes.insert(i, (a, e));
                    os.unmap(view, base);
                    return None;
                }
                if a < addr {
                    os.holes.push((a, addr));
                }
                if end < e {
                    os.holes.push((end, e));
                }
                let v = MapViewOfFile3(section, process, base.add(addr).cast(), offset as u64, size, MEM_REPLACE_PLACEHOLDER, PAGE_READWRITE, std::ptr::null_mut(), 0);
                if v.Value.is_null() {
                    os.holes.push((addr, end));
                    os.unmap(view, base);
                    return None;
                }
            }
            Some((view, base, os))
        }
    }

    impl Os {
        pub fn unmap(self, view: *mut u8, base: *mut u8) {
            // SAFETY: undoes `map`: the guest views (those not in `holes`),
            // the placeholders left, the contiguous view, the section.
            unsafe {
                for &(addr, _, _) in &VIEWS {
                    let a = addr as usize;
                    if !self.holes.iter().any(|&(s, e)| s <= a && a < e) {
                        UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS { Value: base.add(a).cast::<c_void>() });
                    }
                }
                for &(a, _) in &self.holes {
                    VirtualFree(base.add(a).cast(), 0, MEM_RELEASE);
                }
                UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS { Value: view.cast() });
                CloseHandle(self.section);
            }
        }
    }
}
