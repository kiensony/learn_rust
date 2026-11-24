// rust-track: handwritten task-952
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 952;

pub fn shortest_field_with_first_tie_hash(
    text: &str,
    mut visit: impl FnMut(&str) -> usize,
) -> usize {
    visit({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split('#').min_by_key(|s| s.len()).unwrap_or_default()
    })
}
