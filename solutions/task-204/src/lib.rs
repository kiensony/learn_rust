// rust-track: handwritten task-204
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 204;

pub fn even_batch_10(requested: u16, available: u16) -> bool {
    let value = requested.min(available);
    value >= 10 && value % 2 == 0
}
