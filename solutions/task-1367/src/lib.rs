// rust-track: handwritten task-1367
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1367;

pub fn frame_offsets_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_frame_offsets(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_frame_offsets(input: &[u8]) -> Vec<u32> {
    let mut out = vec![];
    let mut i = 0;
    while let Some(&n) = input.get(i) {
        let n = n as usize;
        let Some(payload) = input.get(i + 1..i + 1 + n) else {
            break;
        };
        let _ = payload;
        out.push(i as u32);
        i += 1 + n;
    }
    out
}
