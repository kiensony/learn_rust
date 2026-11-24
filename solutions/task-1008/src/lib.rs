// rust-track: handwritten task-1008
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1008;

pub fn prefix_before_final_mark_colon<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    prefix_before_final_mark_colon_impl(records)
}

fn prefix_before_final_mark_colon_impl<T: AsRef<str>, const N: usize>(
    records: [T; N],
) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.rsplit_once(':').map_or("", |(a, _)| a)).len()
    })
}
