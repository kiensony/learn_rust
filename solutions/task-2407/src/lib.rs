// rust-track: handwritten task-2407
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2407;

pub fn header_alignment_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_header_alignment(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_header_alignment(input: &[u8]) -> Vec<u32> {
    #[repr(align(2))]
    struct Slot;
    let alignment = std::mem::align_of::<Slot>() as u32;
    input
        .iter()
        .map(|&x| u32::from(x).next_multiple_of(alignment))
        .collect()
}
