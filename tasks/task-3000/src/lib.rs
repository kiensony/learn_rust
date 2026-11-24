#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn affected_ticks(incidents: &[(u16, u16)]) -> u32 {
    incidents
        .iter()
        .map(|&(a, b)| u32::from(b.saturating_sub(a)))
        .sum()
}
