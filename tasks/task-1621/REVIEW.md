# Review — Reproduce a flaky first winner

- Demonstrate the two public examples and the edge cases for `fastest`.
- Identify the first incorrect step and show a regression that would have caught it.
- Apply the module constraints in PROBLEM.md. Preserve the supplied tests and API.

## Submission

Make `// rust-track: handwritten task-1621` the first line of `src/lib.rs` and replace the starter’s zero marker with `pub const TRACK_PROOF: u32 = 0x52540000 | 1621;`.

Run `cargo track test task-1621` for the full suite.
