#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub trait Keyed {
    type Key;
    fn key(&self) -> &Self::Key;
    fn secondary_key(&self) -> &Self::Key;
}
pub struct Record<T>(pub T, pub T);
impl<T> Keyed for Record<T> {
    type Key = T;
    fn key(&self) -> &T {
        &self.0
    }
    fn secondary_key(&self) -> &T {
        &self.1
    }
}

pub fn key_of<S: Keyed>(source: &S) -> &S::Key {
    source.secondary_key()
}
