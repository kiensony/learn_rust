#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2003;

use std::future::Future;
pub fn reading(f: impl Future<Output = u16>) -> u16 {
    f.await
}
