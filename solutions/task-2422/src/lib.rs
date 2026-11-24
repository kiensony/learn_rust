// rust-track: handwritten task-2422
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2422;

pub fn wide_alignment_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        #[repr(align(8))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
