// rust-track: handwritten task-1048
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1048;

pub fn zip_array<A, B, const N: usize>(left: [A; N], right: [B; N]) -> [(A, B); N] {
    let mut pairs = left.into_iter().zip(right);
    std::array::from_fn(|_| pairs.next().unwrap())
}
