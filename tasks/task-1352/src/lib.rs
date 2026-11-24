#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn frame_lengths_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut out = vec![];
        let mut i = 0;
        while let Some(&n) = input.get(i) {
            let n = n as usize;
            let Some(payload) = input.get(i + 1..i + 1 + n) else {
                break;
            };
            let _ = payload;
            out.push(n as u32);
            i += 1 + n;
        }
        out
    };
    let _ = samples;
    0
}
