//! The processor check that runs before anything else in `nsmbw-port.exe`.
//!
//! Windows releases ship two builds of the game: `nsmbw-port.exe` for
//! x86-64-v3 (AVX2, FMA, BMI2: Intel Haswell, AMD Excavator and newer) and
//! `compat\nsmbw-port.exe` for x86-64-v2 (SSE4.2, POPCNT: Intel Nehalem, AMD
//! Bulldozer and newer). The launcher picks the right one; this is for the
//! game started some other way.
//!
//! Code compiled for x86-64-v3 may use AVX2 or BMI2 anywhere, even in `main`'s
//! first lines, so the check cannot be Rust: it is written in assembly below,
//! which the compiler does not touch, and it is put in the C runtime's table
//! of initialisers (`.CRT$XCB`), which runs before `main` and before every
//! initialiser of the Rust standard library (`.CRT$XCT`) or of other crates
//! (`.CRT$XCU`). Only the C runtime's start-up code, built for any x86-64,
//! runs before it.
//!
//! - In the x86-64-v3 build: a processor without one of v3's features, or the
//!   environment variable `NSMBW_FORCE_COMPAT` set to anything, starts
//!   `compat\nsmbw-port.exe` next to this program with the same command line,
//!   working folder and standard handles, waits for it and leaves with its
//!   exit code. If it cannot be started, a message says why.
//! - In the compat build: a processor without one of v2's features gets a
//!   message instead of a crash.
//!
//! The messages go to standard error and to a message box.

use std::arch::global_asm;

/// x86-64-v3 (this is the normal build).
#[cfg(target_feature = "avx2")]
mod level {
    /// CPUID leaf 1, ECX: SSE3, SSSE3, FMA, CX16, SSE4.1, SSE4.2, MOVBE,
    /// POPCNT, XSAVE, OSXSAVE, AVX, F16C.
    pub const LEAF1_ECX: u32 = 1 << 0 | 1 << 9 | 1 << 12 | 1 << 13 | 1 << 19 | 1 << 20 | 1 << 22 | 1 << 23 | 1 << 26 | 1 << 27 | 1 << 28 | 1 << 29;
    /// CPUID leaf 7, EBX: BMI1, AVX2, BMI2.
    pub const LEAF7_EBX: u32 = 1 << 3 | 1 << 5 | 1 << 8;
    /// CPUID leaf 0x80000001, ECX: LAHF/SAHF, LZCNT.
    pub const EXT1_ECX: u32 = 1 << 0 | 1 << 5;
    /// XCR0: the system saves SSE and AVX registers.
    pub const XCR0: u32 = 1 << 1 | 1 << 2;
    /// Look at `NSMBW_FORCE_COMPAT`.
    pub const FORCE: u32 = 1;
}

/// x86-64-v2 (the compat build).
#[cfg(not(target_feature = "avx2"))]
mod level {
    /// CPUID leaf 1, ECX: SSE3, SSSE3, CX16, SSE4.1, SSE4.2, POPCNT.
    pub const LEAF1_ECX: u32 = 1 << 0 | 1 << 9 | 1 << 13 | 1 << 19 | 1 << 20 | 1 << 23;
    pub const LEAF7_EBX: u32 = 0;
    /// CPUID leaf 0x80000001, ECX: LAHF/SAHF.
    pub const EXT1_ECX: u32 = 1 << 0;
    pub const XCR0: u32 = 0;
    pub const FORCE: u32 = 0;
}

/// ASCII, NUL-terminated, as UTF-16 (for the Windows calls ending in W).
const fn wide<const N: usize>(s: &str) -> [u16; N] {
    let b = s.as_bytes();
    let mut out = [0u16; N];
    let mut i = 0;
    while i < b.len() {
        out[i] = b[i] as u16;
        i += 1;
    }
    out
}

const TAIL: &str = "compat\\nsmbw-port.exe";
/// Put after the last `\` of this program's path.
static COMPAT_TAIL: [u16; TAIL.len() + 1] = wide(TAIL);
const FORCE_NAME: &str = "NSMBW_FORCE_COMPAT";
static FORCE_ENV: [u16; FORCE_NAME.len() + 1] = wide(FORCE_NAME);

/// A string's bytes and a NUL.
const fn bytes<const N: usize>(s: &str) -> [u8; N] {
    let b = s.as_bytes();
    let mut out = [0u8; N];
    let mut i = 0;
    while i < b.len() {
        out[i] = b[i];
        i += 1;
    }
    out
}

