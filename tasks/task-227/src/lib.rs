#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cycle_ticks_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    let value = value.saturating_sub(1);
    let mut pos = 0;
    for _ in 0..value {
        pos = (pos + 3) % 5;
    }
    pos
}
