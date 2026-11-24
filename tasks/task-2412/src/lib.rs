#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn word_alignment_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        #[repr(align(4))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).div_euclid(alignment) * alignment)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
