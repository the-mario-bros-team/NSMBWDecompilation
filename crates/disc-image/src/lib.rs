//! A Wii disc image extracted to a folder the way Dolphin's Extract Entire
//! Disc lays out the game partition, which is what the port plays from:
//! `sys/` (boot.bin, bi2.bin, apploader.img, main.dol, fst.bin), `files/`,
//! and the partition's ticket.bin, tmd.bin, cert.bin and h3.bin.
//!
//! The images are read by Encounter's `nod` crate (MIT OR Apache-2.0): ISO,
//! WBFS, RVZ and WIA, CISO, GCZ, NFS and TGC. It decrypts the game partition
//! of an ISO or WBFS; RVZ and WIA keep it decrypted already.

use std::fs::File;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use nod::{Disc, PartitionKind};

/// The file name endings of the images [`Image::open`] reads.
pub const EXTENSIONS: &[&str] = &["iso", "gcm", "wbfs", "rvz", "wia", "ciso", "gcz", "nfs", "tgc"];

/// Whether `path` is named like a disc image.
pub fn is_image(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| EXTENSIONS.iter().any(|x| e.eq_ignore_ascii_case(x)))
}

/// An opened disc image.
pub struct Image {
    disc: Disc,
    format: String,
}

/// An open file read at positions, so the clones `nod` makes to read in
/// parallel each keep their own place.
#[derive(Clone)]
struct SharedFile {
    file: Arc<File>,
    pos: u64,
    len: u64,
}

impl Read for SharedFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        #[cfg(unix)]
        let n = std::os::unix::fs::FileExt::read_at(&*self.file, buf, self.pos)?;
        #[cfg(windows)]
        let n = std::os::windows::fs::FileExt::seek_read(&*self.file, buf, self.pos)?;
        self.pos += n as u64;
        Ok(n)
    }
}

impl Seek for SharedFile {
    fn seek(&mut self, to: SeekFrom) -> std::io::Result<u64> {
        let pos = match to {
            SeekFrom::Start(p) => Some(p),
            SeekFrom::End(d) => self.len.checked_add_signed(d),
            SeekFrom::Current(d) => self.pos.checked_add_signed(d),
        };
        self.pos = pos.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "seek before the start"))?;
        Ok(self.pos)
    }
}

impl Image {
    /// Opens the image at `path`. The error is a sentence for the player.
    pub fn open(path: &Path) -> Result<Self, String> {
        Self::checked(Disc::new(path))
    }

    /// Opens an image from an open file (on Android, one the player picked).
    pub fn from_file(file: File) -> Result<Self, String> {
        let len = file.metadata().map_err(|e| e.to_string())?.len();
        Self::checked(Disc::new_stream(Box::new(SharedFile { file: Arc::new(file), pos: 0, len })))
    }

    fn checked(disc: nod::Result<Disc>) -> Result<Self, String> {
        let disc = disc.map_err(|e| format!("This is not a disc image the port can read ({e})."))?;
        if !disc.header().is_wii() {
            return Err(if disc.header().is_gamecube() { "This is a GameCube disc, not a Wii one.".into() } else { "This is not a Wii disc image.".into() });
        }
        let format = disc.meta().format.to_string();
        Ok(Self { disc, format })
    }

    /// The disc's game ID: "SMNE01".
    pub fn id(&self) -> String {
        self.disc.header().game_id_str().to_string()
    }

    /// The disc's revision (its version byte).
    pub fn revision(&self) -> u8 {
        self.disc.header().disc_version
    }

    /// The game's title, as the disc header names it.
    pub fn title(&self) -> String {
        self.disc.header().game_title_str().to_string()
    }

