// rust-track: handwritten task-214
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 214;

pub fn digit_sum_2(value: u8) -> u32 {
    let mut x = u32::from(value);
    let mut total = 0;
    while x > 0 {
        total += x % 2;
        x /= 2;
    }
    total
}
