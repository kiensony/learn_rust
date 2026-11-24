// rust-track: handwritten task-454
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 454;

pub fn transfer(accounts: &mut [u16; 2], wanted: u16) -> u16 {
    let moved = wanted.min(accounts[0]).min(u16::MAX - accounts[1]);
    accounts[0] -= moved;
    accounts[1] += moved;
    moved
}
