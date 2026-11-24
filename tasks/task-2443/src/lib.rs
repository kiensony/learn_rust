#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn page_alignment_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        #[repr(align(32))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).div_euclid(alignment) * alignment)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
