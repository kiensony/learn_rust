// rust-track: handwritten task-279
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 279;

pub fn wrap_a_message_tilde(text: &str) -> String {
    format!("~{text}~")
}
