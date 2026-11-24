// rust-track: handwritten task-070
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 70;

pub fn ticket_text_2(value: u16) -> String {
    format!("B-2/{value}")
}
