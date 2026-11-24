// rust-track: handwritten task-2529
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2529;

pub fn abi_signed_decode_longest_run(input: &[u8]) -> u64 {
    let samples = observe_abi_signed_decode(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_abi_signed_decode(input: &[u8]) -> Vec<u32> {
    extern "C" fn transform(x: u8) -> u32 {
        u32::from((x as i8).unsigned_abs())
    }
    let callback: extern "C" fn(u8) -> u32 = transform;
    input.iter().map(|&x| callback(x)).collect()
}
