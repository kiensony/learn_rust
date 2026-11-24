// rust-track: handwritten task-272
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 272;

pub fn boundary_trim_star(text: &str) -> String {
    text.trim_matches('*').to_owned()
}
