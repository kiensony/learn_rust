# Review for task-051

- Run the public examples and the full test suite. Check the learner's editable files against the behavior in PROBLEM.md.
- Explain why braces inside a user argument remain literal while the braces in the source format string must balance.
- Use safe Rust and the provided standard library. Keep the acceptance tests intact.
- Have the learner show the original symptom, point at the faulty statement and rerun the regression case after the repair.

## Submission step

After the public examples pass, open `src/lib.rs`. Add the following first line and replace the placeholder constant with the second line below. These identify this task during course review.

```rust
// rust-track: handwritten task-051
pub const TRACK_PROOF: u32 = 0x52540000 | 51;
```
