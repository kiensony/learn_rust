// rust-track: handwritten task-1034
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1034;

pub fn leading_marks_removed_equals<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.trim_start_matches('=')
        })
        .len()
    })
}
