#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn seek_offsets_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let observations: Vec<u32> = {
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
        };
        observations.iter().collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
