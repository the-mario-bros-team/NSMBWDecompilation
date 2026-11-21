//! Player 1 picks a character too, and players 2-4 can pick Mario: mkwcat's
//! Gecko code "Player 1 Can Change Character"
//! (<https://github.com/mkwcat/gecko-codes>, `nsmbw/`, USA Rev 1; MIT, its
//! notice below), with the run-time part of WitherBlitz's
//! NSMB-Wii-Recompilation (`runtime/src/hle/gameplay_options.cpp`).
//!
//! The port applies the code as a Gecko code handler would, each write once
//! the code or data it changes is in memory: main.dol's at boot, a REL
//! module's once `OSLink` has placed it ([`System::enable_p1_character`]).
//! Its inserted code (C2) runs from [`STUBS`] in low memory, which the game
//! leaves unused (0x80001800..0x80003000; the bytes from 0x80001800 are
//! [`System::call_guest`]'s data): the site branches there and the insert
//! branches back. The interpreter runs the inserts, which keep data in
//! their code; recompiled code is generated from a RAM image with these
//! writes (`ppc-recomp --p1-character`), each insert's return a place it
//! can be entered, so a run without them interprets the functions they
//! change.
//!
//! Unlike the Gecko code, one insert tests `cr0` where the code's
//! `beq cr7` read whatever cr7 held: it follows the `cmpwi r0, 0` it
//! belongs to.
//!
//! Mario's card: the character screen's cards have pictures of Luigi and
//! the Toads, and none of Mario, who could never be picked there. The code
//! shows a card's Luigi picture for him; [`PORTRAIT`], an insert of the
//! port's own at the card's choice of picture, puts Mario's portrait from
//! the disc's Coin Battle results (`pointResult.arc`, 128x128) in its place,
//! centred on a picture of the cards' size, and Luigi's back for anyone else.
//!
//! The code's licence:
//!
//! MIT License
//!
//! Copyright (c) 2024 mkwcat
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use crate::System;