/// A message: its text and a NUL (for the message box), and its length
/// without the NUL (for standard error).
macro_rules! message {
    ($name:ident, $len:ident, $text:expr) => {
        const $len: usize = concat!($text, "\r\n").len();
        static $name: [u8; $len + 1] = bytes(concat!($text, "\r\n"));
    };
}
message!(
    NO_COMPAT,
    NO_COMPAT_LEN,
    "nsmbw-port.exe needs a processor with AVX2, FMA and BMI2 (Intel Haswell, AMD Excavator or newer). \
     This one lacks them (or NSMBW_FORCE_COMPAT is set), and compat\\nsmbw-port.exe, the build for older \
     processors, could not be started. Keep the compat folder next to nsmbw-port.exe, or start the game \
     from nsmbw-launcher.exe."
);
message!(
    TOO_OLD,
    TOO_OLD_LEN,
    "This processor is too old for the NSMBW port: it needs SSE4.2 and POPCNT (Intel Nehalem, AMD \
     Bulldozer or newer)."
);
static TITLE: [u8; 11] = *b"NSMBW Port\0";

/// GetModuleFileNameW's buffer, in UTF-16 units.
const PATH_CAP: usize = 32768;
static mut PATH: [u16; PATH_CAP] = [0; PATH_CAP];
/// STARTUPINFOW (104 bytes) and PROCESS_INFORMATION (24 bytes).
static mut STARTUP_INFO: [u64; 13] = [0; 13];
static mut PROCESS_INFO: [u64; 3] = [0; 3];
/// The compat build's exit code; WriteFile's count.
static mut WORD: u32 = 0;

#[link(name = "kernel32")]
extern "system" {}
#[link(name = "user32")]
extern "system" {}

