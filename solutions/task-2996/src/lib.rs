// rust-track: handwritten task-2996
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2996;

pub fn decode_batch(input: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
    let mut cursor = 0;
    let mut frames = vec![];
    while let Some(&n) = input.get(cursor) {
        let end = cursor + 1 + usize::from(n);
        let Some(frame) = input.get(cursor + 1..end) else {
            break;
        };
        frames.push(frame.to_vec());
        cursor = end;
    }
    (frames, input[cursor..].to_vec())
}
