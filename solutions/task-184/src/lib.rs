// rust-track: handwritten task-184
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 184;

pub fn even_batch_5(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    value >= 5 && value % 2 == 0
}
