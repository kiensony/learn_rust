// rust-track: handwritten task-1249
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1249;

pub fn seek_offsets_frequent_readings(input: &[u8], keep: usize) -> Vec<u32> {
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
    let mut counts = std::collections::BTreeMap::new();
    for (index, value) in observations.into_iter().enumerate() {
        let (count, _) = counts.entry(value).or_insert((0usize, index));
        *count += 1;
    }
    let mut entries: Vec<_> = counts.into_iter().collect();
    entries.sort_unstable_by_key(|(_, (count, first))| (std::cmp::Reverse(*count), *first));
    entries
        .into_iter()
        .take(keep)
        .map(|(value, _)| value)
        .collect()
}
