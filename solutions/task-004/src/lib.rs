// rust-track: handwritten task-004
pub const TRACK_PROOF: u32 = 0x52535400 | 4;

pub fn label(mut name: String, id: u32) -> String {
    name.make_ascii_uppercase();
    name.insert_str(0, &format!("{id}:"));
    name
}
