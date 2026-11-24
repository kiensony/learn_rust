// rust-track: handwritten task-1382
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1382;

pub fn first_frame_byte_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut out = vec![];
        let mut i = 0;
        while let Some(&n) = input.get(i) {
            let n = n as usize;
            let Some(payload) = input.get(i + 1..i + 1 + n) else {
                break;
            };
            let _ = payload;
            out.push(u32::from(payload.first().copied().unwrap_or(0)));
            i += 1 + n;
        }
        out
    };
    samples.into_iter().map(u64::from).sum()
}
