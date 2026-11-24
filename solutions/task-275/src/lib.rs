// rust-track: handwritten task-275
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 275;

pub fn last_field_star(text: &str) -> String {
    last_field_star_impl(text)
}

fn last_field_star_impl(text: &str) -> String {
    text.rsplit('*').next().unwrap_or_default().to_owned()
}
