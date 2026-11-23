//! The Settings button's layouts (after WitherBlitz's NSMB-Wii-Recompilation,
//! `runtime/src/hle/storage/layout_patch.cpp`): the game's archives, changed
//! in memory as the disc is opened, so the game builds the button itself.
//!
//! - The pause window (`/Layout/pauseMenu/pauseMenu.arc`, `pauseMenu_16`):
//!   Exit's button is copied under new names between Continue and Exit,
//!   the window grows by one button and stays centred, and the copy gets an
//!   animation group (registered under a name the game already holds,
//!   `B00_yesButton`) and tracks in every animation.
//! - The world map's + menu (`/Layout/corseSelectMenu/corseSelectMenu.arc`,
//!   `corseSelectMenu_11`): Quick Save's button copied as Settings under it,
//!   Title Screen a button lower, the window grown and raised clear of its
//!   Back hint; its group is `B01_noButton`.
//!
//! - The save-select screen (`/Layout/fileSelectBase/fileSelectBase.arc`,
//!   `fileSelectBase_18`): two empty panes at the right of the title bar,
//!   under the title's own pane, so they come in with the title when the
//!   screen opens: where the port draws its Multiplayer and Settings (cog)
//!   buttons, and where the game's corner cursor goes when either is picked.
//!
//! - A save file's card (`/Layout/dateFile/dateFile.arc`, `dateFile_09`): a
//!   text pane, `T_cheated_00`, hidden, at the top of the card; the port
//!   shows it on the files marked as cheated (the Gameplay options).
//!
//! [`crate::menu_settings`] teaches the game's code the buttons. An archive
//! whose panes aren't the expected ones is left as it is. Nothing is written
//! to disk: the game is handed the changed archives from memory.

use std::collections::{BTreeMap, BTreeSet};

use hle_ios::Disc;

pub const PAUSE_ARC: &str = "/Layout/pauseMenu/pauseMenu.arc";
pub const MAP_ARC: &str = "/Layout/corseSelectMenu/corseSelectMenu.arc";
pub const FILE_ARC: &str = "/Layout/fileSelectBase/fileSelectBase.arc";
/// The save-select screen's Settings (cog) and Multiplayer buttons: empty
/// panes the port draws its buttons on.
pub const FILE_COG_PANE: &str = "N_setCog_00";
pub const FILE_MULTI_PANE: &str = "N_setMulti_00";
pub const CARD_ARC: &str = "/Layout/dateFile/dateFile.arc";
/// A save file's card's "Cheated" text.
pub const CHEATED_PANE: &str = "T_cheated_00";
/// The copies' group names: m2d keeps the pointer it is given, so each is a
/// string in main.dol's constant data that no group of these menus uses.
pub const PAUSE_GROUP: &str = "B00_yesButton";
pub const MAP_GROUP: &str = "B01_noButton";

/// Builds the Settings buttons into the three archives as `disc` serves
/// them, the buttons reading `label`. Returns whether the pause window, the
/// world map's menu and the save-select screen got theirs.
pub fn add_buttons(disc: &mut Disc, label: &str) -> (bool, bool, bool) {
    let label: Vec<u16> = label.encode_utf16().collect();
    let mut one = |path: &str, f: fn(&mut Vec<u8>, &[u16]) -> Option<()>| {
        let Some(mut arc) = disc.file(path) else { return false };
        f(&mut arc, &label).is_some() && disc.replace(path, arc)
    };
    (one(PAUSE_ARC, patch_pause_archive), one(MAP_ARC, patch_map_archive), one(FILE_ARC, patch_file_archive))
}

/// Builds the hidden "Cheated" text, reading `label`, into the save files'
/// card as `disc` serves it. False if the card is not the expected one.
pub fn add_cheated_label(disc: &mut Disc, label: &str) -> bool {
    let label: Vec<u16> = label.encode_utf16().collect();
    let Some(mut arc) = disc.file(CARD_ARC) else { return false };
    patch_card_archive(&mut arc, &label).is_some() && disc.replace(CARD_ARC, arc)
}

