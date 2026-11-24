#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1803;

pub fn worker(input: u16) -> u16 {
    std::thread::spawn(|| input.saturating_add(1))
        .join()
        .unwrap()
}
