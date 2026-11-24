// rust-track: handwritten task-211
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 211;

pub fn countdown_total(start: u8) -> u32 {
    let mut value = start;
    let mut sum = 0;
    while value > 0 {
        sum += u32::from(value);
        value -= 1;
    }
    sum
}
