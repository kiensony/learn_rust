// rust-track: handwritten task-2433
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2433;

pub fn block_alignment_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        #[repr(align(16))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
