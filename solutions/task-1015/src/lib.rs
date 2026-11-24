// rust-track: handwritten task-1015
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1015;

pub fn closing_marks_removed_slash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.trim_end_matches('/')
        })
        .len()
    })
}
