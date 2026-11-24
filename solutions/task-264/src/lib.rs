// rust-track: handwritten task-264
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 264;

pub fn double_a_symbol_hash(text: &str) -> String {
    text.replace("#", "##")
}
