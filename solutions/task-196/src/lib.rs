// rust-track: handwritten task-196
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 196;

pub fn override_code_8(start: u16, end: u16) -> bool {
    let value = start.abs_diff(end);
    value == 8 || value == 10
}
