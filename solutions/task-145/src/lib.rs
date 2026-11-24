// rust-track: handwritten task-145
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 145;

pub fn top_up_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    (8 - u32::from(value) % 8) % 8
}
