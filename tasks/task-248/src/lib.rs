#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn countdown_checksum_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    let mut x = u32::from(value.saturating_sub(1));
    let mut sum = 0;
    while x > 0 {
        sum += x;
        x = x.saturating_sub(8);
    }
    sum
}
