//! HTTP GETs through WinHTTP: Windows' own client, with the system's proxy
//! settings and certificates, so the port carries no TLS code.

/// The body of `url`'s answer, `None` for 404. Follows redirects.
#[cfg(not(windows))]
pub fn get(_url: &str) -> Result<Option<String>, String> {
    Err("the update check is not built for this system".into())
}

/// Saves `url`'s answer to `dest`.
#[cfg(not(windows))]
pub fn download(_url: &str, _dest: &std::path::Path, _expected: u64, _progress: &dyn Fn(u64, u64), _cancel: &std::sync::atomic::AtomicBool) -> Result<(), String> {
    Err("updating is not built for this system".into())
}

#[cfg(windows)]
pub use win::{download, get};

#[cfg(windows)]
mod win {
    use std::ffi::c_void;
    use std::io::Write;
    use std::path::Path;
    use std::ptr::{null, null_mut};
    use std::sync::atomic::{AtomicBool, Ordering};
    use windows_sys::Win32::Foundation::GetLastError;
    use windows_sys::Win32::Networking::WinHttp::*;

    /// An answer larger than this is not a release description.
    const MAX_BODY: usize = 4 << 20;
    /// Nor is a download larger than this a release of the port.
    const MAX_DOWNLOAD: u64 = 2 << 30;

