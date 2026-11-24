// rust-track: handwritten task-849
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 849;

#[derive(Debug, PartialEq, Eq)]
pub struct DoseError {
    pub index: usize,
    pub value: i8,
}

pub fn dose_total(doses: &[i8]) -> Result<u64, DoseError> {
    let mut total = 0;
    for (index, &value) in doses.iter().enumerate() {
        total += u64::try_from(value).map_err(|_| DoseError { index, value })?;
    }
    Ok(total)
}
