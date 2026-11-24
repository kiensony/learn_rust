// rust-track: handwritten task-261
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 261;

pub fn double_tickets(packs: u16) -> u32 {
    fn tickets_per_pack() -> u32 {
        2
    }
    u32::from(packs) * tickets_per_pack()
}
