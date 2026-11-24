// rust-track: handwritten task-136
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 136;

pub fn saturating_debit_5(issued: u16, returned: u16) -> u32 {
    let value = issued.saturating_sub(returned);
    u32::from(value.saturating_sub(5))
}
