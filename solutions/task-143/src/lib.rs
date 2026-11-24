// rust-track: handwritten task-143
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 143;

pub fn round_down_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value) / 8 * 8
}
