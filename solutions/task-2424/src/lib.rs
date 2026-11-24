// rust-track: handwritten task-2424
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2424;

pub fn wide_alignment_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        #[repr(align(8))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
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
