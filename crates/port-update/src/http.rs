//! One HTTP GET through WinHTTP: Windows' own client, with the system's
//! proxy settings and certificates, so the port carries no TLS code.

/// The body of `url`'s answer, `None` for 404. Follows redirects.
#[cfg(not(windows))]
pub fn get(_url: &str) -> Result<Option<String>, String> {
    Err("the update check is not built for this system".into())
}

#[cfg(windows)]
pub fn get(url: &str) -> Result<Option<String>, String> {
    use std::ffi::c_void;
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Foundation::GetLastError;
    use windows_sys::Win32::Networking::WinHttp::*;

    /// An answer larger than this is not a release description.
    const MAX_BODY: usize = 4 << 20;

    struct Handle(*mut c_void);
    impl Drop for Handle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { WinHttpCloseHandle(self.0) };
            }
        }
    }
    let wide = |s: &str| -> Vec<u16> { s.encode_utf16().chain(Some(0)).collect() };
    let failed = |what: &str| format!("{what} failed (Windows error {})", unsafe { GetLastError() });

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
        WinHttpSetTimeouts(session.0, 5_000, 5_000, 5_000, 10_000);
        let connect = Handle(WinHttpConnect(session.0, host.as_ptr(), parts.nPort, 0));
        if connect.0.is_null() {
            return Err(failed("connecting"));
        }
        let secure = if parts.nScheme == WINHTTP_INTERNET_SCHEME_HTTPS { WINHTTP_FLAG_SECURE } else { 0 };
        let request = Handle(WinHttpOpenRequest(connect.0, wide("GET").as_ptr(), path.as_ptr(), null(), null(), null(), secure));
        if request.0.is_null() {
            return Err(failed("opening the request"));
        }
        let headers = wide("Accept: application/vnd.github+json\r\n");
        if WinHttpSendRequest(request.0, headers.as_ptr(), u32::MAX, null(), 0, 0, 0) == 0 || WinHttpReceiveResponse(request.0, null_mut()) == 0 {
            return Err(format!("{url}: no answer (Windows error {})", GetLastError()));
        }
        let (mut status, mut size) = (0u32, 4u32);
        if WinHttpQueryHeaders(request.0, WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER, null(), (&mut status as *mut u32).cast(), &mut size, null_mut()) == 0 {
            return Err(failed("reading the answer's status"));
        }
        if status == 404 {
            return Ok(None);
        }
        if status != 200 {
            return Err(format!("{url}: answered {status}"));
        }
        let mut body = Vec::new();
        loop {
            let mut available = 0u32;
            if WinHttpQueryDataAvailable(request.0, &mut available) == 0 {
                return Err(failed("reading the answer"));
            }
            if available == 0 {
                break;
            }
            let at = body.len();
            if at + available as usize > MAX_BODY {
                return Err(format!("{url}: the answer is too large"));
            }
            body.resize(at + available as usize, 0u8);
            let mut read = 0u32;
            if WinHttpReadData(request.0, body[at..].as_mut_ptr().cast(), available, &mut read) == 0 {
                return Err(failed("reading the answer"));
            }
            body.truncate(at + read as usize);
            if read == 0 {
                break;
            }
        }
        String::from_utf8(body).map(Some).map_err(|_| format!("{url}: the answer is not text"))
    }
}
