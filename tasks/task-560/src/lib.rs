#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 560;

pub fn title(primary: &str, fallback: &str) -> &str {
    if primary.is_empty() {
        fallback
    } else {
        primary
    }
}
