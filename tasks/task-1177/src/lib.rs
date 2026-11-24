#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn stored_key<'a>(keys: &'a [&'a str], query: &str) -> Option<&'a str> {
    if keys.contains(&query) {
        Some(query)
    } else {
        None
    }
}
