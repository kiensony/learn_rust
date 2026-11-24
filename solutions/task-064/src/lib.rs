// rust-track: handwritten task-064
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 64;

pub fn loose_tiles_2(value: u16) -> u32 {
    u32::from(value) % 2
}