/// mkwcat's code for USA Rev 1, as published.
const CODE: &str = "
049027F8 3880FFFF 049028F8 60000000 049029A8 60000000 C2902B7C 0000000B
7FE3FB78 3D808090 618C2BD0 7D8903A6 4E800421 3D208035 81294E60 815F0184
808A0004 548416BA 3C848094 8084CDF0 7C092000 40820014 808A01CC 80840004
909F01D0 48000010 814A0184 2C0A0000 4082FFD0 60000000 04902BE8 3BC00000
C2902D58 00000002 30B7FFFF 7CA5B910 60000000 00000000 C2902D6C 00000002
30B7FFFF 7CA5B910 60000000 00000000 C2902E14 00000005 90640294 80840184
90640294 813F0384 3D408035 814A4E60 554A103A 7D26512E 60000000 00000000
C2909410 00000005 80060000 5400103A 81050384 2C080003 41820008 7C882378
7D03012E 38E50008 60000000 00000000 048EB7D8 3CA08094 048EB7E4 38A5CDF0
04902E78 60000000 04902E84 9BBF0124 042F5FC8 00000347 042F60D0 0000036E
042F60D8 0000036E 042F61EC 000003B5 042F61F4 000003B5 042F6308 000003FB
042F6310 000003FB 047A1C8C 60000000 C27A1C94 00000007 4800000D 03230372
03B903FF 7D4802A6 3D208035 81294E60 5529083C 7C8A4A2E 40820010 2C090000
40820008 38800324 60000000 00000000 C219A974 00000002 7C0802A6 90810008
60000000 00000000 C219AA4C 0000000A D02300BC 801D00AC 2C000000 419E003C
80010008 2C000005 4182000C 2C000007 40820028 48000009 40000000 7C8802A6
C0240000 38800000 3D808026 618C7420 7D8903A6 4E800421 60000000 00000000
047B0770 60000000 C27B0778 00000004 4800000D 0356039D 03E4042A 7D4802A6
81260000 5529083C 7C8A4A2E 60000000 C2902ED4 00000002 7C0802A6 90610008
60000000 00000000 C2902F20 00000004 3D208035 81294E60 7C09E000 4082000C
81210008 906901D0 81830000 60000000 028EE0D6 00000000 028EE0E2 00000000
02902D1E 00000000 02902D22 00000000 02902D26 00000000 02902D2E 00000000
C6902FC8 80903020 0290302E 00000000 02903032 00000000 0290303A 00000000
027A008A 00000000 027A008E 00000000 027A0192 00000000 027A018E 00000004
027A11F2 00000000 027A1206 00000000 027A14DA 00000000 027A14DE 00000000
027A151A 00000000 027A152E 00000000 027A1536 00000000 027A162A 00000000
027A162E 00000000 027A1636 00000000 027A165A 00000000 027A165E 00000000
027A15CA 00000000 027A15CE 00000000 027A16DA 00000000 027A16DE 00000000
027A171A 00000000 027A172E 00000000 027A1736 00000000 027A17AE 00000000
027A17B2 00000000 047A068C 60000000 047A0694 60000000 047A069C 60000000
04918B28 38000000 040BB164 90E30384 047A0FE8 60000000 C27A0D24 00000002
7C0802A6 80830074 8884029A 60000000 C27A0D40 00000005 2C040000 40820018
38600000 80010014 7C0803A6 38210010 4E800020 38800900 60000000 00000000
C27A0D68 00000004 38600000 2C040000 41820014 80010014 7C0803A6 38210010
4E800020 60000000 047A12A8 60000000 C28C5DFC 00000004 3D208035 81294E60
81430004 554A073F 7D4A4A78 7D4A0034 554AD97F 60000000 04770A8C 2C030004
047AC8CC 38000001 04918B50 60000000 0476E630 2C000004 0476EAB8 2C030004
0476DDE8 60000000 04770988 5400173A 047709A8 60000000 047709B0 540407FE
0476FE4C 5400173A C276FF0C 00000003 2C000000 40820008 38000004 38800000
60000000 00000000 C276FE84 00000003 2C000000 40820008 38000004 900302E0
60000000 00000000 C277117C 00000003 388003F2 801F02D4 2C000000 40820008
3880031D 60000000 C2771164 00000003 388003F3 801F02D4 2C000000 40820008
3880031E 60000000 C276FDC4 00000003 2C1F0007 40820008 38800000 7FA3EB78
60000000 00000000 C276FDD4 00000003 2C1F0007 40820008 3BE00008 2C1F0000
60000000 00000000 0476FD8C 7FE628AE 06933CF0 00000005 06000102 07000000
C277359C 0000000C 2C040007 41820024 2C040008 40820048 54A0103A 7CA30214
80A50254 880500BB 60000001 980500BB 386302A4 57E0103A 3C808094 3884F520
7CBE0214 80A50274 38C00000 3D80800B 618C3B60 7D8903A6 4E800421 2C040004
60000000 00000000
";

/// Where the inserts go, one after another; the first is [`PORTRAIT`].
pub const STUBS: u32 = 0x8000_1C00;
const STUBS_END: u32 = 0x8000_3000;
/// The port's insert where a card picks the picture of the character it
/// shows (r3 the card, r4 what it shows: 7 and 8 are Mario, free and taken;
/// r31 which of its two sides), before the instruction it displaced: a
/// hook ([`crate::SPECIAL`]) that never replaces it.
pub const PORTRAIT: u32 = STUBS;
const PORTRAIT_SITE: u32 = 0x8077_3574;
const CMPWI_R4_2: u32 = 0x2C04_0002;
/// The insert whose `beq cr7` becomes `beq cr0` (see the module's notes).
const CR7_SITE: u32 = 0x8019_AA4C;
const BEQ_CR7: u32 = 0x419E_003C;
const BEQ_CR0: u32 = 0x4182_003C;
/// main.dol's code and data end below this; everything above is a REL's.
const DOL_END: u32 = 0x8043_0000;

