// rust-track: handwritten task-230
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 230;

pub fn count_chimes_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    (1..=u32::from(value)).filter(|i| i % 5 == 0).count() as u32
}
