#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1203;

pub fn record(count: &u64, bytes: u64) {
    *count = count.saturating_add(bytes);
}
