// rust-track: handwritten task-237
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 237;

pub fn cycle_ticks_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    let mut pos = 0;
    for _ in 0..value {
        pos = (pos + 5) % 7;
    }
    pos
}
