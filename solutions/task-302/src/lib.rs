// rust-track: handwritten task-302
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 302;

pub fn boundary_trim_bang(text: &str) -> String {
    boundary_trim_bang_impl(text)
}

fn boundary_trim_bang_impl(text: &str) -> String {
    text.trim_matches('!').to_owned()
}