fn be16(b: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([b[at], b[at + 1]])
}
fn be32(b: &[u8], at: usize) -> u32 {
    u32::from_be_bytes(b[at..at + 4].try_into().unwrap())
}
fn put16(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_be_bytes());
}
fn put32(b: &mut [u8], at: usize, v: u32) {
    b[at..at + 4].copy_from_slice(&v.to_be_bytes());
}
fn getf(b: &[u8], at: usize) -> f32 {
    f32::from_bits(be32(b, at))
}
fn putf(b: &mut [u8], at: usize, v: f32) {
    put32(b, at, v.to_bits());
}
/// A NUL-padded name of up to `n` bytes at `at`.
fn name_at(b: &[u8], at: usize, n: usize) -> String {
    let s = b.get(at..(at + n).min(b.len())).unwrap_or_default();
    String::from_utf8_lossy(&s[..s.iter().position(|&c| c == 0).unwrap_or(s.len())]).into_owned()
}
fn set_name(b: &mut [u8], at: usize, n: usize, name: &str) {
    b[at..at + n].fill(0);
    let len = name.len().min(n);
    b[at..at + len].copy_from_slice(&name.as_bytes()[..len]);
}

// ---------------------------------------------------------------- U8

pub(crate) struct Node {
    pub(crate) kind: u8,
    /// File: data offset and size; folder: parent and next index.
    a: u32,
    b: u32,
    pub(crate) name: String,
    path: String,
    pub(crate) data: Vec<u8>,
}

pub(crate) fn read_u8(arc: &[u8]) -> Option<Vec<Node>> {
    if arc.len() < 0x20 || be32(arc, 0) != 0x55AA_382D {
        return None;
    }
    let root = be32(arc, 4) as usize;
    let total = be32(arc, root.checked_add(8).filter(|&e| e + 4 <= arc.len())?) as usize;
    let header = be32(arc, 8) as usize;
    let strings_at = root + total * 12;
    if strings_at > arc.len() || root + header > arc.len() || strings_at > root + header {
        return None;
    }
    let strings = &arc[strings_at..root + header];
    let mut dirs: Vec<(String, u32)> = vec![(String::new(), total as u32)];
    let mut nodes = Vec::new();
    for i in 0..total {
        let n = &arc[root + i * 12..root + i * 12 + 12];
        let (kind, name_off, a, b) = (n[0], (be32(n, 0) & 0xFF_FFFF) as usize, be32(n, 4), be32(n, 8));
        let name = name_at(strings, name_off.min(strings.len()), strings.len().saturating_sub(name_off));
        while i > 0 && dirs.len() > 1 && i as u32 >= dirs.last().unwrap().1 {
            dirs.pop();
        }
        let mut path: String = dirs.iter().filter(|d| !d.0.is_empty()).map(|d| format!("{}/", d.0)).collect();
        path += &name;
        let mut data = Vec::new();
        if kind == 1 {
            if i > 0 {
                dirs.push((name.clone(), b));
            }
        } else {
            data = arc.get(a as usize..(a as usize).checked_add(b as usize)?)?.to_vec();
        }
        nodes.push(Node { kind, a, b, name, path, data });
    }
    Some(nodes)
}

fn write_u8(nodes: &[Node]) -> Vec<u8> {
    const ROOT: usize = 0x20;
    let mut strings = Vec::new();
    let mut offsets = Vec::new();
    for n in nodes {
        offsets.push(strings.len() as u32);
        strings.extend_from_slice(n.name.as_bytes());
        strings.push(0);
    }
    let header = nodes.len() * 12 + strings.len();
    let data_start = (ROOT + header + 0x1F) & !0x1F;
    let mut out = vec![0u8; data_start];
    put32(&mut out, 0, 0x55AA_382D);
    put32(&mut out, 4, ROOT as u32);
    put32(&mut out, 8, header as u32);
    put32(&mut out, 12, data_start as u32);
    out[ROOT + nodes.len() * 12..ROOT + header].copy_from_slice(&strings);
    for (i, n) in nodes.iter().enumerate() {
        let at = ROOT + i * 12;
        put32(&mut out, at, u32::from(n.kind) << 24 | offsets[i]);
        if n.kind == 1 {
            put32(&mut out, at + 4, n.a);
            put32(&mut out, at + 8, n.b);
            continue;
        }
        let pos = out.len() as u32;
        put32(&mut out, at + 4, pos);
        put32(&mut out, at + 8, n.data.len() as u32);
        out.extend_from_slice(&n.data);
        out.resize((out.len() + 0x1F) & !0x1F, 0);
    }
    out
}

// ---------------------------------------------------------------- BRLYT / BRLAN sections

#[derive(Clone)]
struct Section {
    kind: [u8; 4],
    /// With its 8-byte header.
    bytes: Vec<u8>,
}

