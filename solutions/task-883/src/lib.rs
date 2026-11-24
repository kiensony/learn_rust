// rust-track: handwritten task-883
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 883;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn leading_field_equals(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.split('=').next().unwrap_or_default()
        })
        .len()
}
