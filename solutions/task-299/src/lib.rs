// rust-track: handwritten task-299
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 299;

pub fn wrap_a_message_bang(text: &str) -> String {
    format!("!{text}!")
}
