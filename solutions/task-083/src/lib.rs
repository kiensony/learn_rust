// rust-track: handwritten task-083
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 83;

pub fn complete_rows_5(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    u32::from(value) / 5
}
