#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn digit_sum_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    let mut x = u32::from(value.saturating_sub(1));
    let mut total = 0;
    while x > 0 {
        total += x % 3;
        x /= 3;
    }
    total
}
