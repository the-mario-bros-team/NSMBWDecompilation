//! The port's update check, shared by the launcher and the game so that
//! finding an update never depends on the launcher being shown (it can be
//! skipped).
//!
//! It asks a release source for the newest release and compares its tag with
//! [`VERSION`]. The source is GitHub's "latest release" answer
//! (`https://api.github.com/repos/<owner>/<repo>/releases/latest`), or any
//! address or file that holds the same JSON: `tag_name`, `name`, `body`,
//! `html_url`, and `assets` with `name`, `browser_download_url` and `size`.
//! Nothing is downloaded or installed here.
//!
//! ```no_run
//! let folder = port_update::install_dir();
//! let config = port_update::Config::load(&folder);
//! if config.enabled {
//!     // Off the main thread: the answer can take seconds.
//!     std::thread::spawn(move || match port_update::check(&config) {
//!         Ok(port_update::Status::Available(release)) => println!("{} is out", release.version),
//!         Ok(port_update::Status::UpToDate) => {}
//!         Err(e) => eprintln!("update check: {e}"),
//!     });
//! }
//! ```

use std::path::{Path, PathBuf};

mod http;

/// This build's version, as a release's tag names it (without the `v`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Where releases are looked for unless `update.toml` says otherwise: the
/// project's own repository.
pub const DEFAULT_SOURCE: &str = "https://api.github.com/repos/the-mario-bros-team/NSMBWDecompilation/releases/latest";

/// The settings file, next to the programs.
pub const FILE: &str = "update.toml";

/// An environment variable that replaces the release source.
pub const SOURCE_ENV: &str = "NSMBW_UPDATE_SOURCE";

/// A release's download for this system ends with this.
pub const ASSET_SUFFIX: &str = "-windows-x64.zip";

/// The folder the running program is in: where `update.toml` is kept, so the
/// launcher and the game read the same one.
pub fn install_dir() -> PathBuf {
    std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| PathBuf::from("."))
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// Look for updates at start.
    pub enabled: bool,
    /// A web address or a file holding the newest release's JSON.
    pub source: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { enabled: true, source: DEFAULT_SOURCE.into() }
    }
}

impl Config {
    /// `update.toml` in `folder`; the defaults for anything missing. The
    /// environment variable [`SOURCE_ENV`], when set, is the source whatever
    /// the file says (for testing against a stand-in).
    pub fn load(folder: &Path) -> Self {
        let mut c = Self::default();
        if let Some(t) = std::fs::read_to_string(folder.join(FILE)).ok().and_then(|text| text.parse::<toml::Table>().ok()) {
            c.enabled = t.get("check").and_then(toml::Value::as_bool).unwrap_or(c.enabled);
            if let Some(s) = t.get("source").and_then(toml::Value::as_str).filter(|s| !s.trim().is_empty()) {
                c.source = s.trim().into();
            }
        }
        if let Some(s) = std::env::var(SOURCE_ENV).ok().filter(|s| !s.trim().is_empty()) {
            c.source = s.trim().into();
        }
        c
    }

    pub fn save(&self, folder: &Path) -> std::io::Result<()> {
        let text = format!(
            "# Updates for the NSMBW port (the launcher and the game both read this).\n\
             # check: look for a newer release at start.\n\
             check = {}\n\
             # source: where the newest release is described: GitHub's \"latest release\"\n\
             # address, or a file with the same JSON.\n\
             source = {}\n",
            self.enabled,
            toml::Value::String(self.source.clone())
        );
        std::fs::write(folder.join(FILE), text)
    }
}

