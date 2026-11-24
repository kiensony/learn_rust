// rust-track: handwritten task-2409
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2409;

pub fn header_alignment_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        #[repr(align(2))]
        struct Slot;
        let alignment = std::mem::align_of::<Slot>() as u32;
        input
            .iter()
            .map(|&x| u32::from(x).next_multiple_of(alignment))
            .collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
