// rust-track: handwritten task-174
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 174;

pub fn even_batch_3(first: u16, second: u16) -> bool {
    let value = first.saturating_add(second);
    value >= 3 && value % 2 == 0
}
