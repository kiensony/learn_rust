// rust-track: handwritten task-902
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 902;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn shortest_field_with_first_tie_hash(text: &str) -> usize {
    wire::HEADER_BYTES
        + ({
            let text: &str = &text;
            let text = text
                .split_once('[')
                .and_then(|(_, tail)| tail.split_once(']'))
                .map_or("", |(inside, _)| inside);
            text.split('#').min_by_key(|s| s.len()).unwrap_or_default()
        })
        .len()
}
