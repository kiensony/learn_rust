// rust-track: handwritten task-292
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 292;

pub fn boundary_trim_plus(text: &str) -> String {
    boundary_trim_plus_impl(text)
}

fn boundary_trim_plus_impl(text: &str) -> String {
    text.trim_matches('+').to_owned()
}
