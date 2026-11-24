// rust-track: handwritten task-222
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 222;

pub fn skip_reserved_seats_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    (1..=u32::from(value)).filter(|i| i % 3 != 0).count() as u32
}
