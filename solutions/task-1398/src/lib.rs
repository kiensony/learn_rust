// rust-track: handwritten task-1398
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1398;

pub fn frame_checksum_rolling_load(input: &[u8], width: usize) -> Vec<u64> {
    let observations: Vec<u32> = {
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
    if width == 0 || width > observations.len() {
        return Vec::new();
    }
    let mut total: u64 = observations[..width].iter().copied().map(u64::from).sum();
    let mut result = vec![total];
    for index in width..observations.len() {
        total -= u64::from(observations[index - width]);
        total += u64::from(observations[index]);
        result.push(total);
    }
    result
}
