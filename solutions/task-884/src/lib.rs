// rust-track: handwritten task-884
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 884;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn closing_field_equals(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.rsplit('=').next().unwrap_or_default()
        })
        .len()
}
