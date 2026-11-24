#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 267;

pub fn badge_bytes(name: &str) -> usize {
    if name.is_empty() {
        return;
    }
    name.len()
}
