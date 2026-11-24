#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn frame_offsets_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
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
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
