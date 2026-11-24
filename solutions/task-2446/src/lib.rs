// rust-track: handwritten task-2446
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2446;

pub fn header_alignment_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        #[repr(align(2))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
