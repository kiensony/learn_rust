// rust-track: handwritten task-1032
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1032;

pub fn leading_field_equals<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.split('=').next().unwrap_or_default()
        })
        .len()
    })
}
