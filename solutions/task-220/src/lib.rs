// rust-track: handwritten task-220
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 220;

pub fn before_stop(readings: &[u8]) -> u32 {
    let mut sum = 0;
    for &value in readings {
        if value == 255 {
            break;
        }
        sum += u32::from(value);
    }
    sum
}
