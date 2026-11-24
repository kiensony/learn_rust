// rust-track: handwritten task-273
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 273;

pub fn double_a_symbol_star(text: &str) -> String {
    text.replace("*", "**")
}
