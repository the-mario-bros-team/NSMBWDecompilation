//! Looking at a folder the player picked: is it an extracted Wii disc, which
//! game is it (the ID and revision in `sys\boot.bin`), and does the selected
//! version accept it. The full validation is the game's own
//! (`nsmbw-port --check`), run here and read back.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::versions::Version;

/// What `sys\boot.bin` says.
#[derive(Clone, Debug, PartialEq)]
pub struct Found {
    /// The folder that holds `sys` and `files`.
    pub root: PathBuf,
    /// The six-character game ID: "SMNE01".
    pub id: String,
    pub revision: u8,
}

impl Found {
    pub fn region(&self) -> &'static str {
        match self.id.as_bytes().get(3) {
            Some(b'E') => "USA",
            Some(b'P') => "Europe",
            Some(b'J') => "Japan",
            Some(b'K') => "Korea",
            Some(b'W') => "Taiwan",
            Some(b'C') => "China",
            _ => "unknown region",
        }
    }

    /// "SMNE01, USA, revision 1"
    pub fn describe(&self) -> String {
        format!("{}, {}, revision {}", self.id, self.region(), self.revision)
    }
}

fn is_extracted(p: &Path) -> bool {
    p.join("files").is_dir() && p.join("sys").join("boot.bin").is_file()
}

/// The extracted disc at `picked`: the folder itself, or one inside it
/// (Dolphin's "Extract Entire Disc" puts the game in `DATA`).
pub fn find_root(picked: &Path) -> Option<PathBuf> {
    if is_extracted(picked) {
        return Some(picked.to_path_buf());
    }
    let mut inside: Vec<PathBuf> = std::fs::read_dir(picked).ok()?.flatten().map(|e| e.path()).filter(|p| is_extracted(p)).collect();
    inside.sort();
    inside.into_iter().next()
}

/// Reads the game ID and revision of the extracted disc at `picked`.
pub fn identify(picked: &Path) -> Result<Found, String> {
    let root = find_root(picked).ok_or("There is no extracted game here: the folder needs the sys and files folders that Dolphin's \"Extract Entire Disc\" makes.")?;
    let boot = std::fs::read(root.join("sys").join("boot.bin")).map_err(|e| format!("The disc header (sys\\boot.bin) could not be read: {e}"))?;
    if boot.len() < 0x20 || boot[0x18..0x1C] != [0x5D, 0x1C, 0x9E, 0xA3] {
        return Err("sys\\boot.bin is not a Wii disc header.".into());
    }
    if !boot[..6].iter().all(u8::is_ascii_alphanumeric) {
        return Err("sys\\boot.bin has no game ID.".into());
    }
    Ok(Found { root, id: String::from_utf8_lossy(&boot[..6]).into_owned(), revision: boot[7] })
}

/// Whether `found` is one of `v`'s discs, with the files it needs.
fn fits(found: &Found, v: &Version) -> bool {
    v.discs.iter().any(|d| d.id == found.id && d.revision.is_none_or(|r| r == found.revision))
        && v.with.iter().all(|p| found.root.join(p).exists())
        && !v.without.iter().any(|p| found.root.join(p).exists())
}

/// `Ok` when `found` is a game `version` accepts; else why not, in words for
/// the player.
pub fn judge(found: &Found, version: &Version, all: &[Version]) -> Result<(), String> {
    if fits(found, version) {
        return Ok(());
    }
    if let Some(other) = all.iter().find(|o| o.id != version.id && fits(found, o)) {
        return Err(format!("That is {} ({}). Pick it under Switch Version, then set this folder there.", other.title, found.id));
    }
    let accepted: Vec<String> = version
        .discs
        .iter()
        .map(|d| match d.revision {
            Some(r) => format!("{} revision {r}", d.id),
            None => d.id.clone(),
        })
        .collect();
    let accepted = accepted.join(" or ");
    if version.discs.iter().any(|d| d.id == found.id) {
        if let Some(missing) = version.with.iter().find(|p| !found.root.join(p).exists()) {
            return Err(format!("This is {}, but {} is missing from it, so it is not {}.", found.describe(), missing.replace('/', "\\"), version.title));
        }
        if let Some(extra) = version.without.iter().find(|p| found.root.join(p).exists()) {
            return Err(format!("This is {}, but it holds {}: it is a modified copy, not {}.", found.describe(), extra.replace('/', "\\"), version.title));
        }
        return Err(format!("This is {}. {} needs {accepted} (Dolphin shows the revision under Properties > Info).", found.describe(), version.title));
    }
    Err(format!("This is {}, not {}. It needs {accepted}.", found.describe(), version.title))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Level {
    Ok,
    Fail,
}

/// What `nsmbw-port --check` said.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Report {
    /// One line per check that passed or failed.
    pub lines: Vec<(Level, String)>,
    pub passed: bool,
    /// Why the check could not be run at all.
    pub error: Option<String>,
}

fn parse_report(output: &str, success: bool) -> Report {
    let mut lines = Vec::new();
    for line in output.lines() {
        let line = line.trim();
        let (level, rest) = match (line.strip_prefix("OK"), line.strip_prefix("FAIL")) {
            (Some(rest), _) => (Level::Ok, rest),
            (_, Some(rest)) => (Level::Fail, rest),
            _ => continue,
        };
        if rest.starts_with(' ') {
            lines.push((level, rest.trim().to_string()));
        }
    }
    let failed = lines.iter().any(|l| l.0 == Level::Fail);
    Report { passed: success && !failed, error: (!success && !failed).then(|| "The game's check stopped without saying why.".to_string()), lines }
}

