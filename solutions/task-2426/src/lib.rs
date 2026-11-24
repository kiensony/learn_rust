// rust-track: handwritten task-2426
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2426;

pub fn wide_alignment_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        #[repr(align(8))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
