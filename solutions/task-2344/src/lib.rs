// rust-track: handwritten task-2344
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2344;

pub fn checksum(input: &[u8]) -> u32 {
    input.iter().copied().fold(0, update)
}

fn update(accumulator: u32, byte: u8) -> u32 {
    accumulator.rotate_left(3) ^ u32::from(byte)
}
