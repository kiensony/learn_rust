// rust-track: handwritten task-205
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 205;

pub fn service_interval_10(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    value > 0 && value % 10 == 0
}
