// rust-track: handwritten task-248
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 248;

pub fn countdown_checksum_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    let mut x = u32::from(value);
    let mut sum = 0;
    while x > 0 {
        sum += x;
        x = x.saturating_sub(8);
    }
    sum
}
