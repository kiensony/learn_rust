// rust-track: handwritten task-1127
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1127;

pub fn byte_label(bytes: Vec<u8>) -> String {
    struct Packet(Vec<u8>);
    impl std::fmt::Display for Packet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} bytes", self.0.len())
        }
    }
    Packet(bytes).to_string()
}
