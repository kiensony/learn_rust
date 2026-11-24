// rust-track: handwritten task-257
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 257;

pub fn cycle_ticks_10(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    let mut pos = 0;
    for _ in 0..value {
        pos = (pos + 10) % 12;
    }
    pos
}
