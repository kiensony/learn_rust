# Review — Inspect a borrowed label · leading field (|)

- Demonstrate the two public examples and the edge cases for `leading_field_pipe`.
- Identify the first incorrect step and show a regression that would have caught it.
- Apply the module constraints in PROBLEM.md. Preserve the supplied tests and API.

## Submission

Make `// rust-track: handwritten task-378` the first line of `src/lib.rs` and replace the starter’s zero marker with `pub const TRACK_PROOF: u32 = 0x52540000 | 378;`.

Run `cargo track test task-378` for the full suite.
