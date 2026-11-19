//! Save states (docs/PORT_PLAN.md, section 12): one walk over a value's
//! fields that either writes them or reads them back in place, so saving and
//! loading cannot disagree about the layout.
//!
//! A type implements [`Snapshot`] by visiting each field that is guest
//! state, in a fixed order. Host-side things (threads, channels, statistics,
//! caches that are rebuilt) are left out on purpose and listed at the impl.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// The walk's direction and its byte stream.
pub struct Snap {
    buf: Vec<u8>,
    pos: usize,
    loading: bool,
    /// A load ran past the end or met a bad length.
    bad: bool,
}

impl Snap {
    pub fn saver() -> Self {
        Self { buf: Vec::with_capacity(1 << 20), pos: 0, loading: false, bad: false }
    }

    pub fn loader(bytes: Vec<u8>) -> Self {
        Self { buf: bytes, pos: 0, loading: true, bad: false }
    }

    pub fn loading(&self) -> bool {
        self.loading
    }

    /// The saved bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    /// After a load: everything was read, and nothing was malformed.
    pub fn finish(&self) -> Result<(), String> {
        if self.bad {
            Err("save state is truncated or malformed".into())
        } else if self.pos != self.buf.len() {
            Err(format!("save state has {} bytes left over", self.buf.len() - self.pos))
        } else {
            Ok(())
        }
    }

    /// Raw bytes of a known length.
    pub fn bytes(&mut self, b: &mut [u8]) {
        if self.loading {
            match self.buf.get(self.pos..self.pos + b.len()) {
                Some(src) => b.copy_from_slice(src),
                None => self.bad = true,
            }
            self.pos += b.len();
        } else {
            self.buf.extend_from_slice(b);
        }
    }

    /// A length: written as is, checked on load against `max`.
    pub fn len(&mut self, n: usize, max: usize) -> usize {
        let mut v = n as u64;
        v.snap(self);
        if self.loading && v as usize > max {
            self.bad = true;
            return 0;
        }
        v as usize
    }

    /// Marks a load as failed (a nested part was malformed).
    pub fn fail(&mut self) {
        self.bad = true;
    }

    /// A section marker: a load fails here if the stream lost its place.
    pub fn tag(&mut self, t: &[u8; 4]) {
        let mut b = *t;
        self.bytes(&mut b);
        if self.loading && &b != t {
            self.bad = true;
        }
    }
}

pub trait Snapshot {
    fn snap(&mut self, s: &mut Snap);
}

/// Visits each listed field: `snap!(s; self.a, self.b.c)`.
#[macro_export]
macro_rules! snap {
    ($s:expr; $($f:expr),* $(,)?) => {
        { $( $crate::snap::Snapshot::snap(&mut $f, $s); )* }
    };
}

macro_rules! numbers {
    ($($t:ty),*) => {$(
        impl Snapshot for $t {
            fn snap(&mut self, s: &mut Snap) {
                let mut b = self.to_le_bytes();
                s.bytes(&mut b);
                *self = <$t>::from_le_bytes(b);
            }
        }
    )*};
}
numbers!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl Snapshot for usize {
    fn snap(&mut self, s: &mut Snap) {
        let mut v = *self as u64;
        v.snap(s);
        *self = v as usize;
    }
}

impl Snapshot for bool {
    fn snap(&mut self, s: &mut Snap) {
        let mut v = *self as u8;
        v.snap(s);
        *self = v != 0;
    }
}

impl<T: Snapshot, const N: usize> Snapshot for [T; N] {
    fn snap(&mut self, s: &mut Snap) {
        for v in self {
            v.snap(s);
        }
    }
}

/// Longest list a load accepts (guards against a corrupt length).
const MAX_LEN: usize = 1 << 28;

impl<T: Snapshot + Default> Snapshot for Vec<T> {
    fn snap(&mut self, s: &mut Snap) {
        let n = s.len(self.len(), MAX_LEN);
        if s.loading() {
            self.clear();
            self.resize_with(n, T::default);
        }
        for v in self {
            v.snap(s);
        }
    }
}

