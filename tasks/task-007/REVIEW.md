# Review — task-007

## Must

- Proof + `TRACK_PROOF == 0x52535400 | 7`
- Exact errors `"div by zero"` and `"overflow"`
- `checked_*`, no wrapping as success
- `match` (or equivalent exhaustive)

## Quality

- No `unwrap` on checked ops
- Enum, not stringly ops

## Reject

- Missing proof
- Ignoring i64::MIN / -1 edge if you add Mul later (not required)
- `unsafe` or wrapping_add as the answer
