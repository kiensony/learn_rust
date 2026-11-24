# Review — Measure Unicode text · prefix before final mark (/)

- Demonstrate the two public examples and the edge cases for `prefix_before_final_mark_slash`.
- Show the private helper, explain the duplicated or partial behavior it replaced, and check callers.
- Apply the module constraints in PROBLEM.md. Preserve the supplied tests and API.

## Submission

Make `// rust-track: handwritten task-524` the first line of `src/lib.rs` and replace the starter’s zero marker with `pub const TRACK_PROOF: u32 = 0x52540000 | 524;`.

Run `cargo track test task-524` for the full suite.