    /// "RVZ", "WBFS", "ISO", ...
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Writes the game partition into `root` (which should be empty): the
    /// layout in the crate's description. `progress` hears the bytes written
    /// and the total; setting `cancel` stops it. Returns how many files
    /// were written under `files/`. Whatever was written stays: extract into
    /// a folder of its own and remove it on an error.
    pub fn extract(&self, root: &Path, progress: &mut dyn FnMut(u64, u64), cancel: &AtomicBool) -> Result<usize, String> {
        let mut partition = self.disc.open_partition_kind(PartitionKind::Data).map_err(|e| format!("The disc has no game partition ({e})."))?;
        let meta = partition.meta().map_err(|e| format!("The game partition cannot be read ({e})."))?;
        let fst = meta.fst().map_err(|e| format!("The disc's file table is damaged ({e})."))?;
        let files: Vec<_> = fst.nodes.iter().skip(1).filter(|n| n.is_file()).collect();
        let sys: [(&str, &[u8]); 5] = [("boot.bin", &meta.raw_boot[..]), ("bi2.bin", &meta.raw_bi2[..]), ("apploader.img", &meta.raw_apploader), ("main.dol", &meta.raw_dol), ("fst.bin", &meta.raw_fst)];
        let extra = [("ticket.bin", &meta.raw_ticket), ("tmd.bin", &meta.raw_tmd), ("cert.bin", &meta.raw_cert_chain), ("h3.bin", &meta.raw_h3_table)];
        let total = files.iter().map(|n| n.length()).sum::<u64>() + sys.iter().map(|(_, b)| b.len() as u64).sum::<u64>();
        let mut done = 0u64;
        progress(done, total);
        let made = |dir: &Path| std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()));
        let write = |path: &Path, bytes: &[u8]| std::fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()));
        made(&root.join("sys"))?;
        for (name, bytes) in sys {
            write(&root.join("sys").join(name), bytes)?;
            done += bytes.len() as u64;
        }
        for (name, bytes) in extra {
            if let Some(bytes) = bytes {
                write(&root.join(name), bytes)?;
            }
        }
        progress(done, total);
        // Directory nodes hold the index just past their last child: a stack
        // of (folder, end) gives each node its folder.
        let mut stack: Vec<(PathBuf, usize)> = vec![(root.join("files"), fst.nodes.len())];
        made(&root.join("files"))?;
        let mut count = 0;
        let mut buf = vec![0u8; 1 << 20];
        for (i, node) in fst.nodes.iter().enumerate().skip(1) {
            while stack.len() > 1 && stack.last().is_some_and(|(_, end)| i >= *end) {
                stack.pop();
            }
            let name = fst.get_name(*node).map_err(|e| format!("The disc's file table is damaged ({e})."))?;
            if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\\', ':']) {
                return Err(format!("The disc's file table names a file \"{name}\", which cannot be written."));
            }
            let path = stack.last().map(|(dir, _)| dir.join(name.as_ref())).unwrap_or_default();
            if node.is_dir() {
                made(&path)?;
                stack.push((path, node.length() as usize));
                continue;
            }
            let mut from = partition.open_file(*node).map_err(|e| format!("{}: {e}", path.display()))?;
            let mut to = BufWriter::new(File::create(&path).map_err(|e| format!("{}: {e}", path.display()))?);
            let mut left = node.length();
            while left > 0 {
                if cancel.load(Ordering::Relaxed) {
                    return Err("cancelled".into());
                }
                let n = (left.min(buf.len() as u64)) as usize;
                from.read_exact(&mut buf[..n]).map_err(|e| format!("{}: {e}", path.display()))?;
                to.write_all(&buf[..n]).map_err(|e| format!("{}: {e}", path.display()))?;
                left -= n as u64;
                done += n as u64;
                progress(done, total);
            }
            to.flush().map_err(|e| format!("{}: {e}", path.display()))?;
            count += 1;
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn images_are_known_by_their_names() {
        for name in ["game.iso", "Game.WBFS", "a b.rvz", "x.wia", "y.gcz", "z.ciso"] {
            assert!(is_image(Path::new(name)), "{name}");
        }
        for name in ["game.zip", "files", "boot.bin", "iso"] {
            assert!(!is_image(Path::new(name)), "{name}");
        }
    }

    #[test]
    fn a_file_that_is_no_disc_is_refused_in_words() {
        let path = std::env::temp_dir().join("disc-image-test-not-a-disc.iso");
        std::fs::write(&path, vec![0u8; 0x10000]).unwrap();
        let e = Image::open(&path).err().unwrap();
        let _ = std::fs::remove_file(&path);
        assert!(e.starts_with("This is not") || e.contains("disc"), "{e}");
    }
}