    struct Handle(*mut c_void);
    impl Drop for Handle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                // SAFETY: a handle WinHTTP gave out, closed once.
                unsafe { WinHttpCloseHandle(self.0) };
            }
        }
    }

    /// An answer being read. Its handles close request first, then the
    /// connection, then the session (the fields' order).
    struct Answer {
        request: Handle,
        _connect: Handle,
        _session: Handle,
        status: u32,
        /// Content-Length, when the answer gives one.
        length: Option<u64>,
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(Some(0)).collect()
    }

    fn failed(what: &str) -> String {
        // SAFETY: reads the calling thread's last error.
        format!("{what} failed (Windows error {})", unsafe { GetLastError() })
    }

    /// Sends a GET for `url`, asking for `accept`, and waits for the
    /// answer's headers. Redirects are followed.
    fn request(url: &str, accept: &str, receive_ms: i32) -> Result<Answer, String> {
        let url_w = wide(url);
        let mut host = [0u16; 256];
        let mut path = [0u16; 2048];
        let mut parts = URL_COMPONENTS {
            dwStructSize: std::mem::size_of::<URL_COMPONENTS>() as u32,
            lpszHostName: host.as_mut_ptr(),
            dwHostNameLength: host.len() as u32,
            lpszUrlPath: path.as_mut_ptr(),
            dwUrlPathLength: path.len() as u32,
            ..Default::default()
        };
        // SAFETY: every pointer handed to WinHTTP below is to a live local
        // buffer of the stated length, and each handle is closed by `Handle`.
        unsafe {
            if WinHttpCrackUrl(url_w.as_ptr(), 0, 0, &mut parts) == 0 {
                return Err(format!("{url}: not a web address"));
            }
            let session = Handle(WinHttpOpen(wide("nsmbw-port-update").as_ptr(), WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY, null(), null(), 0));
            if session.0.is_null() {
                return Err(failed("WinHttpOpen"));
            }
            WinHttpSetTimeouts(session.0, 5_000, 5_000, 5_000, receive_ms);
            let connect = Handle(WinHttpConnect(session.0, host.as_ptr(), parts.nPort, 0));
            if connect.0.is_null() {
                return Err(failed("connecting"));
            }
            let secure = if parts.nScheme == WINHTTP_INTERNET_SCHEME_HTTPS { WINHTTP_FLAG_SECURE } else { 0 };
            let request = Handle(WinHttpOpenRequest(connect.0, wide("GET").as_ptr(), path.as_ptr(), null(), null(), null(), secure));
            if request.0.is_null() {
                return Err(failed("opening the request"));
            }
            let headers = wide(&format!("Accept: {accept}\r\n"));
            if WinHttpSendRequest(request.0, headers.as_ptr(), u32::MAX, null(), 0, 0, 0) == 0 || WinHttpReceiveResponse(request.0, null_mut()) == 0 {
                return Err(format!("{url}: no answer (Windows error {})", GetLastError()));
            }
            let (mut status, mut size) = (0u32, 4u32);
            if WinHttpQueryHeaders(request.0, WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER, null(), (&mut status as *mut u32).cast(), &mut size, null_mut()) == 0 {
                return Err(failed("reading the answer's status"));
            }
            let (mut length, mut size) = (0u32, 4u32);
            let has_length = WinHttpQueryHeaders(request.0, WINHTTP_QUERY_CONTENT_LENGTH | WINHTTP_QUERY_FLAG_NUMBER, null(), (&mut length as *mut u32).cast(), &mut size, null_mut()) != 0;
            Ok(Answer { request, _connect: connect, _session: session, status, length: has_length.then_some(u64::from(length)) })
        }
    }

    impl Answer {
        /// The next part of the body into `buf`; 0 at its end.
        fn read(&self, buf: &mut [u8]) -> Result<usize, String> {
            let mut read = 0u32;
            // SAFETY: `buf` is live and at least as long as stated.
            if unsafe { WinHttpReadData(self.request.0, buf.as_mut_ptr().cast(), buf.len().min(u32::MAX as usize) as u32, &mut read) } == 0 {
                return Err(failed("reading the answer"));
            }
            Ok(read as usize)
        }
    }

    pub fn get(url: &str) -> Result<Option<String>, String> {
        let answer = request(url, "application/vnd.github+json", 10_000)?;
        if answer.status == 404 {
            return Ok(None);
        }
        if answer.status != 200 {
            return Err(format!("{url}: answered {}", answer.status));
        }
        let mut body = Vec::new();
        let mut buf = vec![0u8; 64 << 10];
        loop {
            let n = answer.read(&mut buf)?;
            if n == 0 {
                break;
            }
            if body.len() + n > MAX_BODY {
                return Err(format!("{url}: the answer is too large"));
            }
            body.extend_from_slice(&buf[..n]);
        }
        String::from_utf8(body).map(Some).map_err(|_| format!("{url}: the answer is not text"))
    }

    /// Saves `url`'s answer to `dest`, calling `progress(done, total)` as
    /// it comes (`total`: the answer's length, else `expected`, else 0).
    /// Stops when `cancel` is set, and at more than `expected` bytes when
    /// that is known. GitHub's downloads redirect to its storage; followed.
    pub fn download(url: &str, dest: &Path, expected: u64, progress: &dyn Fn(u64, u64), cancel: &AtomicBool) -> Result<(), String> {
        let answer = request(url, "application/octet-stream", 30_000)?;
        if answer.status != 200 {
            return Err(format!("the download answered {}", answer.status));
        }
        let total = answer.length.filter(|&n| n > 0).unwrap_or(expected);
        let limit = if expected > 0 { expected } else { MAX_DOWNLOAD };
        if total > limit {
            return Err("the download is larger than the release says".into());
        }
        let mut file = std::fs::File::create(dest).map_err(|e| format!("{}: {e}", dest.display()))?;
        let mut buf = vec![0u8; 256 << 10];
        let mut done = 0u64;
        progress(0, total);
        loop {
            if cancel.load(Ordering::Relaxed) {
                return Err("cancelled".into());
            }
            let n = answer.read(&mut buf)?;
            if n == 0 {
                break;
            }
            done += n as u64;
            if done > limit {
                return Err("the download is larger than the release says".into());
            }
            file.write_all(&buf[..n]).map_err(|e| format!("{}: {e}", dest.display()))?;
            progress(done, total);
        }
        file.flush().map_err(|e| format!("{}: {e}", dest.display()))?;
        if total > 0 && done != total {
            return Err(format!("the download stopped at {done} of {total} bytes"));
        }
        Ok(())
    }
}
