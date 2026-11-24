#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn heading(report: &str) -> &str {
    let copy = report.to_string();
    copy.lines().next().unwrap_or("")
}
