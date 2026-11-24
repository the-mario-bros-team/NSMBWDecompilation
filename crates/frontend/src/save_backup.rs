//! Save backups: each time the game starts, its save file
//! (`saves/nand/title/00010000/<title>/data/wiimj2d.sav`) is copied into
//! `SaveBackups`, as `wiimj2d-<title>-<UTC date and time>.sav`, and only
//! the newest [`KEEP`] of each title are kept. A save the same as the
//! newest backup is not copied again, so starting the game without playing
//! does not push older backups out. The Android app does the same in its
//! files.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DIR: &str = "SaveBackups";
pub const KEEP: usize = 10;
const SAVE: &str = "wiimj2d.sav";

/// Backs up every title's save under `nand` into `backups`. Returns what
/// was written, or why not.
pub fn backup(nand: &Path, backups: &Path, now: SystemTime) -> Vec<Result<PathBuf, String>> {
    let titles = nand.join("title").join("00010000");
    let Ok(entries) = std::fs::read_dir(&titles) else { return Vec::new() };
    let mut out = Vec::new();
    for e in entries.flatten() {
        let save = e.path().join("data").join(SAVE);
        if !save.is_file() {
            continue;
        }
        let title = title_name(&e.file_name().to_string_lossy());
        match backup_one(&save, backups, &title, now) {
            Ok(Some(p)) => out.push(Ok(p)),
            Ok(None) => {}
            Err(e) => out.push(Err(format!("{}: {e}", save.display()))),
        }
    }
    out
}

/// The title folder's name as text (534d4e45 is "SMNE"), or as it is.
fn title_name(hex: &str) -> String {
    let bytes: Option<Vec<u8>> = (0..hex.len()).step_by(2).map(|i| hex.get(i..i + 2).and_then(|b| u8::from_str_radix(b, 16).ok())).collect();
    match bytes {
        Some(b) if hex.len() == 8 && b.iter().all(u8::is_ascii_alphanumeric) => String::from_utf8(b).unwrap_or_else(|_| hex.into()),
        _ => hex.into(),
    }
}

fn backup_one(save: &Path, backups: &Path, title: &str, now: SystemTime) -> std::io::Result<Option<PathBuf>> {
    let data = std::fs::read(save)?;
    std::fs::create_dir_all(backups)?;
    let prefix = format!("wiimj2d-{title}-");
    let mut old: Vec<PathBuf> = std::fs::read_dir(backups)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with(&prefix) && n.ends_with(".sav")))
        .collect();
    // The names sort by time.
    old.sort();
    if old.last().is_some_and(|p| std::fs::read(p).is_ok_and(|d| d == data)) {
        return Ok(None);
    }
    let path = backups.join(format!("{prefix}{}.sav", stamp(now)));
    if !path.exists() {
        // Whole or not at all.
        let part = path.with_extension("part");
        std::fs::write(&part, &data)?;
        std::fs::rename(&part, &path)?;
        old.push(path.clone());
    }
    let extra = old.len().saturating_sub(KEEP);
    for p in &old[..extra] {
        std::fs::remove_file(p)?;
    }
    Ok(Some(path))
}

/// `YYYY-MM-DD_HH-MM-SS` in UTC.
fn stamp(t: SystemTime) -> String {
    let secs = t.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let (days, rest) = ((secs / 86_400) as i64, secs % 86_400);
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}_{:02}-{:02}-{:02}", rest / 3600, rest / 60 % 60, rest % 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn keeps_the_newest_ten() {
        let dir = std::env::temp_dir().join(format!("nsmbw-save-backups-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let data = dir.join("nand/title/00010000/534d4e45/data");
        std::fs::create_dir_all(&data).unwrap();
        let backups = dir.join(DIR);
        let start = UNIX_EPOCH + Duration::from_secs(1_795_467_909); // 2026-11-23 21:05:09 UTC
        assert_eq!(stamp(start), "2026-11-23_21-05-09");
        assert_eq!(stamp(UNIX_EPOCH + Duration::from_secs(951_782_400)), "2000-02-29_00-00-00");
        for i in 0..12u8 {
            std::fs::write(data.join(SAVE), [i; 16]).unwrap();
            let made = backup(&dir.join("nand"), &backups, start + Duration::from_secs(u64::from(i) * 60));
            assert!(matches!(made.as_slice(), [Ok(_)]), "{made:?}");
            // The same save again: no new backup.
            assert!(backup(&dir.join("nand"), &backups, start + Duration::from_secs(u64::from(i) * 60 + 30)).is_empty());
        }
        let mut names: Vec<String> = std::fs::read_dir(&backups).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        names.sort();
        assert_eq!(names.len(), KEEP);
        assert_eq!(names[0], "wiimj2d-SMNE-2026-11-23_21-07-09.sav");
        assert_eq!(std::fs::read(backups.join(&names[9])).unwrap(), [11; 16]);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
