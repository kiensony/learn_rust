// rust-track: handwritten task-078
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 78;

pub fn two_counters_3(first: u16, second: u16) -> (u16, u16) {
    let value = first.saturating_add(second);
    (value / 3, value % 3)
}
