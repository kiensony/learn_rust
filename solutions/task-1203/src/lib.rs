// rust-track: handwritten task-1203
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1203;

pub fn record(count: &mut u64, bytes: u64) {
    *count = count.saturating_add(bytes);
}
