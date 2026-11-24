// rust-track: handwritten task-859
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 859;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn prefix_before_final_mark_colon(text: &str) -> usize {
    prefix_before_final_mark_colon_impl(text)
}

fn prefix_before_final_mark_colon_impl(text: &str) -> usize {
    wire::HEADER_BYTES + (text.rsplit_once(':').map_or("", |(a, _)| a)).len()
}
