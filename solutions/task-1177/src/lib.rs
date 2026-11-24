// rust-track: handwritten task-1177
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1177;

pub fn stored_key<'a>(keys: &'a [&'a str], query: &str) -> Option<&'a str> {
    keys.iter().copied().find(|&key| key == query)
}
