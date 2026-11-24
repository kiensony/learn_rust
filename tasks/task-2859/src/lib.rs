#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn little_endian_registers_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .windows(2)
            .map(|c| u32::from(u16::from_le_bytes([c[0], c[1]])))
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
