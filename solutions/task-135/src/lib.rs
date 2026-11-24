// rust-track: handwritten task-135
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 135;

pub fn top_up_5(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    (5 - u32::from(value) % 5) % 5
}
