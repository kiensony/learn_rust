#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn word_alignment_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        #[repr(align(4))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
