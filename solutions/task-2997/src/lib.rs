// rust-track: handwritten task-2997
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2997;

pub fn reconcile(events: &[(u16, u16)]) -> Result<u64, u16> {
    let mut entries = std::collections::BTreeMap::new();
    for &(id, amount) in events {
        if entries.insert(id, amount).is_some_and(|old| old != amount) {
            return Err(id);
        }
    }
    Ok(entries.values().copied().map(u64::from).sum())
}
