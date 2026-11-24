// rust-track: handwritten task-091
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 91;

pub fn count_packs_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    u32::from(value) * 8
}
