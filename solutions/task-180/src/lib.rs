// rust-track: handwritten task-180
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 180;

pub fn admission_gate_5(issued: u16, returned: u16) -> bool {
    let value = issued.saturating_sub(returned);
    value >= 5
}
