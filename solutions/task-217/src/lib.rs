// rust-track: handwritten task-217
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 217;

pub fn cycle_ticks_2(value: u8) -> u32 {
    let mut pos = 0;
    for _ in 0..value {
        pos = (pos + 2) % 4;
    }
    pos
}
