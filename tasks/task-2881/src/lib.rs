#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packed_pin_state_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
    };
    samples
}
