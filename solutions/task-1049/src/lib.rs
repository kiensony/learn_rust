// rust-track: handwritten task-1049
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1049;

pub fn first_nonempty_field_hash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text
                .split_once('[')
                .and_then(|(_, tail)| tail.split_once(']'))
                .map_or("", |(inside, _)| inside);
            text.split('#').find(|s| !s.is_empty()).unwrap_or_default()
        })
        .len()
    })
}
