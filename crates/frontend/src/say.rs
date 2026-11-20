//! Lines printed while the game runs go through a thread of their own: the
//! window and game threads must never wait on stdout or stderr. A console in
//! selection mode stops every write until the selection ends, and output
//! redirected to a file on a slow disk can take tens to hundreds of
//! milliseconds per line (measured on this PC's laptop hard drive: up to
//! 350 ms), which froze frames and the game at scene loads, where "slow
//! field" lines come.

use std::io::Write;
use std::sync::mpsc::{channel, Sender};
use std::sync::OnceLock;

enum Line {
    Out(String),
    Err(String),
    Flush(Sender<()>),
}

fn sender() -> &'static Sender<Line> {
    static TX: OnceLock<Sender<Line>> = OnceLock::new();
    TX.get_or_init(|| {
        let (tx, rx) = channel::<Line>();
        std::thread::Builder::new()
            .name("say".into())
            .spawn(move || {
                for line in rx {
                    match line {
                        Line::Out(s) => {
                            let mut o = std::io::stdout().lock();
                            let _ = writeln!(o, "{s}");
                            let _ = o.flush();
                        }
                        Line::Err(s) => {
                            let _ = writeln!(std::io::stderr().lock(), "{s}");
                        }
                        Line::Flush(done) => {
                            let _ = done.send(());
                        }
                    }
                }
            })
            .expect("spawn the output thread");
        tx
    })
}

pub fn out(s: String) {
    let _ = sender().send(Line::Out(s));
}

pub fn err(s: String) {
    let _ = sender().send(Line::Err(s));
}

/// Waits until every line sent so far is written (before printing directly
/// again, so the order holds).
pub fn flush() {
    let (tx, rx) = channel();
    if sender().send(Line::Flush(tx)).is_ok() {
        let _ = rx.recv();
    }
}

/// `println!` through the output thread.
macro_rules! say {
    ($($t:tt)*) => { $crate::say::out(format!($($t)*)) };
}

/// `eprintln!` through the output thread.
macro_rules! say_err {
    ($($t:tt)*) => { $crate::say::err(format!($($t)*)) };
}

pub(crate) use {say, say_err};
