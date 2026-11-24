// rust-track: handwritten task-2998
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2998;

pub fn read_versioned(input: &[u8]) -> Result<u16, &'static str> {
    let [version, a, b] = input else {
        return Err("length");
    };
    match version {
        1 => Ok(u16::from_le_bytes([*a, *b])),
        2 => Ok(u16::from_be_bytes([*a, *b])),
        _ => Err("version"),
    }
}
