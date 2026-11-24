// rust-track: handwritten task-195
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 195;

pub fn service_interval_8(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    value > 0 && value % 8 == 0
}
