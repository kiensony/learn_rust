// rust-track: handwritten task-228
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 228;

pub fn countdown_checksum_3(first: u8, second: u8) -> u32 {
    let value = first.saturating_add(second);
    let mut x = u32::from(value);
    let mut sum = 0;
    while x > 0 {
        sum += x;
        x = x.saturating_sub(3);
    }
    sum
}
