// rust-track: handwritten task-583
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 583;

pub fn heading(report: &str) -> &str {
    report.lines().next().unwrap_or("")
}
