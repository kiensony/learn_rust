// rust-track: handwritten task-2408
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2408;

pub fn header_alignment_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        #[repr(align(2))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