/// Words where the code writes, as USA revision 1 has them, checked before
/// each part (main.dol, a REL module) is changed: a part whose words differ
/// (another revision, a mod) is left as it is.
const EXPECTED: [(u32, u32); 11] = [
    (0x800B_B164, 0x9083_0384),
    (0x8019_A974, 0x7C08_02A6),
    (0x8019_AA4C, 0x480C_CA65),
    (0x802F_5FC8, 0x0000_032B),
    (0x8077_3574, CMPWI_R4_2),
    (0x8077_359C, 0x2C04_0004),
    (0x807A_1C94, 0x3880_0324),
    (0x808C_5DFC, 0x2C00_0000),
    (0x8090_2B7C, 0x4B7D_4275),
    (0x8091_8B28, 0x3800_0003),
    (0x8093_3CF0, 0x0000_0006),
];

/// Mario's portrait, built at boot: in MEM2 above the game's (as
/// [`System::call_guest`]'s stack), where nothing else goes.
pub const PORTRAIT_IMAGE: u32 = 0x9342_0000;
const PORTRAIT_FILE: &str = "/Layout/pointResult/pointResult.arc";
const PORTRAIT_TPL: &str = "im_mario_00.tpl";
/// The cards' pictures: 150 pixels square, RGB5A3 (2 bytes a pixel, in
/// tiles of 4x4); Mario's is 128 square, placed at (11, 14).
const SIDE: usize = 150;
const TILES: usize = SIDE.div_ceil(4);
const MARIO: usize = 128;
const MARIO_AT: (usize, usize) = (11, 14);
const RGB5A3: u32 = 5;
/// A card: its eight pictures (`nw4r::lyt::Pane`s) from +0x254.
const CARD_PICTURES: u32 = 0x254;
const PANE_MATERIAL: u32 = 0x28;
const PANE_FLAGS: u32 = 0xBB;
/// `nw4r::lyt::Material`'s texture maps; a `TexMap` is its image, palette,
/// then width and height.
const MATERIAL_TEX_MAPS: u32 = 0x40;
const MARIO_FREE: u32 = 7;
const MARIO_TAKEN: u32 = 8;

/// The character screen (profile 0x2B6): its title
/// (`T_titleNinzu_00`, "<count> Players", the count `dInfo_c`'s) and the
/// row of remotes under the cards ("Add Wii Remote").
const SCREEN_TITLE: u32 = 0x608;
const TITLE_PANE: &str = "T_titleNinzu_00";
const REMOTES_PANE: &str = "N_textBase_00";
const INFO: u32 = 0x8042_9F7C; // dInfo_c::m_instance
const INFO_PLAYERS: u32 = 0x3CC;
/// `nw4r::lyt::TextBox`: its text (UTF-16) and its length in characters.
const TEXT_BUF: u32 = 0xD8;
const TEXT_LEN: u32 = 0xFE;
/// "Players" as the game's languages write it, and the singular.
const PLURALS: [(&str, &str); 4] = [("Players", "Player"), ("joueurs", "joueur"), ("Jugadores", "Jugador"), ("jugadores", "jugador")];

/// One write: bytes at an address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Write {
    pub addr: u32,
    pub bytes: Vec<u8>,
}

fn branch(from: u32, to: u32) -> u32 {
    0x4800_0000 | (to.wrapping_sub(from) & 0x03FF_FFFC)
}

fn word_write(addr: u32, w: u32) -> Write {
    Write { addr, bytes: w.to_be_bytes().to_vec() }
}

