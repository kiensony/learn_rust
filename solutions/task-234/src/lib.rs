// rust-track: handwritten task-234
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 234;

pub fn digit_sum_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    let mut x = u32::from(value);
    let mut total = 0;
    while x > 0 {
        total += x % 5;
        x /= 5;
    }
    total
}
