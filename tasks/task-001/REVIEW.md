# Review — task-001

## Must

- Proof comment + `TRACK_PROOF == 0x52535401`
- `greet` signature exact
- Trim, empty → stranger
- No `unsafe`, no extra crates
- Tests pass: `cargo test -p task_001`

## Quality

- Prefer `format!` or `push_str`, not manual byte hacks
- No `unwrap` on trim
- No unused helpers

## Reject

- Missing proof
- Comment-novelization
- Extra public functions
