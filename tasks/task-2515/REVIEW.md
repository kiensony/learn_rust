# Review — Decode through a C ABI boundary

- Demonstrate the two public examples and the edge cases for `scaled_reading`.
- Identify the first incorrect step and show a regression that would have caught it.
- Apply the module constraints in PROBLEM.md. Preserve the supplied tests and API.

## Submission

Make `// rust-track: handwritten task-2515` the first line of `src/lib.rs` and replace the starter’s zero marker with `pub const TRACK_PROOF: u32 = 0x52540000 | 2515;`.

Run `cargo track test task-2515` for the full suite.