/// Every write the code makes, its inserts laid out from [`STUBS`] in the
/// order they come, and where each insert returns (an address recompiled
/// code must be enterable at).
pub fn writes() -> (Vec<Write>, Vec<u32>) {
    let words: Vec<u32> = CODE.split_whitespace().map(|w| u32::from_str_radix(w, 16).expect("hex")).collect();
    let (mut out, mut returns, mut stub) = (Vec::new(), Vec::new(), STUBS);
    let mut insert = |out: &mut Vec<Write>, site: u32, body: &[u32]| {
        out.push(word_write(site, branch(site, stub)));
        for &w in body {
            out.push(word_write(stub, w));
            stub += 4;
        }
        out.push(word_write(stub, branch(stub, site + 4)));
        stub += 4;
        returns.push(site + 4);
    };
    insert(&mut out, PORTRAIT_SITE, &[CMPWI_R4_2]);
    let mut i = 0;
    while i + 1 < words.len() {
        let (head, value) = (words[i], words[i + 1]);
        i += 2;
        let addr = 0x8000_0000 | head & 0x01FF_FFFF;
        match head >> 24 & 0xFE {
            0x02 => out.push(Write { addr, bytes: (value as u16).to_be_bytes().to_vec() }),
            0x04 => out.push(word_write(addr, value)),
            0x06 => {
                let n = value as usize;
                let lines = n.div_ceil(8) * 2;
                let bytes = words[i..i + lines].iter().flat_map(|w| w.to_be_bytes()).take(n).collect();
                i += lines;
                out.push(Write { addr, bytes });
            }
            0xC2 => {
                // The last word becomes the branch back.
                let n = value as usize * 2;
                let mut body = words[i..i + n - 1].to_vec();
                i += n;
                if addr == CR7_SITE {
                    body.iter_mut().filter(|w| **w == BEQ_CR7).for_each(|w| *w = BEQ_CR0);
                }
                insert(&mut out, addr, &body);
            }
            0xC6 => out.push(word_write(addr, branch(addr, value))),
            k => unreachable!("Gecko code type {k:02X}"),
        }
    }
    (out, returns)
}

/// The run-time state.
#[derive(Default)]
pub struct P1Character {
    /// The code is applied (from boot, outside scripted, headless and net runs).
    pub on: bool,
    /// Mario's portrait is at [`PORTRAIT_IMAGE`].
    portrait: bool,
    /// Luigi's picture of each card material Mario's replaced.
    luigi: Vec<(u32, u32)>,
    /// The character screen (its object and unique number) and its row of
    /// remotes, once found.
    screen: (u32, u32, u32),
}

impl System {
    /// Applies the code: main.dol's part and the inserts now, every REL
    /// module already placed (a loaded state's), and each one `OSLink`
    /// places from now on; Mario's portrait from the disc. False, and
    /// nothing changed, if main.dol is not the one the code is for.
    pub fn enable_p1_character(&mut self) -> bool {
        if !self.p1_expected(|a| a < DOL_END) {
            return false;
        }
        self.p1.on = true;
        self.p1_write(|a| a < DOL_END || (STUBS..STUBS_END).contains(&a));
        let modules: Vec<Vec<(u32, u32, bool)>> = self.linked.iter().filter(|m| m.ok).map(|m| m.sections.clone()).collect();
        for sections in modules {
            self.p1_linked(&sections);
        }
        self.p1.portrait = self.build_portrait().is_some();
        true
    }

    /// A REL module placed by `OSLink`: the code's writes into it.
    pub(crate) fn p1_linked(&mut self, sections: &[(u32, u32, bool)]) {
        let inside = |a: u32| a >= DOL_END && sections.iter().any(|&(s, n, _)| (s..s.wrapping_add(n)).contains(&a));
        if !self.p1.on || !self.p1_expected(inside) {
            return;
        }
        self.p1_write(inside);
    }

    /// Whether the checked words among `part` hold what USA revision 1 has
    /// there, or what the code writes (a state saved with it applied).
    fn p1_expected(&mut self, part: impl Fn(u32) -> bool) -> bool {
        let (writes, _) = writes();
        for &(addr, original) in EXPECTED.iter().filter(|e| part(e.0)) {
            let now = self.mem.read_u32(addr).unwrap_or(!original);
            // Each checked word gets a whole word or more.
            let patched = writes.iter().find(|w| w.addr == addr && w.bytes.len() >= 4).map(|w| u32::from_be_bytes([w.bytes[0], w.bytes[1], w.bytes[2], w.bytes[3]]));
            if now != original && Some(now) != patched {
                eprintln!("p1 character: {addr:#010x} holds {now:#010x}, not {original:#010x}: this part of the game is left as it is");
                return false;
            }
        }
        true
    }

