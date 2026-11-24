// rust-track: handwritten task-235
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 235;

pub fn digit_count_5(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    let mut x = u32::from(value);
    let mut count = 1;
    while x >= 5 {
        x /= 5;
        count += 1;
    }
    count
}
