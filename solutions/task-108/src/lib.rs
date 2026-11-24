// rust-track: handwritten task-108
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 108;

pub fn two_counters_10(requested: u16, available: u16) -> (u16, u16) {
    let value = requested.min(available);
    (value / 10, value % 10)
}