/// A release newer than this build.
#[derive(Clone, Debug, PartialEq)]
pub struct Release {
    /// The tag without its `v`: "0.2.0".
    pub version: String,
    /// The release's title, or its version when it has none.
    pub name: String,
    /// The release notes, as written (Markdown).
    pub notes: String,
    /// The release's web page; empty unless it is a web address.
    pub page: String,
    /// The download for this system, when the release has one.
    pub download: Option<Download>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Download {
    pub name: String,
    pub url: String,
    pub size: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    /// The newest release is this build or older, or nothing is published yet.
    UpToDate,
    Available(Release),
}

/// Asks `config.source` for the newest release and compares it with
/// [`VERSION`]. Blocks until the answer or a timeout (about 15 seconds).
pub fn check(config: &Config) -> Result<Status, String> {
    check_from(&config.source, VERSION)
}

/// As [`check`], against a given source and version.
pub fn check_from(source: &str, current: &str) -> Result<Status, String> {
    let Some(json) = fetch(source)? else { return Ok(Status::UpToDate) };
    let release = parse_release(&json)?;
    Ok(if is_newer(&release.version, current) { Status::Available(release) } else { Status::UpToDate })
}

/// True for an `https://` address, or an `http://` one on this computer (a
/// stand-in for testing). Only such an address may be opened in the browser.
pub fn is_web_url(url: &str) -> bool {
    let local = |rest: &str| {
        let host = rest.split(['/', ':']).next().unwrap_or("");
        host == "127.0.0.1" || host.eq_ignore_ascii_case("localhost")
    };
    let plain = |u: &str| !u.chars().any(|c| c.is_whitespace() || c.is_control() || c == '"');
    match url.split_once("://") {
        Some((scheme, rest)) if scheme.eq_ignore_ascii_case("https") => !rest.is_empty() && plain(url),
        Some((scheme, rest)) if scheme.eq_ignore_ascii_case("http") => local(rest) && plain(url),
        _ => false,
    }
}

/// The source's answer; `None` when it has no release (GitHub answers 404
/// until the first one is published).
fn fetch(source: &str) -> Result<Option<String>, String> {
    let source = source.trim();
    if source.is_empty() {
        return Err("no release source is set".into());
    }
    let lower = source.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        return http::get(source);
    }
    let path = source.strip_prefix("file:///").or_else(|| source.strip_prefix("file://")).unwrap_or(source);
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("{path}: {e}")),
    }
}

fn parse_release(json: &str) -> Result<Release, String> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("the release source's answer is not JSON: {e}"))?;
    let text = |v: &serde_json::Value, key: &str| v.get(key).and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let tag = text(&v, "tag_name");
    let version = tag.strip_prefix(['v', 'V']).unwrap_or(&tag).to_string();
    if version.is_empty() {
        return Err("the release source's answer has no tag_name".into());
    }
    let name = match text(&v, "name") {
        n if n.is_empty() => version.clone(),
        n => n,
    };
    let page = Some(text(&v, "html_url")).filter(|p| is_web_url(p)).unwrap_or_default();
    let download = v.get("assets").and_then(|a| a.as_array()).and_then(|assets| {
        assets.iter().find_map(|a| {
            let (name, url) = (text(a, "name"), text(a, "browser_download_url"));
            (name.to_ascii_lowercase().ends_with(ASSET_SUFFIX) && is_web_url(&url)).then(|| Download { name, url, size: a.get("size").and_then(|s| s.as_u64()).unwrap_or(0) })
        })
    });
    Ok(Release { version, name, notes: text(&v, "body"), page, download })
}

