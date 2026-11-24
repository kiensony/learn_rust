// rust-track: handwritten task-3000
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 3000;

pub fn affected_ticks(incidents: &[(u16, u16)]) -> u32 {
    let mut sorted = incidents.to_vec();
    sorted.sort_unstable();
    let mut end = 0;
    let mut total = 0;
    for (a, b) in sorted {
        if b > a && b > end {
            total += u32::from(b - a.max(end));
            end = b;
        }
    }
    total
}
