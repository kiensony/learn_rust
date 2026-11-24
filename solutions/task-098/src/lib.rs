// rust-track: handwritten task-098
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 98;

pub fn two_counters_8(start: u16, end: u16) -> (u16, u16) {
    let value = start.abs_diff(end);
    (value / 8, value % 8)
}
