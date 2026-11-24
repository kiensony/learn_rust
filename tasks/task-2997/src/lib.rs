#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn reconcile(events: &[(u16, u16)]) -> Result<u64, u16> {
    let mut entries = std::collections::BTreeMap::new();
    for &(id, amount) in events {
        entries.entry(id).or_insert(amount);
    }
    Ok(entries.values().copied().map(u64::from).sum())
}
