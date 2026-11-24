#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn reverse_a_reading_8(start: u16, end: u16) -> u32 {
    let value = start.abs_diff(end);
    (65535 - u32::from(value.saturating_sub(1))) / 8
}
