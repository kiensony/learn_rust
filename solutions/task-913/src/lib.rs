// rust-track: handwritten task-913
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 913;

pub fn visit_total(values: &[u8], mut visit: impl FnMut(u8) -> u32) -> u64 {
    values.iter().map(|&v| u64::from(visit(v))).sum()
}
