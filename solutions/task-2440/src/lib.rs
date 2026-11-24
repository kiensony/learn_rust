// rust-track: handwritten task-2440
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2440;

pub fn block_alignment_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        #[repr(align(16))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    samples.into_iter().rev().collect()
}
