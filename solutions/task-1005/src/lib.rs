// rust-track: handwritten task-1005
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1005;

pub fn closing_marks_removed_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N] {
    records.map(|record| {
        let text = record.as_ref();
        (text.trim_end_matches(':')).len()
    })
}
