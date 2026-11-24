# Review for task-060

- Run the public examples and the full test suite. Check the learner's editable files against the behavior in PROBLEM.md.
- Review a regression using distinct values, run it on the original defect and discuss why the identical-value case misses swapped positions.
- Use safe Rust and the provided standard library. Keep the acceptance tests intact.
- Inspect the new regression test and NOTES.md. The test must distinguish swapped values. The explanation must predict what happens when the values match.
- Count this checkpoint toward mastery after the learner demonstrates an independent repair or implementation.

## Submission step

After the public examples pass, open `src/lib.rs`. Add the following first line and replace the placeholder constant with the second line below. These identify this task during course review.

```rust
// rust-track: handwritten task-060
pub const TRACK_PROOF: u32 = 0x52540000 | 60;
```