/// Runs the game's own validation of `game` (`exe --check --game <game>`) in
/// `dir` and reads its report. Takes a second or two.
pub fn full_check(exe: &Path, dir: &Path, game: &Path) -> Report {
    if !exe.is_file() {
        return Report { error: Some(format!("{} is missing, so the folder could not be checked in full.", exe.display())), ..Report::default() };
    }
    if let Err(e) = std::fs::create_dir_all(dir) {
        return Report { error: Some(format!("{}: {e}", dir.display())), ..Report::default() };
    }
    let mut command = Command::new(exe);
    command.arg("--check").arg("--game").arg(game).current_dir(dir);
    crate::launch::no_console(&mut command);
    match command.output() {
        Ok(out) => parse_report(&String::from_utf8_lossy(&out.stdout), out.status.success()),
        Err(e) => Report { error: Some(format!("{} could not be started: {e}", exe.display())), ..Report::default() },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_disc(root: &Path, id: &[u8; 6], revision: u8) {
        std::fs::create_dir_all(root.join("sys")).unwrap();
        std::fs::create_dir_all(root.join("files")).unwrap();
        let mut boot = vec![0u8; 0x440];
        boot[..6].copy_from_slice(id);
        boot[7] = revision;
        boot[0x18..0x1C].copy_from_slice(&[0x5D, 0x1C, 0x9E, 0xA3]);
        std::fs::write(root.join("sys").join("boot.bin"), boot).unwrap();
    }

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nsmbw-launcher-disc-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn discs_are_identified_and_judged() {
        let dir = temp("judge");
        let all = crate::versions::load(Path::new("no such folder")).0;
        let (original, newer) = (&all[0], &all[1]);

        assert!(identify(&dir).unwrap_err().contains("no extracted game"));

        // The supported disc, found inside the folder Dolphin makes.
        fake_disc(&dir.join("usa").join("DATA"), b"SMNE01", 1);
        let usa = identify(&dir.join("usa")).unwrap();
        assert_eq!((usa.id.as_str(), usa.revision, usa.region()), ("SMNE01", 1, "USA"));
        assert_eq!(usa.root, dir.join("usa").join("DATA"));
        assert_eq!(judge(&usa, original, &all), Ok(()));
        assert!(judge(&usa, newer, &all).unwrap_err().contains("That is New Super Mario Bros. Wii"));

        // Another revision, another region, another game.
        fake_disc(&dir.join("rev2"), b"SMNE01", 2);
        let e = judge(&identify(&dir.join("rev2")).unwrap(), original, &all).unwrap_err();
        assert!(e.contains("revision 2") && e.contains("needs SMNE01 revision 1"), "{e}");
        fake_disc(&dir.join("pal"), b"SMNP01", 1);
        let e = judge(&identify(&dir.join("pal")).unwrap(), original, &all).unwrap_err();
        assert!(e.contains("SMNP01, Europe, revision 1, not New Super Mario Bros. Wii"), "{e}");
        fake_disc(&dir.join("kart"), b"RMCE01", 0);
        assert!(judge(&identify(&dir.join("kart")).unwrap(), original, &all).is_err());

        // Newer: its own ID and its NewerRes folder.
        fake_disc(&dir.join("newer"), b"SMNE03", 0);
        let found = identify(&dir.join("newer")).unwrap();
        assert!(judge(&found, newer, &all).unwrap_err().contains("NewerRes is missing"));
        std::fs::create_dir_all(dir.join("newer").join("files").join("NewerRes")).unwrap();
        assert_eq!(judge(&found, newer, &all), Ok(()));
        assert!(judge(&found, original, &all).unwrap_err().contains("That is Newer Super Mario Bros. Wii"));

        // The original's ID with Newer's files in it.
        std::fs::create_dir_all(dir.join("usa").join("DATA").join("files").join("NewerRes")).unwrap();
        assert!(judge(&usa, original, &all).unwrap_err().contains("modified copy"));

        // Not a disc header.
        std::fs::write(dir.join("kart").join("sys").join("boot.bin"), [0u8; 0x440]).unwrap();
        assert!(identify(&dir.join("kart")).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_games_report_is_read() {
        let good = "game folder: E:\\game\n  OK    game id: SMNE01 rev 1\n  OK    sys/main.dol: 18 sections\n  SKIP  code vs analysed image: not found\n  OK    files/ vs fst.bin: 2933 files\nresult: OK\n";
        let r = parse_report(good, true);
        assert!(r.passed && r.error.is_none());
        assert_eq!(r.lines.len(), 3);
        assert_eq!(r.lines[0], (Level::Ok, "game id: SMNE01 rev 1".to_string()));
        let bad = "  FAIL  game id: SMNP01 rev 1; the analysed build is SMNE01 rev 1\n        x is missing\n  FAIL  files/ vs fst.bin: 3 of 2933 files missing or the wrong size\nresult: 2 check(s) failed\n";
        let r = parse_report(bad, false);
        assert!(!r.passed && r.error.is_none());
        assert_eq!(r.lines.iter().filter(|l| l.0 == Level::Fail).count(), 2);
        assert!(parse_report("", false).error.is_some());
        let missing = full_check(Path::new("no such program.exe"), Path::new("."), Path::new("."));
        assert!(!missing.passed && missing.error.unwrap().contains("is missing"));
    }
}
