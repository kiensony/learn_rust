// rust-track: handwritten task-287
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 287;

pub fn count_separators_tilde(text: &str) -> String {
    count_separators_tilde_impl(text)
}

fn count_separators_tilde_impl(text: &str) -> String {
    text.matches('~').count().to_string()
}
