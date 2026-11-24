// rust-track: handwritten task-254
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 254;

pub fn digit_sum_10(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    let mut x = u32::from(value);
    let mut total = 0;
    while x > 0 {
        total += x % 10;
        x /= 10;
    }
    total
}
