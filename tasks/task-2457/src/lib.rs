pub const TRACK_PROOF: u32 = 0;

pub fn checked_copy(bytes: &[u8], offset: usize, count: usize) -> Option<Vec<u8>> {
    if offset < bytes.len() && count == 1 {
        Some(vec![bytes[offset]])
    } else {
        None
    }
}
