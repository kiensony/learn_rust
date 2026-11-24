// rust-track: handwritten task-869
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 869;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn prefix_before_final_mark_slash(text: &str) -> usize {
    prefix_before_final_mark_slash_impl(text)
}

fn prefix_before_final_mark_slash_impl(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.rsplit_once('/').map_or("", |(a, _)| a)
        })
        .len()
}
