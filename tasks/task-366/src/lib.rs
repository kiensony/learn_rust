#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 366;

pub fn first_reading(v: &[i16]) -> i16 {
    let n = v.first().copied().unwrap_or(0);
    *n
}
