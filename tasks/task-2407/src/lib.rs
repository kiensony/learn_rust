#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn header_alignment_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            #[repr(align(2))]
            struct Slot;
            let alignment = std::mem::align_of::<Slot>() as u32;
            input
                .iter()
                .map(|&x| u32::from(x).next_multiple_of(alignment))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
