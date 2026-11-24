// rust-track: handwritten task-283
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 283;

pub fn double_a_symbol_tilde(text: &str) -> String {
    text.replace("~", "~~")
}
