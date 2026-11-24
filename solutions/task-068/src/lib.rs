// rust-track: handwritten task-068
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 68;

pub fn two_counters_2(value: u16) -> (u16, u16) {
    (value / 2, value % 2)
}
