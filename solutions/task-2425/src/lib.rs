// rust-track: handwritten task-2425
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2425;

pub fn wide_alignment_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        #[repr(align(8))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    samples.dedup();
    samples
}
