#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn decode_batch(input: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
    let mut cursor = 0;
    let mut frames = vec![];
    while cursor < input.len() {
        let n = usize::from(input[cursor]);
        let Some(frame) = input.get(cursor + 1..cursor + 1 + n) else {
            break;
        };
        frames.push(frame.to_vec());
        cursor += 1 + n;
    }
    (frames, vec![])
}
