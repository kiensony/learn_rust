// rust-track: handwritten task-238
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 238;

pub fn countdown_checksum_5(issued: u8, returned: u8) -> u32 {
    countdown_checksum_5_impl(issued, returned)
}

fn countdown_checksum_5_impl(issued: u8, returned: u8) -> u32 {
    let value = issued.saturating_sub(returned);
    let mut x = u32::from(value);
    let mut sum = 0;
    while x > 0 {
        sum += x;
        x = x.saturating_sub(5);
    }
    sum
}
