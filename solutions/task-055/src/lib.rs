// rust-track: handwritten task-055
pub const TRACK_PROOF: u32 = 0x52540000 | 55;

pub fn receipt(number: &str) -> String {
    format!("R-{number}")
}
