// rust-track: handwritten task-301
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 301;

pub fn remove_a_suffix_bang(text: &str) -> String {
    text.strip_suffix("!").unwrap_or(text).to_owned()
}
