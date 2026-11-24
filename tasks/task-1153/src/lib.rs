#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1153;

pub fn prefix(text: &str) -> &str {
    let scratch = text.to_owned();
    scratch.split(':').next().unwrap_or("")
}
