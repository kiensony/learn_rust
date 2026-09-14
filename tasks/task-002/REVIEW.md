# Review — task-002

## Must

- Proof: first line + `TRACK_PROOF == 0x52535400 | 2`
- Correct order: 15 before 3 or 5
- Negative and zero handled
- No `unsafe`

## Quality

- One function, no magic string builders
- Remainder (`%`), not string conversion of n

## Reject

- Missing proof
- `if` soup that misses `0` / negatives
- Extra public API
