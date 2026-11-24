// rust-track: handwritten task-250
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 250;

pub fn count_chimes_10(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    (1..=u32::from(value)).filter(|i| i % 10 == 0).count() as u32
}
