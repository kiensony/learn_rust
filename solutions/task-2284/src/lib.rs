// rust-track: handwritten task-2284
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2284;

pub fn find_newlines(input: &[u8]) -> usize {
    input.iter().copied().filter(is_terminator).count()
}

fn is_terminator(byte: &u8) -> bool {
    *byte == b'\n'
}