impl<T: Snapshot + Default> Snapshot for VecDeque<T> {
    fn snap(&mut self, s: &mut Snap) {
        let n = s.len(self.len(), MAX_LEN);
        if s.loading() {
            self.clear();
            self.resize_with(n, T::default);
        }
        for v in self {
            v.snap(s);
        }
    }
}

impl<T: Snapshot + Default> Snapshot for Option<T> {
    fn snap(&mut self, s: &mut Snap) {
        let mut some = self.is_some();
        some.snap(s);
        if s.loading() {
            *self = if some { Some(T::default()) } else { None };
        }
        if let Some(v) = self {
            v.snap(s);
        }
    }
}

impl<T: Snapshot + Default> Snapshot for Box<T> {
    fn snap(&mut self, s: &mut Snap) {
        (**self).snap(s);
    }
}

impl<A: Snapshot, B: Snapshot> Snapshot for (A, B) {
    fn snap(&mut self, s: &mut Snap) {
        self.0.snap(s);
        self.1.snap(s);
    }
}

impl<A: Snapshot, B: Snapshot, C: Snapshot> Snapshot for (A, B, C) {
    fn snap(&mut self, s: &mut Snap) {
        self.0.snap(s);
        self.1.snap(s);
        self.2.snap(s);
    }
}

impl Snapshot for String {
    fn snap(&mut self, s: &mut Snap) {
        let mut b = std::mem::take(self).into_bytes();
        b.snap(s);
        *self = String::from_utf8(b).unwrap_or_default();
    }
}

impl Snapshot for std::path::PathBuf {
    fn snap(&mut self, s: &mut Snap) {
        let mut t = self.to_string_lossy().into_owned();
        t.snap(s);
        *self = t.into();
    }
}

/// Saved sorted by key, so equal maps give equal bytes.
impl<K: Snapshot + Default + Ord + Hash + Clone, V: Snapshot + Default> Snapshot for HashMap<K, V> {
    fn snap(&mut self, s: &mut Snap) {
        let n = s.len(self.len(), MAX_LEN);
        if s.loading() {
            self.clear();
            for _ in 0..n {
                let (mut k, mut v) = (K::default(), V::default());
                k.snap(s);
                v.snap(s);
                self.insert(k, v);
            }
        } else {
            let mut keys: Vec<K> = self.keys().cloned().collect();
            keys.sort();
            for mut k in keys {
                let mut v = self.remove(&k).unwrap();
                k.snap(s);
                v.snap(s);
                self.insert(k, v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, PartialEq, Debug)]
    struct T {
        a: u32,
        b: Vec<(u8, f64)>,
        c: Option<String>,
        d: HashMap<u32, bool>,
        e: [u16; 3],
    }

    impl Snapshot for T {
        fn snap(&mut self, s: &mut Snap) {
            snap!(s; self.a, self.b, self.c, self.d, self.e);
        }
    }

    #[test]
    fn round_trip() {
        let mut t = T { a: 7, b: vec![(1, 2.5), (3, -0.0)], c: Some("x".into()), d: [(5, true), (1, false)].into(), e: [1, 2, 3] };
        let mut s = Snap::saver();
        t.snap(&mut s);
        let bytes = s.into_bytes();
        let mut u = T::default();
        let mut l = Snap::loader(bytes.clone());
        u.snap(&mut l);
        l.finish().unwrap();
        assert_eq!(t, u);
        // Saving again gives the same bytes (maps are sorted).
        let mut s2 = Snap::saver();
        u.snap(&mut s2);
        assert_eq!(s2.into_bytes(), bytes);
    }

    #[test]
    fn truncated_is_an_error() {
        let mut t = T { a: 1, ..Default::default() };
        let mut s = Snap::saver();
        t.snap(&mut s);
        let mut bytes = s.into_bytes();
        bytes.truncate(bytes.len() - 1);
        let mut l = Snap::loader(bytes);
        T::default().snap(&mut l);
        assert!(l.finish().is_err());
    }
}
