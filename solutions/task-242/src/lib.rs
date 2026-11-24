// rust-track: handwritten task-242
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 242;

pub fn skip_reserved_seats_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    (1..=u32::from(value)).filter(|i| i % 8 != 0).count() as u32
}
