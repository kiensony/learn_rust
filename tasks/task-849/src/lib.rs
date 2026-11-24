#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct DoseError {
    pub index: usize,
    pub value: i8,
}

pub fn dose_total(doses: &[i8]) -> Result<u64, DoseError> {
    Ok(0)
}
