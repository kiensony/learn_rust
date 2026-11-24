// rust-track: handwritten task-274
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 274;

pub fn read_a_heading_star(text: &str) -> String {
    text.split('*').next().unwrap_or_default().to_owned()
}
