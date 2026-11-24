// rust-track: handwritten task-2457

pub const TRACK_PROOF: u32 = 0x52540000 | 2457;

pub fn checked_copy(bytes: &[u8], offset: usize, count: usize) -> Option<Vec<u8>> {
    let range = checked_range(bytes.len(), offset, count)?;
    // SAFETY: the validated range lies within this live, initialized byte slice.
    // A zero-length range may begin at the one-past pointer.
    let sub = unsafe { std::slice::from_raw_parts(bytes.as_ptr().add(range.start), range.len()) };
    Some(sub.to_vec())
}

fn checked_range(len: usize, offset: usize, count: usize) -> Option<std::ops::Range<usize>> {
    let end = offset.checked_add(count)?;
    (end <= len).then_some(offset..end)
}