// Microsoft x64 calling convention: rcx, rdx, r8, r9, then the stack above
// 32 bytes of shadow space; rsp is 16-aligned at each call. None of these
// routines returns to Rust code except `nsmbw_cpu_guard` on a good processor.
global_asm!(
    // The C runtime calls every pointer in .CRT$XCA..XCZ in order before main.
    ".pushsection .CRT$XCB,\"dr\"",
    ".p2align 3",
    ".quad nsmbw_cpu_guard",
    ".popsection",
    ".pushsection .text,\"xr\"",
    ".p2align 4",
    "nsmbw_cpu_guard:",
    "    push rbx",
    "    sub rsp, 32",
    "    xor eax, eax",
    "    cpuid",
    "    cmp eax, 7",
    "    jb 2f",
    "    mov eax, 1",
    "    xor ecx, ecx",
    "    cpuid",
    "    and ecx, {leaf1_ecx}",
    "    cmp ecx, {leaf1_ecx}",
    "    jne 2f",
    "    mov eax, 7",
    "    xor ecx, ecx",
    "    cpuid",
    "    and ebx, {leaf7_ebx}",
    "    cmp ebx, {leaf7_ebx}",
    "    jne 2f",
    "    mov eax, 0x80000000",
    "    cpuid",
    "    cmp eax, 0x80000001",
    "    jb 2f",
    "    mov eax, 0x80000001",
    "    cpuid",
    "    and ecx, {ext1_ecx}",
    "    cmp ecx, {ext1_ecx}",
    "    jne 2f",
    // XGETBV (only once CPUID has said OSXSAVE, which v3 asks for).
    "    mov eax, {xcr0}",
    "    test eax, eax",
    "    jz 3f",
    "    xor ecx, ecx",
    "    .byte 0x0f, 0x01, 0xd0",
    "    and eax, {xcr0}",
    "    cmp eax, {xcr0}",
    "    jne 2f",
    "3:",
    "    mov eax, {force}",
    "    test eax, eax",
    "    jz 4f",
    // GetEnvironmentVariableW(name, NULL, 0) is not 0 when it is set.
    "    lea rcx, [rip + {force_env}]",
    "    xor edx, edx",
    "    xor r8d, r8d",
    "    call qword ptr [rip + __imp_GetEnvironmentVariableW]",
    "    test eax, eax",
    "    jnz 2f",
    "4:",
    "    add rsp, 32",
    "    pop rbx",
    "    ret",
    "2:",
    "    mov eax, {force}",
    "    test eax, eax",
    "    jz 5f",
    "    call nsmbw_compat_handoff",
    "5:",
    "    lea rcx, [rip + {too_old}]",
    "    mov edx, {too_old_len}",
    "    call nsmbw_cpu_fail",
    "    int3",
    //
    // Starts compat\nsmbw-port.exe and leaves with its exit code.
    ".p2align 4",
    "nsmbw_compat_handoff:",
    "    sub rsp, 88",
    "    xor ecx, ecx",
    "    lea rdx, [rip + {path}]",
    "    mov r8d, {path_cap}",
    "    call qword ptr [rip + __imp_GetModuleFileNameW]",
    "    test eax, eax",
    "    jz 8f",
    "    cmp eax, {path_cap}",
    "    jae 8f",
    // The last backslash.
    "    lea rdx, [rip + {path}]",
    "    mov ecx, eax",
    "6:",
    "    test ecx, ecx",
    "    jz 8f",
    "    dec ecx",
    "    cmp word ptr [rdx + rcx*2], 0x5c",
    "    jne 6b",
    "    lea eax, [rcx + 1 + {tail_len}]",
    "    cmp eax, {path_cap}",
    "    ja 8f",
    "    lea r8, [rdx + rcx*2 + 2]",
    "    lea r9, [rip + {tail}]",
    "7:",
    "    movzx eax, word ptr [r9]",
    "    mov word ptr [r8], ax",
    "    add r8, 2",
    "    add r9, 2",
    "    test eax, eax",
    "    jnz 7b",
    "    lea rcx, [rip + {startup_info}]",
    "    call qword ptr [rip + __imp_GetStartupInfoW]",
    "    call qword ptr [rip + __imp_GetCommandLineW]",
    // CreateProcessW(path, command line, NULL, NULL, inherit handles,
    // 0, NULL environment and folder (this one's), &si, &pi).
    "    lea rcx, [rip + {path}]",
    "    mov rdx, rax",
    "    xor r8d, r8d",
    "    xor r9d, r9d",
    "    mov qword ptr [rsp + 32], 1",
    "    mov qword ptr [rsp + 40], 0",
    "    mov qword ptr [rsp + 48], 0",
    "    mov qword ptr [rsp + 56], 0",
    "    lea rax, [rip + {startup_info}]",
    "    mov qword ptr [rsp + 64], rax",
    "    lea rax, [rip + {process_info}]",
    "    mov qword ptr [rsp + 72], rax",
    "    call qword ptr [rip + __imp_CreateProcessW]",
    "    test eax, eax",
    "    jz 8f",
    "    mov rcx, qword ptr [rip + {process_info}]",
    "    mov edx, 0xffffffff",
    "    call qword ptr [rip + __imp_WaitForSingleObject]",
    "    mov rcx, qword ptr [rip + {process_info}]",
    "    lea rdx, [rip + {word}]",
    "    call qword ptr [rip + __imp_GetExitCodeProcess]",
    "    mov ecx, dword ptr [rip + {word}]",
    "    call qword ptr [rip + __imp_ExitProcess]",
    "8:",
    "    lea rcx, [rip + {no_compat}]",
    "    mov edx, {no_compat_len}",
    "    call nsmbw_cpu_fail",
    "    int3",
    //
    // Says rcx's message (rdx bytes) on standard error and in a message box,
    // and leaves with exit code 3.
    ".p2align 4",
    "nsmbw_cpu_fail:",
    "    push rbx",
    "    sub rsp, 48",
    "    mov rbx, rcx",
    "    mov qword ptr [rsp + 40], rdx",
    "    mov ecx, -12",
    "    call qword ptr [rip + __imp_GetStdHandle]",
    // WriteFile(handle, text, length, &count, NULL); a bad handle just fails.
    "    mov rcx, rax",
    "    mov rdx, rbx",
    "    mov r8, qword ptr [rsp + 40]",
    "    lea r9, [rip + {word}]",
    "    mov qword ptr [rsp + 32], 0",
    "    call qword ptr [rip + __imp_WriteFile]",
    // MessageBoxA(NULL, text, title, MB_ICONERROR).
    "    xor ecx, ecx",
    "    mov rdx, rbx",
    "    lea r8, [rip + {title}]",
    "    mov r9d, 0x10",
    "    call qword ptr [rip + __imp_MessageBoxA]",
    "    mov ecx, 3",
    "    call qword ptr [rip + __imp_ExitProcess]",
    "    int3",
    ".popsection",
    leaf1_ecx = const level::LEAF1_ECX,
    leaf7_ebx = const level::LEAF7_EBX,
    ext1_ecx = const level::EXT1_ECX,
    xcr0 = const level::XCR0,
    force = const level::FORCE,
    force_env = sym FORCE_ENV,
    path = sym PATH,
    path_cap = const PATH_CAP,
    tail = sym COMPAT_TAIL,
    tail_len = const TAIL.len() + 1,
    startup_info = sym STARTUP_INFO,
    process_info = sym PROCESS_INFO,
    word = sym WORD,
    no_compat = sym NO_COMPAT,
    no_compat_len = const NO_COMPAT_LEN,
    too_old = sym TOO_OLD,
    too_old_len = const TOO_OLD_LEN,
    title = sym TITLE,
);
