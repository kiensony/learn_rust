// rust-track: handwritten task-087
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 87;

pub fn after_delivery(start: i16, delivered: i16) -> i32 {
    let mut total = i32::from(start);
    total += i32::from(delivered);
    total
}
