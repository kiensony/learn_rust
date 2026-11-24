// rust-track: handwritten task-255
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 255;

pub fn digit_count_10(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    let mut x = u32::from(value);
    let mut count = 1;
    while x >= 10 {
        x /= 10;
        count += 1;
    }
    count
}
