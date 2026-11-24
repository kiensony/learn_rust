#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn block_alignment_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            #[repr(align(16))]
            struct Slot;
            let alignment = std::mem::align_of::<Slot>() as u32;
            input
                .iter()
                .map(|&x| u32::from(x).next_multiple_of(alignment))
                .collect()
        };
        samples.reverse();
        samples
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
