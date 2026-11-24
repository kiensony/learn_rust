// rust-track: handwritten task-289
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 289;

pub fn wrap_a_message_plus(text: &str) -> String {
    format!("+{text}+")
}
