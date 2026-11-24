// rust-track: handwritten task-258
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 258;

pub fn countdown_checksum_10(requested: u8, available: u8) -> u32 {
    let value = requested.min(available);
    let mut x = u32::from(value);
    let mut sum = 0;
    while x > 0 {
        sum += x;
        x = x.saturating_sub(10);
    }
    sum
}
