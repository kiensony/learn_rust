// rust-track: handwritten task-1035
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1035;

pub fn closing_marks_removed_equals<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    closing_marks_removed_equals_impl(records)
}

fn closing_marks_removed_equals_impl<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.split_once(';').map_or(text, |(data, _)| data);
            text.trim_end_matches('=')
        })
        .len()
    })
}
