// rust-track: handwritten task-263
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 263;

pub fn boundary_trim_hash(text: &str) -> String {
    text.trim_matches('#').to_owned()
}
