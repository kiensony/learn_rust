// rust-track: handwritten task-088
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 88;

pub fn two_counters_5(issued: u16, returned: u16) -> (u16, u16) {
    let value = issued.saturating_sub(returned);
    (value / 5, value % 5)
}
