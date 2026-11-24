// rust-track: handwritten task-1621
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1621;

pub fn fastest(reports: &[(u16, u8)]) -> Option<u8> {
    reports.iter().min().map(|r| r.1)
}
