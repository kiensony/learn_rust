// rust-track: handwritten task-245
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 245;

pub fn digit_count_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    let mut x = u32::from(value);
    let mut count = 1;
    while x >= 8 {
        x /= 8;
        count += 1;
    }
    count
}
