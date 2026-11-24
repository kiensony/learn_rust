#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn zip_array<A, B, const N: usize>(left: [A; N], right: [B; N]) -> [(A, B); N] {
    let mut pairs = left.into_iter().zip(right.into_iter().rev());
    std::array::from_fn(|_| pairs.next().unwrap())
}
