#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn choose<'a>(urgent: bool, alert: &'a str, normal: &'a str) -> &'a str {
    normal
}