    fn p1_write(&mut self, part: impl Fn(u32) -> bool) {
        for w in writes().0.into_iter().filter(|w| part(w.addr)) {
            for (i, &b) in w.bytes.iter().enumerate() {
                let _ = self.mem.write_u8(w.addr + i as u32, b);
            }
        }
    }

    /// Mario's 128x128 portrait from the disc, centred on a picture the
    /// cards' size at [`PORTRAIT_IMAGE`]; `None` without the file (a mod's
    /// disc), and Mario's cards keep Luigi's picture.
    fn build_portrait(&mut self) -> Option<()> {
        let arc = self.hw.borrow().ios.disc.file(PORTRAIT_FILE)?;
        let nodes = crate::menu_layout::read_u8(&arc)?;
        let tpl = &nodes.iter().find(|n| n.kind == 0 && n.name == PORTRAIT_TPL)?.data;
        let be = |at: usize| tpl.get(at..at + 4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]));
        let header = be(be(8)? as usize)? as usize;
        let (size, format, data) = (be(header)?, be(header + 4)?, be(header + 8)? as usize);
        let pixels = tpl.get(data..data + MARIO * MARIO * 2)?;
        if size != (MARIO << 16 | MARIO) as u32 || format != RGB5A3 {
            return None;
        }
        // Tiled: tile (y / 4, x / 4) row by row, then its 4x4 pixels.
        let at = |x: usize, y: usize, row_tiles: usize| ((y / 4 * row_tiles + x / 4) * 16 + y % 4 * 4 + x % 4) * 2;
        let mut canvas = vec![0u8; TILES * TILES * 32];
        for y in 0..MARIO {
            for x in 0..MARIO {
                let (from, to) = (at(x, y, MARIO / 4), at(x + MARIO_AT.0, y + MARIO_AT.1, TILES));
                canvas[to..to + 2].copy_from_slice(&pixels[from..from + 2]);
            }
        }
        self.mem.ram_mut(PORTRAIT_IMAGE, canvas.len())?.copy_from_slice(&canvas);
        Some(())
    }

    /// The character screen at `screen`, after a field: the row of remotes
    /// under the cards hidden (the pick is made on the cards), and with one
    /// player the title's "Players" singular: the game only ever opened the
    /// screen for two to four, and the disc has no singular to show.
    pub fn character_screen(&mut self, screen: u32) {
        let id = self.r32(screen);
        // Found once the screen has built its layout.
        if (self.p1.screen.0, self.p1.screen.1) != (screen, id) || self.p1.screen.2 == 0 {
            // Scan the object for its layout's root pane.
            let root = (0..0x800).step_by(4).map(|off| screen + off).find_map(|a| {
                let p = self.r32(a);
                (crate::menu_settings::guest_ram(p) && self.pane_name(p) == "RootPane").then_some(p)
            });
            let panel = root.map_or(0, |r| self.find_pane(r, REMOTES_PANE, 0));
            self.p1.screen = (screen, id, panel);
        }
        let panel = self.p1.screen.2;
        if panel != 0 {
            let flags = self.r8(panel + PANE_FLAGS);
            self.w8(panel + PANE_FLAGS, flags & !1);
        }
        let info = self.r32(INFO);
        let title = self.r32(screen + SCREEN_TITLE);
        if !crate::menu_settings::guest_ram(info) || self.r32(info + INFO_PLAYERS) != 1 || !crate::menu_settings::guest_ram(title) || self.pane_name(title) != TITLE_PANE {
            return;
        }
        let (buf, len) = (self.r32(title + TEXT_BUF), u32::from(self.mem.read_u16(title + TEXT_LEN).unwrap_or(0)));
        if !crate::menu_settings::guest_ram(buf) {
            return;
        }
        let text: Vec<u16> = (0..len.min(64)).map(|i| self.mem.read_u16(buf + 2 * i).unwrap_or(0)).collect();
        for (plural, singular) in PLURALS {
            let plural: Vec<u16> = plural.encode_utf16().collect();
            if !text.ends_with(&plural) {
                continue;
            }
            let start = buf + 2 * (text.len() - plural.len()) as u32;
            let singular: Vec<u16> = singular.encode_utf16().chain([0]).collect();
            for (i, &c) in singular.iter().enumerate() {
                let _ = self.mem.write_u16(start + 2 * i as u32, c);
            }
            let _ = self.mem.write_u16(title + TEXT_LEN, (text.len() - plural.len() + singular.len() - 1) as u16);
            return;
        }
    }

    /// [`PORTRAIT`]: a card shows Mario (7, 8) in its Luigi picture with
    /// Mario's portrait, and anyone else with Luigi's again.
    pub(crate) fn card_portrait(&mut self) {
        if !self.p1.portrait {
            return;
        }
        let (card, shows, side) = (self.cpu.gpr[3], self.cpu.gpr[4], self.cpu.gpr[31]);
        let name = if side & 1 == 0 { "P_lui_00" } else { "P_lui_01" };
        let mut picture = 0;
        for i in 0..8 {
            let p = self.r32(card + CARD_PICTURES + 4 * i);
            if crate::menu_settings::guest_ram(p) && self.pane_name(p) == name {
                picture = p;
                break;
            }
        }
        if picture == 0 {
            return;
        }
        let material = self.r32(picture + PANE_MATERIAL);
        let maps = if crate::menu_settings::guest_ram(material) { self.r32(material + MATERIAL_TEX_MAPS) } else { 0 };
        if !crate::menu_settings::guest_ram(maps) {
            return;
        }
        let image = self.r32(maps);
        if shows == MARIO_FREE || shows == MARIO_TAKEN {
            let fits = self.mem.read_u16(maps + 8) == Ok(SIDE as u16) && self.mem.read_u16(maps + 10) == Ok(SIDE as u16);
            if !fits {
                return;
            }
            if image != PORTRAIT_IMAGE {
                self.p1.luigi.retain(|l| l.0 != material);
                if self.p1.luigi.len() >= 16 {
                    self.p1.luigi.remove(0);
                }
                self.p1.luigi.push((material, image));
                self.w32(maps, PORTRAIT_IMAGE);
            }
            let flags = self.r8(picture + PANE_FLAGS);
            self.w8(picture + PANE_FLAGS, flags | 1);
        } else if image == PORTRAIT_IMAGE {
            if let Some(&(_, luigi)) = self.p1.luigi.iter().find(|l| l.0 == material) {
                self.w32(maps, luigi);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_code_decodes_and_its_inserts_fit() {
        let (w, returns) = writes();
        // 93 entries: 22 inserts (a site and its words each), plus the port's own.
        assert_eq!(returns.len(), 23);
        let stub_end = w.iter().filter(|w| (STUBS..STUBS_END).contains(&w.addr)).map(|w| w.addr + 4).max().unwrap();
        assert!(stub_end <= STUBS_END, "{stub_end:#x}");
        assert_eq!(stub_end - STUBS, 4 * (198 + 2));
        // The portrait's insert first, the cr0 fix in place.
        assert_eq!(w[0], word_write(PORTRAIT_SITE, branch(PORTRAIT_SITE, PORTRAIT)));
        assert_eq!(w[1], word_write(PORTRAIT, CMPWI_R4_2));
        assert!(w.iter().any(|w| w.bytes == BEQ_CR0.to_be_bytes()) && !w.iter().any(|w| w.bytes == BEQ_CR7.to_be_bytes()));
        // The icon table's five bytes, and a halfword write.
        assert!(w.contains(&Write { addr: 0x8093_3CF0, bytes: vec![6, 0, 1, 2, 7] }));
        assert!(w.contains(&Write { addr: 0x807A_018E, bytes: vec![0, 4] }));
        // Every insert branches back to the word after its site.
        assert!(returns.iter().all(|&r| w.iter().any(|x| x.addr >= STUBS && x.bytes == branch(x.addr, r).to_be_bytes())));
    }
}
