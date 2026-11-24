// rust-track: handwritten task-175
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 175;

pub fn service_interval_3(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    value > 0 && value % 3 == 0
}
