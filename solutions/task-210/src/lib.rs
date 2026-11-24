// rust-track: handwritten task-210
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 210;

pub fn knocks(times: u8) -> String {
    let mut sound = String::new();
    for _ in 0..times {
        sound.push('!');
    }
    sound
}
