// rust-track: handwritten task-303
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 303;

pub fn double_a_symbol_bang(text: &str) -> String {
    text.replace("!", "!!")
}
