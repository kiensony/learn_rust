// rust-track: handwritten task-1234
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1234;

pub fn seek_offsets_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_seek_offsets(input);
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}

fn observe_seek_offsets(input: &[u8]) -> Vec<u32> {
    use std::io::{Cursor, Read, Seek, SeekFrom};
    let mut file = Cursor::new(input);
    let mut data = vec![];
    for i in (1..input.len()).step_by(2) {
        file.seek(SeekFrom::Start(i as u64)).unwrap();
        let mut b = [0u8];
        file.read_exact(&mut b).unwrap();
        data.push(u32::from(b[0]));
    }
    data
}