fn read_sections(file: &[u8], magic: &[u8; 4]) -> Option<(Vec<u8>, Vec<Section>)> {
    if file.len() < 16 || &file[..4] != magic {
        return None;
    }
    let (header, count) = (be16(file, 12) as usize, be16(file, 14));
    let mut pos = header;
    let mut sections = Vec::new();
    for _ in 0..count {
        let size = be32(file.get(pos..pos + 8)?, 4) as usize;
        if size < 8 || pos + size > file.len() {
            return None;
        }
        sections.push(Section { kind: file[pos..pos + 4].try_into().unwrap(), bytes: file[pos..pos + size].to_vec() });
        pos += size;
    }
    Some((file[..header].to_vec(), sections))
}

fn write_sections(mut header: Vec<u8>, sections: &[Section]) -> Vec<u8> {
    for s in sections {
        header.extend_from_slice(&s.bytes);
    }
    let len = header.len() as u32;
    put32(&mut header, 8, len);
    put16(&mut header, 14, sections.len() as u16);
    header
}

fn is_pane(s: &Section) -> bool {
    matches!(&s.kind, b"pan1" | b"pic1" | b"txt1" | b"wnd1" | b"bnd1")
}

fn pane_name(s: &Section) -> String {
    name_at(&s.bytes, 0x0C, 16)
}

fn find_pane(sections: &[Section], name: &str) -> Option<usize> {
    sections.iter().position(|s| is_pane(s) && pane_name(s) == name)
}

