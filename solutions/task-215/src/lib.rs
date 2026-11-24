// rust-track: handwritten task-215
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 215;

pub fn digit_count_2(value: u8) -> u32 {
    let mut x = u32::from(value);
    let mut count = 1;
    while x >= 2 {
        x /= 2;
        count += 1;
    }
    count
}
