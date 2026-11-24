// rust-track: handwritten task-200
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 200;

pub fn admission_gate_10(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    value >= 10
}
