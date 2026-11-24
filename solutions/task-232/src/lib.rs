// rust-track: handwritten task-232
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 232;

pub fn skip_reserved_seats_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    (1..=u32::from(value)).filter(|i| i % 5 != 0).count() as u32
}
