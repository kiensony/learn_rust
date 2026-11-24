// rust-track: handwritten task-1017
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1017;

pub fn remainder_after_first_mark_slash<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.split_once('/').map_or("", |(_, b)| b)
        })
        .len()
    })
}
