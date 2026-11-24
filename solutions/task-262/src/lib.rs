// rust-track: handwritten task-262
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 262;

fn scale(count: u16, grams: u16) -> u32 {
    u32::from(count) * u32::from(grams)
}
pub fn mass(scoops: u16) -> u32 {
    scale(scoops, 12)
}
