// rust-track: handwritten task-225
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 225;

pub fn digit_count_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    let mut x = u32::from(value);
    let mut count = 1;
    while x >= 3 {
        x /= 3;
        count += 1;
    }
    count
}
