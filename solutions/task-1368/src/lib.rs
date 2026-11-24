// rust-track: handwritten task-1368
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1368;

pub fn frame_offsets_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
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
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
