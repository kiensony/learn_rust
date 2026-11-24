// rust-track: handwritten task-2417
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2417;

pub fn word_alignment_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_word_alignment(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_word_alignment(input: &[u8]) -> Vec<u32> {
    #[repr(align(4))]
    struct Slot;
    let alignment = std::mem::align_of::<Slot>() as u32;
    input
        .iter()
        .map(|&x| u32::from(x).next_multiple_of(alignment))
        .collect()
}
