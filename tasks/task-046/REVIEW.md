# Review for task-046

- Run the public examples and the full test suite. Check the learner's editable files against the behavior in PROBLEM.md.
- Try two user arguments and identify the executable name at argument slot zero. Only user argument one is displayed.
- Use safe Rust and the provided standard library. Keep the acceptance tests intact.
- Have the learner show the original symptom, point at the faulty statement and rerun the regression case after the repair.

## Submission step

After the public examples pass, open `src/lib.rs`. Add the following first line and replace the placeholder constant with the second line below. These identify this task during course review.

```rust
// rust-track: handwritten task-046
pub const TRACK_PROOF: u32 = 0x52540000 | 46;
```
