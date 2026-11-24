//! Pictures from the player's own game folder, read at run time and never
//! stored: the title logo and the course previews, named per version in
//! `versions.toml`. The archive and texture readers follow the game's menu
//! code on branch `ui` (`crates/frontend/src/ui/layout.rs`); the pixel
//! formats are decoded by `gx`, as the game's renderer does.

use std::path::Path;

use crate::versions::{Art, ArtRef};

/// A picture as straight-alpha RGBA, rows top down.
pub struct Picture {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

/// What a game folder gave.
#[derive(Default)]
pub struct Pictures {
    pub logo: Option<Picture>,
    pub gallery: Vec<Picture>,
    /// The course background's models, far to near.
    pub background: Vec<crate::gamebg::Model>,
    /// The game's banner.
    pub banner: Option<crate::banner::Banner>,
}

/// The most course previews shown.
const GALLERY_MAX: usize = 40;

fn be16(p: &[u8], at: usize) -> usize {
    p.get(at..at + 2).map_or(0, |b| usize::from(u16::from_be_bytes([b[0], b[1]])))
}

fn be32(p: &[u8], at: usize) -> usize {
    p.get(at..at + 4).map_or(0, |b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize)
}

/// A U8 archive's files: name (without folders), offset, size.
pub(crate) fn list_u8(data: &[u8]) -> Vec<(String, usize, usize)> {
    let mut files = Vec::new();
    if data.len() < 0x20 || be32(data, 0) != 0x55AA_382D {
        return files;
    }
    let root = be32(data, 4);
    let total = be32(data, root + 8);
    let strings = root.saturating_add(total.saturating_mul(12));
    if total == 0 || strings > data.len() {
        return files;
    }
    for i in 1..total {
        let node = root + i * 12;
        let type_name = be32(data, node);
        if type_name >> 24 != 0 {
            continue;
        }
        let name_at = strings + (type_name & 0xFF_FFFF);
        let name = data.get(name_at..).unwrap_or(&[]);
        let name = &name[..name.iter().position(|&c| c == 0).unwrap_or(name.len()).min(256)];
        files.push((String::from_utf8_lossy(name).into_owned(), be32(data, node + 4), be32(data, node + 8)));
    }
    files
}

/// A TPL file's first texture.
pub(crate) fn parse_tpl(data: &[u8]) -> Option<Picture> {
    if data.len() < 12 || be32(data, 0) != 0x0020_AF30 || be32(data, 4) == 0 {
        return None;
    }
    let table = be32(data, 8);
    let (header, palette_header) = (be32(data, table), be32(data, table + 4));
    if header == 0 || header + 0x24 > data.len() {
        return None;
    }
    let (height, width, format, offset) = (be16(data, header), be16(data, header + 2), be32(data, header + 4), be32(data, header + 8));
    if width == 0 || height == 0 || width > 2048 || height > 2048 || !matches!(format, 0..=6 | 8..=10 | 14) {
        return None;
    }
    let (mut palette, mut palette_format): (&[u8], usize) = (&[], 0);
    if palette_header != 0 && palette_header + 12 <= data.len() {
        palette_format = be32(data, palette_header + 4);
        let at = be32(data, palette_header + 8);
        palette = data.get(at..(at + be16(data, palette_header) * 2).min(data.len())).unwrap_or(&[]);
    }
    let rgba = gx::texture::decode(format as u32, width, height, data.get(offset..)?, palette, palette_format as u32);
    (rgba.len() >= width * height * 4).then_some(Picture { width, height, rgba })
}

/// `file` in `game`; one in the USA disc's region folder (`files/US/`) is
/// taken from the disc's own (`files/EU/`, `files/JP/`, ...) if it has that
/// folder instead.
pub(crate) fn in_region(game: &Path, file: &str) -> std::path::PathBuf {
    let path = game.join(file);
    if let Some(rest) = file.strip_prefix("files/US/").filter(|_| !path.exists()) {
        for region in ["EU", "JP", "KR", "TW"] {
            let other = game.join("files").join(region).join(rest);
            if other.exists() {
                return other;
            }
        }
    }
    path
}

/// The textures of `art`'s archive in `game` that it names: the one called
/// `name` (or the largest), or all whose names start with `prefix`.
fn read(game: &Path, art: &ArtRef, limit: usize) -> Vec<Picture> {
    let Ok(data) = std::fs::read(in_region(game, &art.archive)) else { return Vec::new() };
    let mut textures: Vec<(String, usize, usize)> = list_u8(&data).into_iter().filter(|f| f.0.to_ascii_lowercase().ends_with(".tpl")).collect();
    if !art.prefix.is_empty() {
        textures.retain(|f| f.0.starts_with(&art.prefix));
    } else if !art.name.is_empty() {
        textures.retain(|f| f.0 == art.name);
    } else {
        textures.sort_by_key(|f| std::cmp::Reverse(f.2));
        textures.truncate(1);
    }
    // Spread a long gallery's picks over the whole list.
    let stride = textures.len().div_ceil(limit).max(1);
    textures.iter().step_by(stride).filter_map(|(_, offset, size)| parse_tpl(data.get(*offset..offset.checked_add(*size)?)?)).collect()
}

/// Reads the version's pictures from its game folder. Whatever is missing or
/// unreadable is left out: the launcher has its own art for that case.
pub fn load(game: &Path, art: &Art) -> Pictures {
    let background = art.background.as_ref().map(|b| b.models.iter().filter_map(|m| crate::gamebg::load(game, m)).collect()).unwrap_or_default();
    Pictures { logo: art.logo.as_ref().and_then(|a| read(game, a, 1).into_iter().next()), gallery: art.gallery.as_ref().map(|a| read(game, a, GALLERY_MAX)).unwrap_or_default(), background, banner: art.banner.as_ref().and_then(|b| crate::banner::load(game, &b.file, &b.hide)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A U8 archive holding one folder with the given files.
    fn u8_archive(files: &[(&str, Vec<u8>)]) -> Vec<u8> {
        let mut names = vec![0u8]; // the root's empty name
        let folder_name = names.len();
        names.extend_from_slice(b"timg\0");
        let mut nodes = Vec::new();
        let total = files.len() + 2;
        nodes.extend_from_slice(&[1, 0, 0, 0, 0, 0, 0, 0]);
        nodes.extend_from_slice(&(total as u32).to_be_bytes());
        nodes.extend_from_slice(&(0x0100_0000 | folder_name as u32).to_be_bytes());
        nodes.extend_from_slice(&0u32.to_be_bytes());
        nodes.extend_from_slice(&(total as u32).to_be_bytes());
        let data_start = 0x20 + total * 12 + files.iter().map(|f| f.0.len() + 1).sum::<usize>() + names.len();
        let mut data = Vec::new();
        for (name, bytes) in files {
            nodes.extend_from_slice(&(names.len() as u32).to_be_bytes());
            nodes.extend_from_slice(&((data_start + data.len()) as u32).to_be_bytes());
            nodes.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
            names.extend_from_slice(name.as_bytes());
            names.push(0);
            data.extend_from_slice(bytes);
        }
        let mut out = vec![0x55, 0xAA, 0x38, 0x2D, 0, 0, 0, 0x20];
        out.resize(0x20, 0);
        out.extend_from_slice(&nodes);
        out.extend_from_slice(&names);
        assert_eq!(out.len(), data_start);
        out.extend_from_slice(&data);
        out
    }

    /// A TPL holding one RGB565 texture of a single colour.
    fn tpl(width: u16, height: u16, rgb565: u16) -> Vec<u8> {
        let mut t = vec![0x00, 0x20, 0xAF, 0x30, 0, 0, 0, 1, 0, 0, 0, 0x0C, 0, 0, 0, 0x14, 0, 0, 0, 0];
        t.extend_from_slice(&height.to_be_bytes());
        t.extend_from_slice(&width.to_be_bytes());
        t.extend_from_slice(&4u32.to_be_bytes());
        t.extend_from_slice(&0x40u32.to_be_bytes());
        t.resize(0x40, 0);
        let blocks = usize::from(width.div_ceil(4)) * usize::from(height.div_ceil(4));
        for _ in 0..blocks * 16 {
            t.extend_from_slice(&rgb565.to_be_bytes());
        }
        t
    }

    #[test]
    fn pictures_come_out_of_the_games_archives() {
        let game = std::env::temp_dir().join(format!("nsmbw-launcher-art-{}", std::process::id()));
        std::fs::create_dir_all(game.join("files")).unwrap();
        let archive = u8_archive(&[("im_W1_1.tpl", tpl(8, 4, 0xF800)), ("im_W1_2.tpl", tpl(8, 4, 0x07E0)), ("msk_coin01.tpl", tpl(4, 4, 0x001F)), ("logo.tpl", tpl(16, 8, 0xFFFF)), ("notes.txt", vec![1, 2, 3])]);
        std::fs::write(game.join("files").join("art.arc"), archive).unwrap();
        let art = |name: &str, prefix: &str| ArtRef { archive: "files/art.arc".into(), name: name.into(), prefix: prefix.into() };

        let p = load(&game, &Art { logo: Some(art("logo.tpl", "")), gallery: Some(art("", "im_W")), background: None, banner: None });
        let logo = p.logo.unwrap();
        assert_eq!((logo.width, logo.height), (16, 8));
        assert_eq!(&logo.rgba[..4], &[255, 255, 255, 255]);
        assert_eq!(p.gallery.len(), 2);
        assert_eq!(&p.gallery[0].rgba[..4], &[255, 0, 0, 255]);
        assert_eq!(&p.gallery[1].rgba[..4], &[0, 255, 0, 255]);

        // No name: the largest texture. A missing archive: nothing.
        let largest = load(&game, &Art { logo: Some(art("", "")), gallery: None, background: None, banner: None });
        assert_eq!(largest.logo.unwrap().width, 16);
        let none = load(&game, &Art { logo: Some(ArtRef { archive: "files/none.arc".into(), name: String::new(), prefix: String::new() }), gallery: None, background: None, banner: None });
        assert!(none.logo.is_none() && none.gallery.is_empty());
        std::fs::remove_dir_all(&game).unwrap();
    }
}
