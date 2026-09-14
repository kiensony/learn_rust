# Review — task-003

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 3`
- Exact error strings
- `Result`, not panic
- No `unwrap` on parse

## Quality

- `split_once` or equivalent; do not ignore extra colons
- Map parse errors to the specified string

## Reject

- Missing proof
- `panic!` / `unwrap` on bad input
- Wrong error text (common AI miss)
