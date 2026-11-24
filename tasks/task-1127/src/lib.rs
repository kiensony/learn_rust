#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn byte_label(bytes: Vec<u8>) -> String {
    impl std::fmt::Display for Vec<u8> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} bytes", self.len())
        }
    }
    format!("{bytes:?}")
}