/// The last section of a pane's subtree: the pane, its user data, and its
/// children's `pas1` ... `pae1`.
fn subtree_end(sections: &[Section], pane: usize) -> usize {
    let mut end = pane;
    while sections.get(end + 1).is_some_and(|s| &s.kind == b"usd1") {
        end += 1;
    }
    if sections.get(end + 1).is_some_and(|s| &s.kind == b"pas1") {
        let mut depth = 0;
        for (i, s) in sections.iter().enumerate().skip(end + 1) {
            if &s.kind == b"pas1" {
                depth += 1;
            }
            if &s.kind == b"pae1" {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
        }
    }
    end
}

fn move_pane(sections: &mut [Section], name: &str, dy: f32) {
    if let Some(i) = find_pane(sections, name) {
        let y = getf(&sections[i].bytes, 0x28);
        // Panes parked far away (as Newer parks unused ones) stay there.
        if y.abs() < 1000.0 {
            putf(&mut sections[i].bytes, 0x28, y + dy);
        }
    }
}

fn grow_pane(sections: &mut [Section], name: &str, dh: f32) {
    if let Some(i) = find_pane(sections, name) {
        let h = getf(&sections[i].bytes, 0x48);
        putf(&mut sections[i].bytes, 0x48, h + dh);
    }
}

/// A text pane's text set to `text` (its buffer room for at least 32 letters).
fn set_pane_text(s: &mut Section, text: &[u16]) {
    if &s.kind != b"txt1" || s.bytes.len() < 0x74 {
        return;
    }
    let at = be32(&s.bytes, 0x58) as usize;
    if at < 0x74 || at > s.bytes.len() {
        return;
    }
    s.bytes.truncate(at);
    for &c in text {
        s.bytes.extend_from_slice(&c.to_be_bytes());
    }
    s.bytes.extend_from_slice(&[0, 0]);
    let len = ((text.len() + 1) * 2) as u16;
    while s.bytes.len() % 4 != 0 {
        s.bytes.push(0);
    }
    put16(&mut s.bytes, 0x4C, len.max(64));
    put16(&mut s.bytes, 0x4E, len);
    let size = s.bytes.len() as u32;
    put32(&mut s.bytes, 4, size);
}

/// The group `from`, copied as `to` right after it with its members renamed
/// (members not copied are left out). False if there is no such group or
/// nothing of it was copied.
fn copy_group(sections: &mut Vec<Section>, from: &str, to: &str, renames: &BTreeMap<String, String>) -> bool {
    let Some(i) = sections.iter().position(|s| &s.kind == b"grp1" && s.bytes.len() >= 0x1C && name_at(&s.bytes, 8, 16) == from) else { return false };
    let s = &sections[i].bytes;
    let mut group = vec![0u8; 0x1C];
    group[..4].copy_from_slice(b"grp1");
    set_name(&mut group, 8, 16, to);
    let mut kept = 0u16;
    for m in 0..be16(s, 0x18) as usize {
        let at = 0x1C + m * 16;
        if at + 16 > s.len() {
            break;
        }
        if let Some(copy) = renames.get(&name_at(s, at, 16)) {
            group.resize(group.len() + 16, 0);
            let end = group.len();
            set_name(&mut group, end - 16, 16, copy);
            kept += 1;
        }
    }
    put16(&mut group, 0x18, kept);
    let size = group.len() as u32;
    put32(&mut group, 4, size);
    sections.insert(i + 1, Section { kind: *b"grp1", bytes: group });
    kept > 0
}

/// A BRLAN with each pane the copy came from also animated under the copy's
/// name, and the copy's group bound beside `from_group`. With `only_bound`,
/// an animation not bound to `from_group` is left as it is.
fn patch_animation(file: &mut Vec<u8>, renames: &BTreeMap<String, String>, from_group: &str, to_group: &str, only_bound: bool) -> Option<()> {
    let (header, mut sections) = read_sections(file, b"RLAN")?;
    let bound = sections.iter().any(|s| {
        let b = &s.bytes;
        &s.kind == b"pat1" && b.len() >= 0x1C && (0..be16(b, 0x0A) as usize).any(|g| name_at(b, be32(b, 0x10) as usize + g * 20, 16) == from_group)
    });
    if only_bound && !bound {
        return Some(());
    }
    for s in &mut sections {
        let b = &mut s.bytes;
        // m2d binds an animation to the groups its pat1 lists.
        if &s.kind == b"pat1" && b.len() >= 0x1C {
            let count = be16(b, 0x0A) as usize;
            let groups = be32(b, 0x10) as usize;
            if groups + count * 20 != b.len() {
                return None; // the groups are its end, as authored
            }
            if let Some(g) = (0..count).find(|g| name_at(b, groups + g * 20, 16) == from_group) {
                let mut copy = b[groups + g * 20..groups + g * 20 + 20].to_vec();
                set_name(&mut copy, 0, 16, to_group);
                b.extend_from_slice(&copy);
                put16(b, 0x0A, count as u16 + 1);
                let size = b.len() as u32;
                put32(b, 4, size);
            }
            continue;
        }
        if &s.kind != b"pai1" {
            continue;
        }
        if b.len() < 0x14 || be16(b, 0x0C) != 0 {
            return None; // texture patterns: not expected here
        }
        let count = be16(b, 0x0E) as usize;
        let table = be32(b, 0x10) as usize;
        if table + count * 4 > b.len() {
            return None;
        }
        let mut entries = Vec::new();
        for e in 0..count {
            let at = be32(b, table + e * 4) as usize;
            let next = if e + 1 < count { be32(b, table + (e + 1) * 4) as usize } else { b.len() };
            if at >= next || next > b.len() {
                return None;
            }
            entries.push(b[at..next].to_vec());
        }
        for e in 0..entries.len() {
            if let Some(to) = renames.get(&name_at(&entries[e], 0, 20)) {
                let mut copy = entries[e].clone();
                set_name(&mut copy, 0, 20, to);
                entries.push(copy);
            }
        }
        let mut out = b[..table].to_vec();
        put16(&mut out, 0x0E, entries.len() as u16);
        let table_at = out.len();
        out.resize(out.len() + entries.len() * 4, 0);
        for (e, entry) in entries.iter().enumerate() {
            let pos = out.len() as u32;
            put32(&mut out, table_at + e * 4, pos);
            out.extend_from_slice(entry);
        }
        let size = out.len() as u32;
        put32(&mut out, 4, size);
        *b = out;
    }
    *file = write_sections(header, &sections);
    Some(())
}

/// Every pane name in the layout.
fn pane_names(sections: &[Section]) -> BTreeSet<String> {
    sections.iter().filter(|s| is_pane(s)).map(pane_name).collect()
}

/// The pause window's copy of Exit's pane names: unique, within 16 letters.
fn pause_copy_name(name: &str) -> String {
    if name == "P_SBBase_02" {
        return "P_SBBase_03".into();
    }
    if let Some(at) = name.find("modoru") {
        return format!("{}setting{}", &name[..at], &name[at + 6..]);
    }
    if let Some(stem) = name.strip_suffix("_02") {
        return format!("{stem}_03");
    }
    if let Some(stem) = name.strip_suffix("_2") {
        return format!("{stem}_3");
    }
    if let Some(rest) = name.strip_prefix("Btn2") {
        return format!("Btn3{rest}");
    }
    format!("{name}S")
}

/// `pauseMenu_16.brlyt` with the Settings button; `renames`: Exit's panes
/// to their copies.
fn patch_pause_layout(file: &mut Vec<u8>, renames: &mut BTreeMap<String, String>, label: &[u16]) -> Option<()> {
    let (header, mut sections) = read_sections(file, b"RLYT")?;
    let cont = find_pane(&sections, "P_SBBase_00")?;
    let exit = find_pane(&sections, "P_SBBase_02")?;
    if find_pane(&sections, "P_SBBase_03").is_some() || find_pane(&sections, "W_N_pauseMenu_00").is_none() || find_pane(&sections, "N_pauseMenu_01").is_none() {
        return None;
    }
    let pitch = getf(&sections[cont].bytes, 0x28) - getf(&sections[exit].bytes, 0x28);
    if !(pitch > 20.0 && pitch < 200.0) {
        return None;
    }
    let existing = pane_names(&sections);
    // Exit's subtree, without the grey cover that marks it unavailable.
    let end = subtree_end(&sections, exit);
    let mut copy = Vec::new();
    let mut i = exit;
    while i <= end {
        if is_pane(&sections[i]) && pane_name(&sections[i]) == "P_shadowBlack" {
            i = subtree_end(&sections, i) + 1;
            continue;
        }
        copy.push(sections[i].clone());
        i += 1;
    }
    for s in copy.iter_mut().filter(|s| is_pane(s)) {
        let from = pane_name(s);
        let to = pause_copy_name(&from);
        if to.len() > 16 || existing.contains(&to) {
            return None;
        }
        s.bytes[0x0C..0x1C].fill(0);
        s.bytes[0x0C..0x0C + to.len()].copy_from_slice(to.as_bytes());
        renames.insert(from.clone(), to);
        set_pane_text(s, label);
        // Newer colours Exit's three slices red: Settings takes Continue's.
        if &s.kind == b"pic1" && from.starts_with("Btn2") && s.bytes.len() >= 0x5E {
            if let Some(c) = find_pane(&sections, &format!("Btn1{}", &from[4..])) {
                let colour: [u8; 2] = sections[c].bytes[0x5C..0x5E].try_into().unwrap();
                s.bytes[0x5C..0x5E].copy_from_slice(&colour);
            }
        }
    }
    // Settings takes Exit's place, Exit moves a button lower, and the
    // window grows by a button with everything in it raised by half of
    // that, so it stays centred.
    move_pane(&mut sections, "P_SBBase_02", -pitch);
    let after = end + 1;
    sections.splice(after..after, copy);
    for grow in ["P_bg_00", "W_N_pauseMenu_00", "BG_Left", "BG_Middle", "BG_Right"] {
        grow_pane(&mut sections, grow, pitch);
    }
    for up in ["N_pauseMenu_01", "N_worldText_00", "N_osusumeText_00", "N_multiText_00", "LevelNumShadow", "LevelNameShadow"] {
        move_pane(&mut sections, up, pitch / 2.0);
    }
    if !copy_group(&mut sections, "B02_menuButton", PAUSE_GROUP, renames) {
        return None;
    }
    *file = write_sections(header, &sections);
    Some(())
}

/// `corseSelectMenu_11.brlyt` with Settings under Quick Save; copies are
/// named `..._06` (the layout already has a `T_corseSelect_05`).
fn patch_map_layout(file: &mut Vec<u8>, renames: &mut BTreeMap<String, String>, label: &[u16]) -> Option<()> {
    let (header, mut sections) = read_sections(file, b"RLYT")?;
    let save = find_pane(&sections, "P_SBBase_03")?;
    let above = find_pane(&sections, "P_SBBase_02")?;
    for needed in ["P_SBBase_04", "W_corseSelect_01", "N_corseSelect_01", "P_winbg_01"] {
        find_pane(&sections, needed)?;
    }
    if find_pane(&sections, "P_SBBase_06").is_some() {
        return None;
    }
    let pitch = getf(&sections[above].bytes, 0x28) - getf(&sections[save].bytes, 0x28);
    if !(pitch > 20.0 && pitch < 200.0) {
        return None;
    }
    let existing = pane_names(&sections);
    let end = subtree_end(&sections, save);
    let mut copy = sections[save..=end].to_vec();
    for s in copy.iter_mut().filter(|s| is_pane(s)) {
        let from = pane_name(s);
        let to = match from.strip_suffix("03") {
            Some(stem) => format!("{stem}06"),
            None => format!("{from}S"),
        };
        if to.len() > 16 || existing.contains(&to) {
            return None;
        }
        s.bytes[0x0C..0x1C].fill(0);
        s.bytes[0x0C..0x0C + to.len()].copy_from_slice(to.as_bytes());
        renames.insert(from, to);
        set_pane_text(s, label);
    }
    let y = getf(&copy[0].bytes, 0x28);
    putf(&mut copy[0].bytes, 0x28, y - pitch);
    let after = end + 1;
    sections.splice(after..after, copy);
    // The buttons move up half a button, Title Screen down half, the
    // separator and its band with it.
    move_pane(&mut sections, "N_corseSelect_01", pitch / 2.0);
    move_pane(&mut sections, "P_SBBase_04", -pitch);
    move_pane(&mut sections, "P_line_00", -pitch / 2.0);
    move_pane(&mut sections, "P_winbg_00", -pitch / 2.0);
    grow_pane(&mut sections, "P_winbg_01", pitch);
    grow_pane(&mut sections, "W_corseSelect_01", pitch);
    // The window's frame then ends clear of "Back (1)" (N_back, at -183).
    let window = find_pane(&sections, "W_corseSelect_01")?;
    let half = getf(&sections[window].bytes, 0x48) / 2.0;
    move_pane(&mut sections, "N_corseSelect_00", (half - 168.0).max(0.0));
    if !copy_group(&mut sections, "B03_Button", MAP_GROUP, renames) {
        return None;
    }
    *file = write_sections(header, &sections);
    Some(())
}

/// One archive: its layout `layout` patched by `f`, then every animation
/// (`only_bound`: see [`patch_animation`]).
fn patch_archive(arc: &mut Vec<u8>, layout: &str, label: &[u16], f: fn(&mut Vec<u8>, &mut BTreeMap<String, String>, &[u16]) -> Option<()>, from_group: &str, to_group: &str, only_bound: bool) -> Option<()> {
    let mut nodes = read_u8(arc)?;
    let mut renames = BTreeMap::new();
    let mut found = false;
    for n in nodes.iter_mut().filter(|n| n.kind == 0 && n.path.ends_with(".brlyt") && n.path.contains(layout)) {
        f(&mut n.data, &mut renames, label)?;
        found = true;
    }
    if !found {
        return None;
    }
    let mut animations = 0;
    for n in nodes.iter_mut().filter(|n| n.kind == 0 && n.path.ends_with(".brlan")) {
        patch_animation(&mut n.data, &renames, from_group, to_group, only_bound)?;
        animations += 1;
    }
    if animations < 4 {
        return None;
    }
    *arc = write_u8(&nodes);
    Some(())
}

fn patch_pause_archive(arc: &mut Vec<u8>, label: &[u16]) -> Option<()> {
    // Every animation: onButton, idleButton, offButon (sic) and hitButton for
    // the copy's group, and the window's own, which animate Exit's panes too.
    patch_archive(arc, "pauseMenu_16", label, patch_pause_layout, "B02_menuButton", PAUSE_GROUP, false)
}

fn patch_map_archive(arc: &mut Vec<u8>, label: &[u16]) -> Option<()> {
    patch_archive(arc, "corseSelectMenu_11", label, patch_map_layout, "B03_Button", MAP_GROUP, false)
}

/// `fileSelectBase_18.brlyt` with the Settings and Multiplayer buttons'
/// places: a pane of its own under the title's (`N_titlePos_00`, animated
/// when the screen opens and closes, so the buttons come and go with the
/// title), at the right of the title bar, holding an empty pane for each
/// button, Multiplayer at the left of the cog. The two keep their shape and
/// their distance on a 16:9 screen (bit 2: the 16:9 shrink also moves a
/// pane toward its parent), so their holder alone moves out to the right.
fn patch_file_layout(file: &mut Vec<u8>) -> Option<()> {
    /// The buttons' sizes, in layout units.
    const COG: (f32, f32) = (60.0, 46.0);
    const MULTI: (f32, f32) = (200.0, 46.0);
    const GAP: f32 = 10.0;
    /// The cog's right edge, in the 4:3 picture (the screen ends at 304).
    const RIGHT: f32 = 288.0;
    /// The holder's x: where the OK-button copy that was the Settings
    /// button sat (its centre, 184 wide).
    const HOLDER_X: f32 = RIGHT - (184.0 + 8.0) / 2.0;
    let (header, mut sections) = read_sections(file, b"RLYT")?;
    let title = find_pane(&sections, "N_titlePos_00")?;
    let existing = pane_names(&sections);
    if [FILE_COG_PANE, FILE_MULTI_PANE, "N_setPos_00"].iter().any(|n| existing.contains(*n)) {
        return None;
    }
    if &sections[title].kind != b"pan1" || sections[title].bytes.len() < 0x4C {
        return None;
    }
    // An empty pane from the title's: named, shown, at (x, y), unscaled.
    let pane = |name: &str, x: f32, y: f32, size: (f32, f32), flags: u8| {
        let mut p = sections[title].clone();
        p.bytes.truncate(0x4C);
        put32(&mut p.bytes, 4, 0x4C);
        set_name(&mut p.bytes, 0x0C, 16, name);
        p.bytes[8] = flags;
        p.bytes[9] = 4; // centred on its place
        p.bytes[0xA] = 255;
        putf(&mut p.bytes, 0x24, x);
        putf(&mut p.bytes, 0x28, y);
        putf(&mut p.bytes, 0x2C, 0.0);
        for at in [0x30, 0x34, 0x38] {
            putf(&mut p.bytes, at, 0.0);
        }
        for at in [0x3C, 0x40] {
            putf(&mut p.bytes, at, 1.0);
        }
        putf(&mut p.bytes, 0x44, size.0);
        putf(&mut p.bytes, 0x48, size.1);
        p
    };
    let title_x = getf(&sections[title].bytes, 0x24);
    let title_y = find_pane(&sections, "T_title_00").map_or(12.0, |t| getf(&sections[t].bytes, 0x28));
    let cog_x = RIGHT - COG.0 / 2.0 - HOLDER_X;
    let multi_x = cog_x - (COG.0 + MULTI.0) / 2.0 - GAP;
    let holder = pane("N_setPos_00", HOLDER_X - title_x, title_y, (0.0, 0.0), 1);
    let cog = pane(FILE_COG_PANE, cog_x, 0.0, COG, 1 | 4);
    let multi = pane(FILE_MULTI_PANE, multi_x, 0.0, MULTI, 1 | 4);
    let mark = |kind: &[u8; 4]| Section { kind: *kind, bytes: [&kind[..], &[0, 0, 0, 8]].concat() };
    // Last among the title pane's children.
    let end = subtree_end(&sections, title);
    if end == title || &sections[end].kind != b"pae1" {
        return None;
    }
    sections.splice(end..end, [holder, mark(b"pas1"), multi, cog, mark(b"pae1")]);
    *file = write_sections(header, &sections);
    Some(())
}

/// `dateFile_09.brlyt` with the "Cheated" text: a copy of the "World" text,
/// smaller and orange, centred at the card's top, last among the card's
/// panes (drawn over them) and hidden.
fn patch_card_layout(file: &mut Vec<u8>, label: &[u16]) -> Option<()> {
    let (header, mut sections) = read_sections(file, b"RLYT")?;
    let world = find_pane(&sections, "T_worldNumber_00")?;
    let card = find_pane(&sections, "N_dateFile_00")?;
    if find_pane(&sections, CHEATED_PANE).is_some() || &sections[world].kind != b"txt1" || sections[world].bytes.len() < 0x74 {
        return None;
    }
    let mut text = sections[world].clone();
    let b = &mut text.bytes;
    set_name(b, 0x0C, 16, CHEATED_PANE);
    b[8] &= !1; // hidden
    b[9] = 4; // centred on its place
    b[0x54] = 4; // and its text in it
    putf(b, 0x24, 0.0);
    putf(b, 0x28, 70.0);
    putf(b, 0x44, 160.0);
    putf(b, 0x48, 24.0);
    for at in [0x64, 0x68] {
        putf(b, at, 22.0);
    }
    for at in [0x5C, 0x60] {
        b[at..at + 4].copy_from_slice(&[255, 144, 64, 255]);
    }
    set_pane_text(&mut text, label);
    let end = subtree_end(&sections, card);
    if end == card || &sections[end].kind != b"pae1" {
        return None;
    }
    sections.insert(end, text);
    *file = write_sections(header, &sections);
    Some(())
}

fn patch_card_archive(arc: &mut Vec<u8>, label: &[u16]) -> Option<()> {
    let mut nodes = read_u8(arc)?;
    let layout = nodes.iter_mut().find(|n| n.kind == 0 && n.path.ends_with(".brlyt") && n.path.contains("dateFile_09"))?;
    patch_card_layout(&mut layout.data, label)?;
    *arc = write_u8(&nodes);
    Some(())
}

fn patch_file_archive(arc: &mut Vec<u8>, _label: &[u16]) -> Option<()> {
    let mut nodes = read_u8(arc)?;
    let layout = nodes.iter_mut().find(|n| n.kind == 0 && n.path.ends_with(".brlyt") && n.path.contains("fileSelectBase_18"))?;
    patch_file_layout(&mut layout.data)?;
    *arc = write_u8(&nodes);
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// On the user's own disc (`NSMBW_GAME`, the extracted folder):
    /// `cargo test -p hle-os -- --ignored`.
    #[test]
    #[ignore]
    fn patches_the_disc_menus() {
        let game = std::env::var("NSMBW_GAME").expect("NSMBW_GAME");
        let mut disc = Disc::open(std::path::Path::new(&game)).unwrap();
        let (pause_before, map_before) = (disc.file(PAUSE_ARC).unwrap(), disc.file(MAP_ARC).unwrap());
        let file_before = disc.file(FILE_ARC).unwrap();
        assert_eq!(add_buttons(&mut disc, "Settings"), (true, true, true));
        let card_before = disc.file(CARD_ARC).unwrap();
        assert!(add_cheated_label(&mut disc, "Cheated"));
        let card = disc.file(CARD_ARC).unwrap();
        assert!(card.len() >= card_before.len());
        let nodes = read_u8(&card).unwrap();
        let lyt = nodes.iter().find(|n| n.path.contains("dateFile_09") && n.path.ends_with(".brlyt")).unwrap();
        let (_, sections) = read_sections(&lyt.data, b"RLYT").unwrap();
        let label = &sections[find_pane(&sections, CHEATED_PANE).unwrap()];
        assert_eq!(label.bytes[8] & 1, 0, "hidden until a file is marked");
        // The save select's two places, each centred and keeping its shape.
        let after = disc.file(FILE_ARC).unwrap();
        assert!(after.len() > file_before.len(), "{FILE_ARC} grew");
        let nodes = read_u8(&after).unwrap();
        let lyt = nodes.iter().find(|n| n.path.contains("fileSelectBase_18") && n.path.ends_with(".brlyt")).unwrap();
        let (_, sections) = read_sections(&lyt.data, b"RLYT").unwrap();
        for pane in [FILE_COG_PANE, FILE_MULTI_PANE] {
            let p = &sections[find_pane(&sections, pane).unwrap()];
            assert_eq!((p.bytes[8], p.bytes[9]), (5, 4), "{pane}");
        }
        assert!(getf(&sections[find_pane(&sections, FILE_MULTI_PANE).unwrap()].bytes, 0x24) < getf(&sections[find_pane(&sections, FILE_COG_PANE).unwrap()].bytes, 0x24));
        for (path, before, layout, pane, group) in [(PAUSE_ARC, pause_before, "pauseMenu_16", "P_SBBase_03", PAUSE_GROUP), (MAP_ARC, map_before, "corseSelectMenu_11", "P_SBBase_06", MAP_GROUP)] {
            let after = disc.file(path).unwrap();
            assert!(after.len() > before.len(), "{path} grew");
            let nodes = read_u8(&after).unwrap();
            assert_eq!(nodes.len(), read_u8(&before).unwrap().len());
            let lyt = nodes.iter().find(|n| n.path.contains(layout) && n.path.ends_with(".brlyt")).unwrap();
            let (_, sections) = read_sections(&lyt.data, b"RLYT").unwrap();
            assert!(find_pane(&sections, pane).is_some(), "{pane}");
            assert!(sections.iter().any(|s| &s.kind == b"grp1" && name_at(&s.bytes, 8, 16) == group), "{group}");
            let text: Vec<String> = sections.iter().filter(|s| &s.kind == b"txt1" && (pane_name(s).ends_with("06") || pane_name(s).contains("setting") || pane_name(s).starts_with("T_set_"))).map(pane_name).collect();
            println!("{path}: {} bytes -> {}, text panes {text:?}", before.len(), after.len());
            for n in nodes.iter().filter(|n| n.path.ends_with(".brlan")) {
                let (_, s) = read_sections(&n.data, b"RLAN").unwrap();
                assert!(!s.is_empty(), "{}", n.path);
            }
        }
        // The FST in RAM points at the new copies; the originals stay.
        assert_ne!(disc.fst(), std::fs::read(std::path::Path::new(&game).join("sys/fst.bin")).unwrap());
    }
}
