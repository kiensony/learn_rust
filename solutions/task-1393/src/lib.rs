// rust-track: handwritten task-1393
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1393;

pub fn frame_maxima_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut out = vec![];
        let mut i = 0;
        while let Some(&n) = input.get(i) {
            let n = n as usize;
            let Some(payload) = input.get(i + 1..i + 1 + n) else {
                break;
            };
            let _ = payload;
            out.push(u32::from(payload.iter().copied().max().unwrap_or(0)));
            i += 1 + n;
        }
        out
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
