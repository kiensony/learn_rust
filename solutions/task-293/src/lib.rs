// rust-track: handwritten task-293
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 293;

pub fn double_a_symbol_plus(text: &str) -> String {
    text.replace("+", "++")
}
