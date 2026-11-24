// rust-track: handwritten task-244
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 244;

pub fn digit_sum_8(start: u8, end: u8) -> u32 {
    let value = start.abs_diff(end);
    let mut x = u32::from(value);
    let mut total = 0;
    while x > 0 {
        total += x % 8;
        x /= 8;
    }
    total
}
