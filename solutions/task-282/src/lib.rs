// rust-track: handwritten task-282
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 282;

pub fn boundary_trim_tilde(text: &str) -> String {
    text.trim_matches('~').to_owned()
}
