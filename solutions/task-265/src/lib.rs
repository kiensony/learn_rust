// rust-track: handwritten task-265
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 265;

pub fn last_field_hash(text: &str) -> String {
    last_field_hash_impl(text)
}

fn last_field_hash_impl(text: &str) -> String {
    text.rsplit('#').next().unwrap_or_default().to_owned()
}
