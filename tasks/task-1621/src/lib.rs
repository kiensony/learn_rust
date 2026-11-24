#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn fastest(reports: &[(u16, u8)]) -> Option<u8> {
    reports.iter().min_by_key(|r| r.0).map(|r| r.1)
}
