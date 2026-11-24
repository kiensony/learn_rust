// rust-track: handwritten task-247
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 247;

pub fn cycle_ticks_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    let mut pos = 0;
    for _ in 0..value {
        pos = (pos + 8) % 10;
    }
    pos
}
