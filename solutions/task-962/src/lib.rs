// rust-track: handwritten task-962
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 962;

pub fn legs(stops: &[u8]) -> Vec<(u8, u8)> {
    stops.windows(2).map(|p| (p[0], p[1])).collect()
}
