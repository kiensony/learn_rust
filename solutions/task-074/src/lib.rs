// rust-track: handwritten task-074
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 74;

pub fn loose_tiles_3(first: u16, second: u16) -> u32 {
    let value = first.saturating_add(second);
    u32::from(value) % 3
}