/// True when version `remote` ("0.2.3") is later than `local`. Parts are
/// compared as numbers; a missing part counts as 0, and anything after a
/// part's digits ("3-beta") is ignored.
pub fn is_newer(remote: &str, local: &str) -> bool {
    let parts = |v: &str| -> Vec<u64> {
        let v = v.trim().strip_prefix(['v', 'V']).unwrap_or(v.trim());
        v.split('.').map(|p| p.chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap_or(0)).collect()
    };
    let (r, l) = (parts(remote), parts(local));
    for i in 0..r.len().max(l.len()) {
        let (a, b) = (r.get(i).copied().unwrap_or(0), l.get(i).copied().unwrap_or(0));
        if a != b {
            return a > b;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const RELEASE: &str = r#"{
        "tag_name": "v0.2.0",
        "name": "NSMBW port 0.2.0",
        "body": "- Faster\n- Launcher",
        "html_url": "https://github.com/example/port/releases/tag/v0.2.0",
        "assets": [
            {"name": "nsmbw-port-0.2.0-linux-x86_64.tar.gz", "browser_download_url": "https://github.com/example/a.tar.gz", "size": 5},
            {"name": "nsmbw-port-0.2.0-windows-x64.zip", "browser_download_url": "https://github.com/example/a.zip", "size": 1234}
        ]
    }"#;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("port-update-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn versions_compare_as_numbers() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("v0.10.0", "0.9.9"));
        assert!(is_newer("1.0", "0.9.9"));
        assert!(is_newer("0.1.0.1", "0.1.0"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.1", "0.1.0"));
        assert!(!is_newer("0.0.9", "0.1.0"));
        assert!(!is_newer("0.1.0-beta", "0.1.0"));
        assert!(!is_newer("", "0.1.0"));
    }

    #[test]
    fn a_release_is_read() {
        let r = parse_release(RELEASE).unwrap();
        assert_eq!(r.version, "0.2.0");
        assert_eq!(r.name, "NSMBW port 0.2.0");
        assert_eq!(r.notes, "- Faster\n- Launcher");
        assert_eq!(r.page, "https://github.com/example/port/releases/tag/v0.2.0");
        assert_eq!(r.download, Some(Download { name: "nsmbw-port-0.2.0-windows-x64.zip".into(), url: "https://github.com/example/a.zip".into(), size: 1234 }));
        assert!(parse_release("{}").is_err());
        assert!(parse_release("<html>").is_err());
    }

    #[test]
    fn only_web_addresses_are_kept() {
        assert!(is_web_url("https://github.com/x/y/releases"));
        assert!(is_web_url("http://127.0.0.1:8123/page"));
        assert!(is_web_url("http://localhost/page"));
        assert!(!is_web_url("http://example.com/page"));
        assert!(!is_web_url("file:///C:/Windows/System32/calc.exe"));
        assert!(!is_web_url("C:\\Windows\\System32\\calc.exe"));
        assert!(!is_web_url("https://a b"));
        let r = parse_release(r#"{"tag_name": "1.0", "html_url": "C:\\evil.exe"}"#).unwrap();
        assert_eq!(r.page, "");
        assert_eq!(r.name, "1.0");
    }

    #[test]
    fn a_file_stands_in_for_the_release_source() {
        let dir = temp("file");
        let file = dir.join("latest.json");
        std::fs::write(&file, RELEASE).unwrap();
        let source = file.to_string_lossy().into_owned();
        assert!(matches!(check_from(&source, "0.1.0"), Ok(Status::Available(r)) if r.version == "0.2.0"));
        assert_eq!(check_from(&source, "0.2.0"), Ok(Status::UpToDate));
        assert_eq!(check_from(&source, "0.3.0"), Ok(Status::UpToDate));
        // No release published yet.
        assert_eq!(check_from(&dir.join("missing.json").to_string_lossy(), "0.1.0"), Ok(Status::UpToDate));
        assert!(check_from("", "0.1.0").is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn settings_round_trip() {
        let dir = temp("config");
        assert_eq!(Config::load(&dir), Config::default());
        let c = Config { enabled: false, source: r"C:\stand-in\latest.json".into() };
        c.save(&dir).unwrap();
        assert_eq!(Config::load(&dir), c);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// A one-request web server on this computer answering as GitHub does.
    #[cfg(windows)]
    fn serve(status: &'static str, body: &'static str) -> String {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut s, _) = listener.accept().unwrap();
            let mut request = [0u8; 4096];
            let _ = s.read(&mut request);
            let _ = write!(s, "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
        });
        format!("http://{addr}/repos/example/port/releases/latest")
    }

    #[cfg(windows)]
    #[test]
    fn a_web_source_is_asked() {
        assert!(matches!(check_from(&serve("200 OK", RELEASE), "0.1.0"), Ok(Status::Available(r)) if r.version == "0.2.0" && r.download.is_some()));
        assert_eq!(check_from(&serve("200 OK", RELEASE), "0.2.0"), Ok(Status::UpToDate));
        // GitHub's answer while a repository has no release.
        assert_eq!(check_from(&serve("404 Not Found", r#"{"message": "Not Found"}"#), "0.1.0"), Ok(Status::UpToDate));
        assert!(check_from(&serve("500 Internal Server Error", "{}"), "0.1.0").is_err());
        // Nothing listens there.
        let free = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap();
        assert!(check_from(&format!("http://{free}/latest"), "0.1.0").is_err());
    }
}
