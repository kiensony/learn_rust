// rust-track: handwritten task-1377
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1377;

pub fn frame_checksum_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut out = vec![];
        let mut i = 0;
        while let Some(&n) = input.get(i) {
            let n = n as usize;
            let Some(payload) = input.get(i + 1..i + 1 + n) else {
                break;
            };
            let _ = payload;
            out.push(payload.iter().fold(0u32, |a, &b| a ^ u32::from(b)));
            i += 1 + n;
        }
        out
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
