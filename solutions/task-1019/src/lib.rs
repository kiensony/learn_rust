// rust-track: handwritten task-1019
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1019;

pub fn first_nonempty_field_slash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.split('/').find(|s| !s.is_empty()).unwrap_or_default()
        })
        .len()
    })
}
