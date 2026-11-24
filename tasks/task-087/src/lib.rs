#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn after_delivery(start: i16, delivered: i16) -> i32 {
    let total = i32::from(start);
    total += i32::from(delivered);
    total
}
