# Review — task-010

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 10`
- Exact `RowError` variants
- Score range `0..=100`
- `passing` sorted names
- No `unsafe`, no extra crates

## Quality

- Iterator adapters on `passing`
- No `unwrap` on parse of user lines
- Private helpers ok if not public API

## Reject

- Missing proof
- `panic` on bad rows
- Unsorted output
- Comment-novelization / stock rustdoc dump
- `unsafe` to "be faster"
