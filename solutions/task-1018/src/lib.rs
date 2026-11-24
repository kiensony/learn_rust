// rust-track: handwritten task-1018
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1018;

pub fn prefix_before_final_mark_slash<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    prefix_before_final_mark_slash_impl(records)
}

fn prefix_before_final_mark_slash_impl<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        ({
            let text: &str = &text;
            let text = text.strip_prefix("path/").unwrap_or(text);
            text.rsplit_once('/').map_or("", |(a, _)| a)
        })
        .len()
    })
}
