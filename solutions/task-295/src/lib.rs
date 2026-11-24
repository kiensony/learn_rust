// rust-track: handwritten task-295
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 295;

pub fn last_field_plus(text: &str) -> String {
    text.rsplit('+').next().unwrap_or_default().to_owned()
}
